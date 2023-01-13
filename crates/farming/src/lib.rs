//! # Farming Module
//! Root can create reward schedules which payout incentives
//! on a per period basis. Users can stake LP tokens, such as
//! those generated from an AMM or lending protocol to receive
//! these rewards by claiming.

// #![deny(warnings)]
#![cfg_attr(test, feature(proc_macro_hygiene))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod default_weights;
pub use default_weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{dispatch::DispatchResult, traits::Get, transactional, weights::Weight, PalletId, RuntimeDebug};
use orml_traits::{MultiCurrency, MultiReservableCurrency};
use primitives::CurrencyId;
use reward::RewardsApi;
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{AccountIdConversion, AtLeast32Bit, CheckedDiv, Saturating, Zero},
    ArithmeticError, DispatchError, TransactionOutcome,
};
use sp_std::vec::Vec;

pub use pallet::*;

#[derive(Clone, Default, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct RewardSchedule<Balance: MaxEncodedLen> {
    /// Number of periods remaining
    pub period_count: u32,
    /// Amount of tokens to release
    #[codec(compact)]
    pub per_period: Balance,
}

impl<Balance: AtLeast32Bit + MaxEncodedLen + Copy> RewardSchedule<Balance> {
    /// Returns total amount to distribute, `None` if calculation overflows
    pub fn total(&self) -> Option<Balance> {
        self.per_period.checked_mul(&self.period_count.into())
    }

