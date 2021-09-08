//! # Fee Module
//! Based on the [specification](https://interlay.gitlab.io/polkabtc-spec/spec/fee.html).

#![deny(warnings)]
#![cfg_attr(test, feature(proc_macro_hygiene))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

mod default_weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod ext;
pub mod types;

#[cfg(test)]
extern crate mocktopus;

#[cfg(test)]
use mocktopus::macros::mockable;

use codec::{Decode, Encode, EncodeLike};
use currency::{Amount, OnSweep};
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    ensure,
    traits::Get,
    transactional,
    weights::Weight,
    PalletId,
};
use frame_system::ensure_signed;
use reward::Rewards;
use sp_arithmetic::{traits::*, FixedPointNumber, FixedPointOperand};
use sp_runtime::traits::{AccountIdConversion, AtLeast32BitUnsigned};
use sp_std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    vec::*,
};
use types::{BalanceOf, Collateral, SignedFixedPoint, UnsignedFixedPoint, UnsignedInner, Version, Wrapped};

pub trait WeightInfo {
    fn withdraw_rewards() -> Weight;
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// ## Configuration
    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + security::Config
        + currency::Config<UnsignedFixedPoint = UnsignedFixedPoint<Self>, SignedFixedPoint = SignedFixedPoint<Self>>
    {
        /// The fee module id, used for deriving its sovereign account ID.
        #[pallet::constant]
        type FeePalletId: Get<PalletId>;

        /// Weight information for the extrinsics in this module.
        type WeightInfo: WeightInfo;

        /// Signed fixed point type.
        type SignedFixedPoint: FixedPointNumber<Inner = <Self as Config>::SignedInner> + Encode + EncodeLike + Decode;

        /// The `Inner` type of the `SignedFixedPoint`.
        type SignedInner: Debug
            + CheckedDiv
            + TryFrom<Collateral<Self>>
            + TryFrom<Wrapped<Self>>
            + TryInto<Collateral<Self>>
            + TryInto<Wrapped<Self>>
            + MaybeSerializeDeserialize
            + TryInto<BalanceOf<Self>>;

        /// Unsigned fixed point type.
        type UnsignedFixedPoint: FixedPointNumber<Inner = <Self as Config>::UnsignedInner>
            + Encode
            + EncodeLike
            + Decode
            + MaybeSerializeDeserialize;

        /// The `Inner` type of the `UnsignedFixedPoint`.
        type UnsignedInner: Debug
            + One
            + CheckedMul
            + CheckedDiv
            + FixedPointOperand
            + AtLeast32BitUnsigned
            + Default
            + Encode
            + EncodeLike
            + Decode
            + MaybeSerializeDeserialize;

        /// Vault reward pool for the wrapped currency.
        type VaultRewards: reward::Rewards<Self::AccountId, SignedFixedPoint = SignedFixedPoint<Self>>;

        /// Vault staking pool for the wrapped currency.
        type VaultStaking: staking::Staking<Self::AccountId, SignedFixedPoint = SignedFixedPoint<Self>>;

        /// Handler to transfer undistributed rewards.
        type OnSweep: OnSweep<Self::AccountId, Amount<Self>>;
    }

    #[pallet::error]
    pub enum Error<T> {
        ArithmeticOverflow,
        ArithmeticUnderflow,
        InvalidRewardDist,
        TryIntoIntError,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// # Issue

    /// Fee share that users need to pay to issue tokens.
    #[pallet::storage]
    #[pallet::getter(fn issue_fee)]
    pub type IssueFee<T: Config> = StorageValue<_, UnsignedFixedPoint<T>, ValueQuery>;

    /// Default griefing collateral (e.g. DOT/KSM) as a percentage of the locked
    /// collateral of a Vault a user has to lock to issue tokens.
    #[pallet::storage]
    #[pallet::getter(fn issue_griefing_collateral)]
    pub type IssueGriefingCollateral<T: Config> = StorageValue<_, UnsignedFixedPoint<T>, ValueQuery>;

    /// # Redeem

