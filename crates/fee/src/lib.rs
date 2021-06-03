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
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    ensure,
    traits::Get,
    transactional,
    weights::Weight,
    PalletId,
};
use frame_system::ensure_signed;
use sp_arithmetic::{traits::*, FixedPointNumber, FixedPointOperand};
use sp_runtime::traits::AccountIdConversion;
use sp_std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    vec::*,
};
use types::{Collateral, Inner, SignedFixedPoint, UnsignedFixedPoint, Version, Wrapped};

pub trait WeightInfo {
    fn withdraw_vault_rewards() -> Weight;
    fn withdraw_relayer_rewards() -> Weight;
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
        + currency::Config<currency::Collateral>
        + currency::Config<currency::Wrapped>
        + security::Config
    {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The fee module id, used for deriving its sovereign account ID.
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// Weight information for the extrinsics in this module.
        type WeightInfo: WeightInfo;

        /// Signed fixed point type.
        type SignedFixedPoint: FixedPointNumber<Inner = Self::SignedInner> + Encode + EncodeLike + Decode;

        /// The `Inner` type of the `SignedFixedPoint`.
        type SignedInner: Debug
            + CheckedDiv
            + TryFrom<Collateral<Self>>
            + TryFrom<Wrapped<Self>>
            + TryInto<Collateral<Self>>
            + TryInto<Wrapped<Self>>
            + MaybeSerializeDeserialize;

        /// Unsigned fixed point type.
        type UnsignedFixedPoint: FixedPointNumber<Inner = Self::UnsignedInner>
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
            + From<Collateral<Self>>
            + From<Wrapped<Self>>
            + Into<Collateral<Self>>
            + Into<Wrapped<Self>>;

        /// Vault reward pool for the collateral currency.
        type CollateralVaultRewards: reward::Rewards<Self::AccountId, SignedFixedPoint = SignedFixedPoint<Self>>;

        /// Vault reward pool for the wrapped currency.
        type WrappedVaultRewards: reward::Rewards<Self::AccountId, SignedFixedPoint = SignedFixedPoint<Self>>;

        /// Relayer reward pool for the collateral currency.
        type CollateralRelayerRewards: reward::Rewards<Self::AccountId, SignedFixedPoint = SignedFixedPoint<Self>>;

        /// Relayer reward pool for the wrapped currency.
        type WrappedRelayerRewards: reward::Rewards<Self::AccountId, SignedFixedPoint = SignedFixedPoint<Self>>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", Wrapped<T> = "Wrapped", Collateral<T> = "Collateral")]
    pub enum Event<T: Config> {
        WithdrawWrapped(T::AccountId, Wrapped<T>),
        WithdrawCollateral(T::AccountId, Collateral<T>),
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

    /// AccountId of the fee pool.
    #[pallet::storage]
    pub type FeePoolAccountId<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

    /// AccountId of the parachain maintainer.
    #[pallet::storage]
    #[pallet::getter(fn maintainer_account_id)]
    pub type MaintainerAccountId<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

    /// # Parachain Fee Pool Distribution

    /// Percentage of fees allocated to Vaults.
    #[pallet::storage]
    #[pallet::getter(fn vault_rewards)]
    pub type VaultRewards<T: Config> = StorageValue<_, UnsignedFixedPoint<T>, ValueQuery>;

    /// Percentage of fees allocated to Staked Relayers.
    #[pallet::storage]
    #[pallet::getter(fn relayer_rewards)]
    pub type RelayerRewards<T: Config> = StorageValue<_, UnsignedFixedPoint<T>, ValueQuery>;

    /// Percentage of fees allocated for development.
    #[pallet::storage]
    #[pallet::getter(fn maintainer_rewards)]
    pub type MaintainerRewards<T: Config> = StorageValue<_, UnsignedFixedPoint<T>, ValueQuery>;

    /// Percentage of fees generated by nominated collateral that is given to nominators.
    #[pallet::storage]
    #[pallet::getter(fn nomination_rewards)]
    pub type NominationRewards<T: Config> = StorageValue<_, UnsignedFixedPoint<T>, ValueQuery>;

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
        pub maintainer_account_id: T::AccountId,
        pub vault_rewards: UnsignedFixedPoint<T>,
        pub relayer_rewards: UnsignedFixedPoint<T>,
        pub maintainer_rewards: UnsignedFixedPoint<T>,
        pub nomination_rewards: UnsignedFixedPoint<T>,
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
                maintainer_account_id: Default::default(),
                vault_rewards: Default::default(),
                relayer_rewards: Default::default(),
                maintainer_rewards: Default::default(),
                nomination_rewards: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            Pallet::<T>::ensure_rationals_sum_to_one(vec![
                self.vault_rewards,
                self.relayer_rewards,
                self.maintainer_rewards,
            ])
            .unwrap();

            IssueFee::<T>::put(self.issue_fee);
            IssueGriefingCollateral::<T>::put(self.issue_griefing_collateral);
            RedeemFee::<T>::put(self.redeem_fee);
            RefundFee::<T>::put(self.refund_fee);
            PremiumRedeemFee::<T>::put(self.premium_redeem_fee);
            PunishmentFee::<T>::put(self.punishment_fee);
            ReplaceGriefingCollateral::<T>::put(self.replace_griefing_collateral);
            MaintainerAccountId::<T>::put(self.maintainer_account_id.clone());
            VaultRewards::<T>::put(self.vault_rewards);
            RelayerRewards::<T>::put(self.relayer_rewards);
            MaintainerRewards::<T>::put(self.maintainer_rewards);
            NominationRewards::<T>::put(self.nomination_rewards);
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Withdraw all vault collateral rewards.
        ///
        /// # Arguments
        ///
        /// * `origin` - signing account
        #[pallet::weight(<T as Config>::WeightInfo::withdraw_vault_rewards())]
        #[transactional]
        fn withdraw_vault_collateral_rewards(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ext::security::ensure_parachain_status_not_shutdown::<T>()?;
            let signer = ensure_signed(origin)?;
            Self::withdraw_collateral::<T::CollateralVaultRewards>(&signer)?;
            Ok(().into())
        }

        /// Withdraw all vault wrapped rewards.
        ///
        /// # Arguments
        ///
        /// * `origin` - signing account
        #[pallet::weight(<T as Config>::WeightInfo::withdraw_vault_rewards())]
        #[transactional]
        fn withdraw_vault_wrapped_rewards(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ext::security::ensure_parachain_status_not_shutdown::<T>()?;
            let signer = ensure_signed(origin)?;
            Self::withdraw_wrapped::<T::WrappedVaultRewards>(&signer)?;
            Ok(().into())
        }

        /// Withdraw all relayer collateral rewards.
        ///
        /// # Arguments
        ///
        /// * `origin` - signing account
        #[pallet::weight(<T as Config>::WeightInfo::withdraw_relayer_rewards())]
        #[transactional]
        fn withdraw_relayer_collateral_rewards(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ext::security::ensure_parachain_status_not_shutdown::<T>()?;
            let signer = ensure_signed(origin)?;
            Self::withdraw_collateral::<T::CollateralRelayerRewards>(&signer)?;
            Ok(().into())
        }

        /// Withdraw all relayer wrapped rewards.
        ///
        /// # Arguments
        ///
        /// * `origin` - signing account
        #[pallet::weight(<T as Config>::WeightInfo::withdraw_relayer_rewards())]
        #[transactional]
        fn withdraw_relayer_wrapped_rewards(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ext::security::ensure_parachain_status_not_shutdown::<T>()?;
            let signer = ensure_signed(origin)?;
            Self::withdraw_wrapped::<T::WrappedRelayerRewards>(&signer)?;
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
        <T as Config>::PalletId::get().into_account()
    }

    // Public functions exposed to other pallets

    /// Distribute collateral rewards to participants.
    ///
    /// # Arguments
    ///
    /// * `amount` - amount of the collateral currency
    pub fn distribute_collateral_rewards(amount: Collateral<T>) -> Result<(), DispatchError> {
        // calculate vault rewards
        let vault_rewards = Self::collateral_for(amount, Self::vault_rewards())?;
        let vault_rewards = Self::distribute::<_, _, T::SignedFixedPoint, T::CollateralVaultRewards>(vault_rewards)?;

        // calculate relayer rewards
        let relayer_rewards = Self::collateral_for(amount, Self::relayer_rewards())?;
        let relayer_rewards =
            Self::distribute::<_, _, T::SignedFixedPoint, T::CollateralRelayerRewards>(relayer_rewards)?;

        // give remaining rewards to maintainer (dev fund)
        let maintainer_rewards = amount.saturating_sub(
            vault_rewards
                .checked_add(&relayer_rewards)
                .ok_or(Error::<T>::ArithmeticOverflow)?,
        );
        let maintainer_account_id = Self::maintainer_account_id();
        ext::collateral::transfer::<T>(&Self::fee_pool_account_id(), &maintainer_account_id, maintainer_rewards)?;

        Ok(())
    }

    /// Distribute wrapped rewards to participants.
    ///
    /// # Arguments
    ///
    /// * `amount` - amount of the wrapped currency
    pub fn distribute_wrapped_rewards(amount: Wrapped<T>) -> Result<(), DispatchError> {
        // calculate vault rewards
        let vault_rewards = Self::wrapped_for(amount, Self::vault_rewards())?;
        let vault_rewards = Self::distribute::<_, _, T::SignedFixedPoint, T::WrappedVaultRewards>(vault_rewards)?;

        // calculate relayer rewards
        let relayer_rewards = Self::wrapped_for(amount, Self::relayer_rewards())?;
        let relayer_rewards = Self::distribute::<_, _, T::SignedFixedPoint, T::WrappedRelayerRewards>(relayer_rewards)?;

        // give remaining rewards to maintainer (dev fund)
        let maintainer_rewards = amount.saturating_sub(
            vault_rewards
                .checked_add(&relayer_rewards)
                .ok_or(Error::<T>::ArithmeticOverflow)?,
        );
        let maintainer_account_id = Self::maintainer_account_id();
        ext::treasury::transfer::<T>(&Self::fee_pool_account_id(), &maintainer_account_id, maintainer_rewards)?;

        Ok(())
    }

    /// Calculate the required issue fee in tokens.
    ///
    /// # Arguments
    ///
    /// * `amount` - issue amount in tokens
    pub fn get_issue_fee(amount: Wrapped<T>) -> Result<Wrapped<T>, DispatchError> {
        Self::wrapped_for(amount, <IssueFee<T>>::get())
    }

    /// Calculate the required issue griefing collateral.
    ///
    /// # Arguments
    ///
    /// * `amount` - issue amount in collateral (at current exchange rate)
    pub fn get_issue_griefing_collateral(amount: Collateral<T>) -> Result<Collateral<T>, DispatchError> {
        Self::collateral_for(amount, <IssueGriefingCollateral<T>>::get())
    }

    /// Calculate the required redeem fee in tokens. Upon execution, the
    /// rewards should be forwarded to the fee pool instead of being burned.
    ///
    /// # Arguments
    ///
    /// * `amount` - redeem amount in tokens
    pub fn get_redeem_fee(amount: Wrapped<T>) -> Result<Wrapped<T>, DispatchError> {
        Self::wrapped_for(amount, <RedeemFee<T>>::get())
    }

    /// Calculate the premium redeem fee in collateral for a user to get if redeeming
    /// with a Vault below the premium redeem threshold.
    ///
    /// # Arguments
    ///
    /// * `amount` - amount in collateral (at current exchange rate)
    pub fn get_premium_redeem_fee(amount: Collateral<T>) -> Result<Collateral<T>, DispatchError> {
        Self::collateral_for(amount, <PremiumRedeemFee<T>>::get())
    }

    /// Calculate punishment fee for a Vault that fails to execute a redeem
    /// request before the expiry.
    ///
    /// # Arguments
    ///
    /// * `amount` - amount in collateral (at current exchange rate)
    pub fn get_punishment_fee(amount: Collateral<T>) -> Result<Collateral<T>, DispatchError> {
        Self::collateral_for(amount, <PunishmentFee<T>>::get())
    }

    /// Calculate the required replace griefing collateral.
    ///
    /// # Arguments
    ///
    /// * `amount` - replace amount in collateral (at current exchange rate)
    pub fn get_replace_griefing_collateral(amount: Collateral<T>) -> Result<Collateral<T>, DispatchError> {
        Self::collateral_for(amount, <ReplaceGriefingCollateral<T>>::get())
    }

    /// Calculate the fee portion of a total amount. For `amount = fee + refund_amount`, this
    /// function returns `fee`.
    ///
    /// # Arguments
    ///
    /// * `amount` - total amount in tokens
    pub fn get_refund_fee_from_total(amount: Wrapped<T>) -> Result<Wrapped<T>, DispatchError> {
        // calculate 'percentage' = x / (1+x)
        let percentage = <RefundFee<T>>::get()
            .checked_div(
                &<RefundFee<T>>::get()
                    .checked_add(&UnsignedFixedPoint::<T>::one())
                    .ok_or(Error::<T>::ArithmeticOverflow)?,
            )
            .ok_or(Error::<T>::ArithmeticUnderflow)?;
        Self::wrapped_for(amount, percentage)
    }

    pub fn wrapped_for(amount: Wrapped<T>, percentage: UnsignedFixedPoint<T>) -> Result<Wrapped<T>, DispatchError> {
        Ok(Self::calculate_for(amount.into(), percentage)?.into())
    }

    pub fn collateral_for(
        amount: Collateral<T>,
        percentage: UnsignedFixedPoint<T>,
    ) -> Result<Collateral<T>, DispatchError> {
        Ok(Self::calculate_for(amount.into(), percentage)?.into())
    }

    // Private functions internal to this pallet

    /// Take the `percentage` of an `amount`
    fn calculate_for(amount: Inner<T>, percentage: UnsignedFixedPoint<T>) -> Result<Inner<T>, DispatchError> {
        // we add 0.5 before we do the final integer division to round the result we return.
        // note that unwrapping is safe because we use a constant
        let rounding_addition = UnsignedFixedPoint::<T>::checked_from_rational(1, 2).unwrap();

        UnsignedFixedPoint::<T>::checked_from_integer(amount)
            .ok_or(Error::<T>::ArithmeticOverflow)?
            .checked_mul(&percentage)
            .ok_or(Error::<T>::ArithmeticOverflow)?
            .checked_add(&rounding_addition)
            .ok_or(Error::<T>::ArithmeticOverflow)?
            .into_inner()
            .checked_div(&UnsignedFixedPoint::<T>::accuracy())
            .ok_or(Error::<T>::ArithmeticUnderflow.into())
    }

    #[allow(dead_code)]
    /// Helper for validating the `chain_spec` parameters
    fn ensure_rationals_sum_to_one(dist: Vec<UnsignedFixedPoint<T>>) -> DispatchResult {
        let sum = dist.iter().fold(UnsignedFixedPoint::<T>::default(), |a, &b| a + b);
        let one =
            UnsignedFixedPoint::<T>::checked_from_integer(Inner::<T>::one()).ok_or(Error::<T>::ArithmeticOverflow)?;
        ensure!(sum == one, Error::<T>::InvalidRewardDist);
        Ok(())
    }

    /// Withdraw collateral rewards and transfer to `account_id`.
    fn withdraw_collateral<R: reward::Rewards<T::AccountId, SignedFixedPoint = SignedFixedPoint<T>>>(
        account_id: &T::AccountId,
    ) -> DispatchResult {
        let collateral_rewards = R::withdraw_reward(account_id)?
            .try_into()
            .map_err(|_| Error::<T>::TryIntoIntError)?;
        ext::collateral::transfer::<T>(&Self::fee_pool_account_id(), account_id, collateral_rewards)?;
        Self::deposit_event(<Event<T>>::WithdrawCollateral(account_id.clone(), collateral_rewards));
        Ok(())
    }

    /// Withdraw wrapped rewards and transfer to `account_id`.
    fn withdraw_wrapped<R: reward::Rewards<T::AccountId, SignedFixedPoint = SignedFixedPoint<T>>>(
        account_id: &T::AccountId,
    ) -> DispatchResult {
        let wrapped_rewards = R::withdraw_reward(account_id)?
            .try_into()
            .map_err(|_| Error::<T>::TryIntoIntError)?;
        ext::treasury::transfer::<T>(&Self::fee_pool_account_id(), account_id, wrapped_rewards)?;
        Self::deposit_event(<Event<T>>::WithdrawWrapped(account_id.clone(), wrapped_rewards));
        Ok(())
    }

    fn distribute<
        Currency: TryInto<SignedInner>,
        SignedInner: TryInto<Currency> + CheckedDiv,
        SignedFixedPoint: FixedPointNumber<Inner = SignedInner>,
        Reward: reward::Rewards<T::AccountId, SignedFixedPoint = SignedFixedPoint>,
    >(
        reward: Currency,
    ) -> Result<Currency, DispatchError> {
        let reward_as_inner = reward.try_into().ok().ok_or(Error::<T>::TryIntoIntError)?;
        let reward_as_fixed =
            SignedFixedPoint::checked_from_integer(reward_as_inner).ok_or(Error::<T>::TryIntoIntError)?;
        Ok(Reward::distribute(reward_as_fixed)?
            .into_inner()
            .checked_div(&SignedFixedPoint::accuracy())
            .ok_or(Error::<T>::ArithmeticUnderflow)?
            .try_into()
            .ok()
            .ok_or(Error::<T>::TryIntoIntError)?)
    }
}