    /// Take the next reward and decrement the period count
    pub fn take(&mut self) -> Option<Balance> {
        if self.period_count.gt(&0) {
            self.period_count.saturating_dec();
            Some(self.per_period)
        } else {
            None
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::{ensure_root, ensure_signed, pallet_prelude::*};

    pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

    pub(crate) type CurrencyIdOf<T> =
        <<T as Config>::MultiCurrency as MultiCurrency<<T as frame_system::Config>::AccountId>>::CurrencyId;

    pub(crate) type BalanceOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<AccountIdOf<T>>>::Balance;

    pub(crate) type RewardScheduleOf<T> = RewardSchedule<BalanceOf<T>>;

    /// ## Configuration
    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The farming pallet id, used for deriving pool accounts.
        #[pallet::constant]
        type FarmingPalletId: Get<PalletId>;

        /// The treasury pallet id, used for deriving its sovereign account ID.
        #[pallet::constant]
        type TreasuryPalletId: Get<PalletId>;

        /// The period to accrue rewards.
        #[pallet::constant]
        type RewardPeriod: Get<Self::BlockNumber>;

        /// Reward pools to track stake.
        type RewardPools: RewardsApi<
            CurrencyIdOf<Self>, // pool id is the lp token
            AccountIdOf<Self>,
            BalanceOf<Self>,
            CurrencyId = CurrencyIdOf<Self>,
        >;

        /// Currency handler to transfer tokens.
        type MultiCurrency: MultiReservableCurrency<AccountIdOf<Self>, CurrencyId = CurrencyId>;

        /// Weight information for the extrinsics.
        type WeightInfo: WeightInfo;
    }

    // The pallet's events
    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        RewardScheduleUpdated {
            pool_currency_id: CurrencyIdOf<T>,
            reward_currency_id: CurrencyIdOf<T>,
            period_count: u32,
            per_period: BalanceOf<T>,
        },
        RewardDistributed {
            pool_currency_id: CurrencyIdOf<T>,
            reward_currency_id: CurrencyIdOf<T>,
            amount: BalanceOf<T>,
        },
        RewardClaimed {
            account_id: AccountIdOf<T>,
            pool_currency_id: CurrencyIdOf<T>,
            reward_currency_id: CurrencyIdOf<T>,
            amount: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        ScheduleNotFound,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
        fn on_initialize(now: T::BlockNumber) -> Weight {
            if now % T::RewardPeriod::get() == Zero::zero() {
                let mut count: u32 = 0;
                // collect first to avoid modifying in-place
                let schedules = RewardSchedules::<T>::iter().collect::<Vec<_>>();
                for (pool_currency_id, reward_currency_id, mut reward_schedule) in schedules.into_iter() {
                    if let Some(amount) = reward_schedule.take() {
                        if let Ok(_) = Self::try_distribute_reward(pool_currency_id, reward_currency_id, amount) {
                            // only update the schedule if we could distribute the reward
                            RewardSchedules::<T>::insert(pool_currency_id, reward_currency_id, reward_schedule);
                            count.saturating_inc();
                            Self::deposit_event(Event::RewardDistributed {
                                pool_currency_id,
                                reward_currency_id,
                                amount,
                            });
                        }
                    } else {
                        // period count is zero
                        RewardSchedules::<T>::remove(pool_currency_id, reward_currency_id);
                        // TODO: sweep leftover rewards?
                    }
                }
                T::WeightInfo::on_initialize(count)
            } else {
                Weight::zero()
            }
        }
    }

    #[pallet::storage]
    #[pallet::getter(fn reward_schedules)]
    pub type RewardSchedules<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        CurrencyIdOf<T>, // lp token
        Blake2_128Concat,
        CurrencyIdOf<T>, // reward currency
        RewardScheduleOf<T>,
        ValueQuery,
    >;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create or overwrite the reward schedule, if a reward schedule
        /// already exists for the rewards currency it will first distribute
        /// any remaining tokens to the rewards pool
        #[pallet::weight(T::WeightInfo::update_reward_schedule())]
        #[transactional]
        pub fn update_reward_schedule(
            origin: OriginFor<T>,
            pool_currency_id: CurrencyIdOf<T>,
            reward_currency_id: CurrencyIdOf<T>,
            period_count: u32,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // fund the pool account from treasury
            let treasury_account_id = Self::treasury_account_id();
            let pool_account_id = Self::pool_account_id(&pool_currency_id);
            T::MultiCurrency::transfer(reward_currency_id, &treasury_account_id, &pool_account_id, amount)?;

            RewardSchedules::<T>::try_mutate(pool_currency_id, reward_currency_id, |reward_schedule| {
                let total_period_count = reward_schedule
                    .period_count
                    .checked_add(period_count)
                    .ok_or(ArithmeticError::Overflow)?;
                let total_free = T::MultiCurrency::free_balance(reward_currency_id, &pool_account_id);
                let total_per_period = total_free.checked_div(&total_period_count.into()).unwrap_or_default();

                reward_schedule.period_count = total_period_count;
                reward_schedule.per_period = total_per_period;

                Self::deposit_event(Event::RewardScheduleUpdated {
                    pool_currency_id,
                    reward_currency_id,
                    period_count: total_period_count,
                    per_period: total_per_period,
                });
                Ok(().into())
            })
        }

        /// Explicitly remove a reward schedule and transfer any remaining
        /// balance to the treasury
        #[pallet::weight(T::WeightInfo::remove_reward_schedule())]
        #[transactional]
        pub fn remove_reward_schedule(
            origin: OriginFor<T>,
            pool_currency_id: CurrencyIdOf<T>,
            reward_currency_id: CurrencyIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            // transfer unspent rewards to treasury
            let treasury_account_id = Self::treasury_account_id();
            let pool_account_id = Self::pool_account_id(&pool_currency_id);
            T::MultiCurrency::transfer(
                reward_currency_id,
                &pool_account_id,
                &treasury_account_id,
                T::MultiCurrency::free_balance(reward_currency_id, &pool_account_id),
            )?;

            RewardSchedules::<T>::remove(pool_currency_id, reward_currency_id);
            Self::deposit_event(Event::RewardScheduleUpdated {
                pool_currency_id,
                reward_currency_id,
                period_count: Zero::zero(),
                per_period: Zero::zero(),
            });

            Ok(().into())
        }

        /// Stake the pool tokens in the reward pool
        #[pallet::weight(T::WeightInfo::deposit())]
        #[transactional]
        pub fn deposit(
            origin: OriginFor<T>,
            pool_currency_id: CurrencyIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // prevent depositing without reward schedule
            ensure!(
                !RewardSchedules::<T>::iter_prefix_values(pool_currency_id)
                    .count()
                    .is_zero(),
                Error::<T>::ScheduleNotFound
            );

            // reserve lp tokens to prevent spending
            T::MultiCurrency::reserve(pool_currency_id.clone(), &who, amount)?;

            // deposit lp tokens as stake
            T::RewardPools::deposit_stake(&pool_currency_id, &who, amount)
        }

        /// Unstake the pool tokens from the reward pool
        #[pallet::weight(T::WeightInfo::withdraw())]
        #[transactional]
        pub fn withdraw(
            origin: OriginFor<T>,
            pool_currency_id: CurrencyIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // unreserve lp tokens to allow spending
            let _remaining = T::MultiCurrency::unreserve(pool_currency_id.clone(), &who, amount);
            // TODO: check remaining is non-zeo

            // withdraw lp tokens from stake
            T::RewardPools::withdraw_stake(&pool_currency_id, &who, amount)
        }

        /// Withdraw any accrued rewards from the reward pool
        #[pallet::weight(T::WeightInfo::claim())]
        #[transactional]
        pub fn claim(
            origin: OriginFor<T>,
            pool_currency_id: CurrencyIdOf<T>,
            reward_currency_id: CurrencyIdOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let pool_account_id = Self::pool_account_id(&pool_currency_id);

            // get reward from staking pool
            let reward = T::RewardPools::withdraw_reward(&pool_currency_id, &who, reward_currency_id)?;
            // transfer from pool to user
            T::MultiCurrency::transfer(reward_currency_id, &pool_account_id, &who, reward)?;

            Self::deposit_event(Event::RewardClaimed {
                account_id: who,
                pool_currency_id,
                reward_currency_id,
                amount: reward,
            });

            Ok(())
        }
    }
}

// "Internal" functions, callable by code.
impl<T: Config> Pallet<T> {
    pub fn pool_account_id(pool_currency_id: &CurrencyIdOf<T>) -> T::AccountId {
        T::FarmingPalletId::get().into_sub_account_truncating(pool_currency_id)
    }

    pub fn treasury_account_id() -> T::AccountId {
        T::TreasuryPalletId::get().into_account_truncating()
    }

    fn try_distribute_reward(
        pool_currency_id: CurrencyIdOf<T>,
        reward_currency_id: CurrencyIdOf<T>,
        amount: BalanceOf<T>,
    ) -> Result<(), DispatchError> {
        frame_support::storage::with_transaction::<_, DispatchError, _>(|| {
            let res = T::RewardPools::distribute_reward(&pool_currency_id, reward_currency_id, amount);
            if res.is_ok() {
                TransactionOutcome::Commit(res)
            } else {
                TransactionOutcome::Rollback(res)
            }
        })
    }
}