    /// Fee share that users need to pay to redeem tokens.
    #[pallet::storage]
    #[pallet::getter(fn redeem_fee)]
    pub type RedeemFee<T: Config> = StorageValue<_, UnsignedFixedPoint<T>, ValueQuery>;

    /// # Refund

    /// Fee share that users need to pay to refund overpaid tokens.
    #[pallet::storage]
    #[pallet::getter(fn refund_fee)]
    pub type RefundFee<T: Config> = StorageValue<_, UnsignedFixedPoint<T>, ValueQuery>;

    /// # Vault Registry

    /// If users execute a redeem with a Vault flagged for premium redeem,
    /// they can earn a collateral premium, slashed from the Vault.
    #[pallet::storage]
    #[pallet::getter(fn premium_redeem_fee)]
    pub type PremiumRedeemFee<T: Config> = StorageValue<_, UnsignedFixedPoint<T>, ValueQuery>;

    /// Fee that a Vault has to pay if it fails to execute redeem or replace requests
    /// (for redeem, on top of the slashed value of the request). The fee is
    /// paid in collateral based on the token amount at the current exchange rate.
    #[pallet::storage]
    #[pallet::getter(fn punishment_fee)]
    pub type PunishmentFee<T: Config> = StorageValue<_, UnsignedFixedPoint<T>, ValueQuery>;

    /// # Replace

    /// Default griefing collateral (e.g. DOT/KSM) as a percentage of the to-be-locked collateral
    /// of the new Vault. This collateral will be slashed and allocated to the replacing Vault
    /// if the to-be-replaced Vault does not transfer BTC on time.
    #[pallet::storage]
    #[pallet::getter(fn replace_griefing_collateral)]
    pub type ReplaceGriefingCollateral<T: Config> = StorageValue<_, UnsignedFixedPoint<T>, ValueQuery>;

    /// # Relayer

    /// Fee that is taken from a liquidated Vault on theft, used to pay the reporter.
    #[pallet::storage]
    #[pallet::getter(fn theft_fee)]
    pub type TheftFee<T: Config> = StorageValue<_, UnsignedFixedPoint<T>, ValueQuery>;

    /// Upper bound to the reward that can be payed to a reporter on success.
    #[pallet::storage]
    #[pallet::getter(fn theft_fee_max)]
    pub type TheftFeeMax<T: Config> = StorageValue<_, UnsignedInner<T>, ValueQuery>;

    /// AccountId of the fee pool.
    #[pallet::storage]
    pub type FeePoolAccountId<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

    #[pallet::type_value]
    pub(super) fn DefaultForStorageVersion() -> Version {
        Version::V0
    }

    /// Build storage at V1 (requires default 0).
    #[pallet::storage]
    #[pallet::getter(fn storage_version)]
    pub(super) type StorageVersion<T: Config> = StorageValue<_, Version, ValueQuery, DefaultForStorageVersion>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub issue_fee: UnsignedFixedPoint<T>,
        pub issue_griefing_collateral: UnsignedFixedPoint<T>,
        pub redeem_fee: UnsignedFixedPoint<T>,
        pub refund_fee: UnsignedFixedPoint<T>,
        pub premium_redeem_fee: UnsignedFixedPoint<T>,
        pub punishment_fee: UnsignedFixedPoint<T>,
        pub replace_griefing_collateral: UnsignedFixedPoint<T>,
        pub theft_fee: UnsignedFixedPoint<T>,
        pub theft_fee_max: UnsignedInner<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                issue_fee: Default::default(),
                issue_griefing_collateral: Default::default(),
                redeem_fee: Default::default(),
                refund_fee: Default::default(),
                premium_redeem_fee: Default::default(),
                punishment_fee: Default::default(),
                replace_griefing_collateral: Default::default(),
                theft_fee: Default::default(),
                theft_fee_max: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            IssueFee::<T>::put(self.issue_fee);
            IssueGriefingCollateral::<T>::put(self.issue_griefing_collateral);
            RedeemFee::<T>::put(self.redeem_fee);
            RefundFee::<T>::put(self.refund_fee);
            PremiumRedeemFee::<T>::put(self.premium_redeem_fee);
            PunishmentFee::<T>::put(self.punishment_fee);
            ReplaceGriefingCollateral::<T>::put(self.replace_griefing_collateral);
            TheftFee::<T>::put(self.theft_fee);
            TheftFeeMax::<T>::put(self.theft_fee_max);
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Withdraw all rewards from the `origin` account in the `vault_id` staking pool.
        ///
        /// # Arguments
        ///
        /// * `origin` - signing account
        #[pallet::weight(<T as Config>::WeightInfo::withdraw_rewards())]
        #[transactional]
        pub fn withdraw_rewards(origin: OriginFor<T>, vault_id: T::AccountId) -> DispatchResultWithPostInfo {
            ext::security::ensure_parachain_status_not_shutdown::<T>()?;
            let nominator_id = ensure_signed(origin)?;
            Self::withdraw_from_reward_pool::<T::VaultRewards, T::VaultStaking>(&vault_id, &nominator_id)?;
            Ok(().into())
        }
    }
}

// "Internal" functions, callable by code.
#[cfg_attr(test, mockable)]
impl<T: Config> Pallet<T> {
    /// The account ID of the fee pool.
    ///
    /// This actually does computation. If you need to keep using it, then make sure you cache the
    /// value and only call this once.
    pub fn fee_pool_account_id() -> T::AccountId {
        <T as Config>::FeePalletId::get().into_account()
    }

    // Public functions exposed to other pallets

    /// Distribute rewards to participants.
    ///
    /// # Arguments
    ///
    /// * `amount` - amount of rewards
    pub fn distribute_rewards(amount: &Amount<T>) -> Result<(), DispatchError> {
        // distribute vault rewards and return leftover
        let remaining = Self::distribute(amount)?;
        if !remaining.is_zero() {
            // sweep the remaining rewards to the treasury if non-zero
            T::OnSweep::on_sweep(&Self::fee_pool_account_id(), remaining)?;
        }
        Ok(())
    }

    /// Calculate the required issue fee in tokens.
    ///
    /// # Arguments
    ///
    /// * `amount` - issue amount in tokens
    pub fn get_issue_fee(amount: &Amount<T>) -> Result<Amount<T>, DispatchError> {
        amount.rounded_mul(<IssueFee<T>>::get())
    }

    /// Calculate the required issue griefing collateral.
    ///
    /// # Arguments
    ///
    /// * `amount` - issue amount in collateral (at current exchange rate)
    pub fn get_issue_griefing_collateral(amount: &Amount<T>) -> Result<Amount<T>, DispatchError> {
        amount.rounded_mul(<IssueGriefingCollateral<T>>::get())
    }

    /// Calculate the required redeem fee in tokens. Upon execution, the
    /// rewards should be forwarded to the fee pool instead of being burned.
    ///
    /// # Arguments
    ///
    /// * `amount` - redeem amount in tokens
    pub fn get_redeem_fee(amount: &Amount<T>) -> Result<Amount<T>, DispatchError> {
        amount.rounded_mul(<RedeemFee<T>>::get())
    }

    /// Calculate the premium redeem fee in collateral for a user to get if redeeming
    /// with a Vault below the premium redeem threshold.
    ///
    /// # Arguments
    ///
    /// * `amount` - amount in collateral (at current exchange rate)
    pub fn get_premium_redeem_fee(amount: &Amount<T>) -> Result<Amount<T>, DispatchError> {
        amount.rounded_mul(<PremiumRedeemFee<T>>::get())
    }

    /// Calculate punishment fee for a Vault that fails to execute a redeem
    /// request before the expiry.
    ///
    /// # Arguments
    ///
    /// * `amount` - amount in collateral (at current exchange rate)
    pub fn get_punishment_fee(amount: &Amount<T>) -> Result<Amount<T>, DispatchError> {
        amount.rounded_mul(<PunishmentFee<T>>::get())
    }

    /// Calculate the required replace griefing collateral.
    ///
    /// # Arguments
    ///
    /// * `amount` - replace amount in collateral (at current exchange rate)
    pub fn get_replace_griefing_collateral(amount: &Amount<T>) -> Result<Amount<T>, DispatchError> {
        amount.rounded_mul(<ReplaceGriefingCollateral<T>>::get())
    }

    /// Calculate the fee taken from a liquidated Vault on theft.
    ///
    /// # Arguments
    ///
    /// * `amount` - the vault's backing collateral
    pub fn get_theft_fee(amount: &Amount<T>) -> Result<Amount<T>, DispatchError> {
        amount.rounded_mul(<TheftFee<T>>::get())
    }

    /// Calculate the fee portion of a total amount. For `amount = fee + refund_amount`, this
    /// function returns `fee`.
    ///
    /// # Arguments
    ///
    /// * `amount` - total amount in tokens
    pub fn get_refund_fee_from_total(amount: &Amount<T>) -> Result<Amount<T>, DispatchError> {
        // calculate 'percentage' = x / (1+x)
        let percentage = <RefundFee<T>>::get()
            .checked_div(
                &<RefundFee<T>>::get()
                    .checked_add(&UnsignedFixedPoint::<T>::one())
                    .ok_or(Error::<T>::ArithmeticOverflow)?,
            )
            .ok_or(Error::<T>::ArithmeticUnderflow)?;
        amount.rounded_mul(percentage)
    }

    pub fn withdraw_all_vault_rewards(account_id: &T::AccountId) -> DispatchResult {
        Self::distribute_from_reward_pool::<T::VaultRewards, T::VaultStaking>(account_id)?;
        Ok(())
    }

    // Private functions internal to this pallet

    #[allow(dead_code)]
    /// Helper for validating the `chain_spec` parameters
    fn ensure_rationals_sum_to_one(dist: Vec<UnsignedFixedPoint<T>>) -> DispatchResult {
        let sum = dist.iter().fold(UnsignedFixedPoint::<T>::default(), |a, &b| a + b);
        let one = UnsignedFixedPoint::<T>::checked_from_integer(UnsignedInner::<T>::one())
            .ok_or(Error::<T>::ArithmeticOverflow)?;
        ensure!(sum == one, Error::<T>::InvalidRewardDist);
        Ok(())
    }

    /// Withdraw rewards from a pool and transfer to `account_id`.
    fn withdraw_from_reward_pool<
        Rewards: reward::Rewards<T::AccountId, SignedFixedPoint = SignedFixedPoint<T>>,
        Staking: staking::Staking<T::AccountId, SignedFixedPoint = SignedFixedPoint<T>>,
    >(
        vault_id: &T::AccountId,
        nominator_id: &T::AccountId,
    ) -> DispatchResult {
        Self::distribute_from_reward_pool::<Rewards, Staking>(&vault_id)?;

        let rewards = Staking::withdraw_reward(vault_id, nominator_id)?
            .try_into()
            .map_err(|_| Error::<T>::TryIntoIntError)?;
        let amount = Amount::<T>::new(rewards, T::GetWrappedCurrencyId::get());
        amount.transfer(&Self::fee_pool_account_id(), nominator_id)?;

        Ok(())
    }

    fn distribute(reward: &Amount<T>) -> Result<Amount<T>, DispatchError> {
        let leftover = T::VaultRewards::distribute_reward(reward.to_signed_fixed_point()?)?;
        Amount::from_signed_fixed_point(leftover, reward.currency())
    }

    pub fn distribute_from_reward_pool<
        Rewards: reward::Rewards<T::AccountId, SignedFixedPoint = SignedFixedPoint<T>>,
        Staking: staking::Staking<T::AccountId, SignedFixedPoint = SignedFixedPoint<T>>,
    >(
        account_id: &T::AccountId,
    ) -> DispatchResult {
        let reward_as_inner = Rewards::withdraw_reward(account_id)?;
        let reward_as_fixed =
            Rewards::SignedFixedPoint::checked_from_integer(reward_as_inner).ok_or(Error::<T>::TryIntoIntError)?;
        Staking::distribute_reward(account_id, reward_as_fixed)?;
        Ok(())
    }
}
