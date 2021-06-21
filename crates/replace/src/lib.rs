//! # Replace Pallet
//! Based on the [specification](https://interlay.gitlab.io/polkabtc-spec/spec/replace.html).

#![deny(warnings)]
#![cfg_attr(test, feature(proc_macro_hygiene))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

mod default_weights;
pub use default_weights::WeightInfo;

#[cfg(test)]
extern crate mocktopus;

use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    ensure, transactional,
};
use frame_system::{ensure_root, ensure_signed};
#[cfg(test)]
use mocktopus::macros::mockable;
use sp_core::H256;
use sp_runtime::traits::Zero;
use sp_std::vec::Vec;

use btc_relay::BtcAddress;

#[doc(inline)]
pub use crate::types::{ReplaceRequest, ReplaceRequestStatus};

use crate::types::{BalanceOf, Collateral, Version, Wrapped};
use vault_registry::CurrencySource;

mod ext;
pub mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

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
        + vault_registry::Config
        + btc_relay::Config
        + exchange_rate_oracle::Config<Balance = BalanceOf<Self>>
        + fee::Config
        + sla::Config<Balance = BalanceOf<Self>>
        + nomination::Config
    {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Weight information for the extrinsics in this module.
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", Wrapped<T> = "Wrapped", Collateral<T> = "Collateral")]
    pub enum Event<T: Config> {
        // [old_vault_id, amount_btc, griefing_collateral]
        RequestReplace(T::AccountId, Wrapped<T>, Collateral<T>),
        // [old_vault_id, withdrawn_tokens, withdrawn_griefing_collateral]
        WithdrawReplace(T::AccountId, Wrapped<T>, Collateral<T>),
        // [replace_id, old_vault_id, new_vault_id, amount, collateral, btc_address]
        AcceptReplace(H256, T::AccountId, T::AccountId, Wrapped<T>, Collateral<T>, BtcAddress),
        // [replace_id, old_vault_id, new_vault_id]
        ExecuteReplace(H256, T::AccountId, T::AccountId),
        // [replace_id, new_vault_id, old_vault_id, griefing_collateral]
        CancelReplace(H256, T::AccountId, T::AccountId, Collateral<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        AmountBelowDustAmount,
        NoReplacement,
        InsufficientCollateral,
        NoPendingRequest,
        UnauthorizedVault,
        ReplaceSelfNotAllowed,
        CancelAcceptedRequest,
        CollateralBelowSecureThreshold,
        VaultHasEnabledNomination,
        ReplacePeriodExpired,
        ReplacePeriodNotExpired,
        ReplaceCompleted,
        ReplaceCancelled,
        ReplaceIdNotFound,
        /// Unable to convert value
        TryIntoIntError,
        ArithmeticUnderflow,
        ArithmeticOverflow,
    }

    /// Vaults create replace requests to transfer locked collateral.
    /// This mapping provides access from a unique hash to a `ReplaceRequest`.
    #[pallet::storage]
    pub(super) type ReplaceRequests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256,
        ReplaceRequest<T::AccountId, T::BlockNumber, Wrapped<T>, Collateral<T>>,
        ValueQuery,
    >;

    /// The time difference in number of blocks between when a replace request is created
    /// and required completion time by a vault. The replace period has an upper limit
    /// to prevent griefing of vault collateral.
    #[pallet::storage]
    #[pallet::getter(fn replace_period)]
    pub(super) type ReplacePeriod<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    /// The minimum amount of btc that is accepted for replace requests; any lower values would
    /// risk the bitcoin client to reject the payment
    #[pallet::storage]
    #[pallet::getter(fn replace_btc_dust_value)]
    pub(super) type ReplaceBtcDustValue<T: Config> = StorageValue<_, Wrapped<T>, ValueQuery>;

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
        pub replace_period: T::BlockNumber,
        pub replace_btc_dust_value: Wrapped<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                replace_period: Default::default(),
                replace_btc_dust_value: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            ReplacePeriod::<T>::put(self.replace_period);
            ReplaceBtcDustValue::<T>::put(self.replace_btc_dust_value);
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Request the replacement of a new vault ownership
        ///
        /// # Arguments
        ///
        /// * `origin` - sender of the transaction
        /// * `amount` - amount of issued tokens
        /// * `griefing_collateral` - amount of collateral
        #[pallet::weight(<T as Config>::WeightInfo::request_replace())]
        #[transactional]
        pub fn request_replace(
            origin: OriginFor<T>,
            #[pallet::compact] amount: Wrapped<T>,
            #[pallet::compact] griefing_collateral: Collateral<T>,
        ) -> DispatchResultWithPostInfo {
            ext::security::ensure_parachain_status_not_shutdown::<T>()?;
            let old_vault = ensure_signed(origin)?;
            Self::_request_replace(old_vault, amount, griefing_collateral)?;
            Ok(().into())
        }

        /// Withdraw a request of vault replacement
        ///
        /// # Arguments
        ///
        /// * `origin` - sender of the transaction: the old vault
        #[pallet::weight(<T as Config>::WeightInfo::withdraw_replace())]
        #[transactional]
        pub fn withdraw_replace(
            origin: OriginFor<T>,
            #[pallet::compact] amount: Wrapped<T>,
        ) -> DispatchResultWithPostInfo {
            ext::security::ensure_parachain_status_not_shutdown::<T>()?;
            let old_vault = ensure_signed(origin)?;
            Self::_withdraw_replace_request(old_vault, amount)?;
            Ok(().into())
        }

        /// Accept request of vault replacement
        ///
        /// # Arguments
        ///
        /// * `origin` - the initiator of the transaction: the new vault
        /// * `old_vault` - id of the old vault that we are (possibly partially) replacing
        /// * `collateral` - the collateral for replacement
        /// * `btc_address` - the address that old-vault should transfer the btc to
        #[pallet::weight(<T as Config>::WeightInfo::accept_replace())]
        #[transactional]
        pub fn accept_replace(
            origin: OriginFor<T>,
            old_vault: T::AccountId,
            #[pallet::compact] amount_btc: Wrapped<T>,
            #[pallet::compact] collateral: Collateral<T>,
            btc_address: BtcAddress,
        ) -> DispatchResultWithPostInfo {
            ext::security::ensure_parachain_status_not_shutdown::<T>()?;
            let new_vault = ensure_signed(origin)?;
            Self::_accept_replace(old_vault, new_vault, amount_btc, collateral, btc_address)?;
            Ok(().into())
        }

        /// Execute vault replacement
        ///
        /// # Arguments
        ///
        /// * `origin` - sender of the transaction: the new vault
        /// * `replace_id` - the ID of the replacement request
        /// * 'merkle_proof' - the merkle root of the block
        /// * `raw_tx` - the transaction id in bytes
        #[pallet::weight(<T as Config>::WeightInfo::execute_replace())]
        #[transactional]
        pub fn execute_replace(
            origin: OriginFor<T>,
            replace_id: H256,
            merkle_proof: Vec<u8>,
            raw_tx: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            ext::security::ensure_parachain_status_not_shutdown::<T>()?;
            let _ = ensure_signed(origin)?;
            Self::_execute_replace(replace_id, merkle_proof, raw_tx)?;
            Ok(().into())
        }

        /// Cancel vault replacement
        ///
        /// # Arguments
        ///
        /// * `origin` - sender of the transaction: the new vault
        /// * `replace_id` - the ID of the replacement request
        #[pallet::weight(<T as Config>::WeightInfo::cancel_replace())]
        #[transactional]
        pub fn cancel_replace(origin: OriginFor<T>, replace_id: H256) -> DispatchResultWithPostInfo {
            ext::security::ensure_parachain_status_not_shutdown::<T>()?;
            let new_vault = ensure_signed(origin)?;
            Self::_cancel_replace(new_vault, replace_id)?;
            Ok(().into())
        }

        /// Set the default replace period for tx verification.
        ///
        /// # Arguments
        ///
        /// * `origin` - the dispatch origin of this call (must be _Root_)
        /// * `period` - default period for new requests
        ///
        /// # Weight: `O(1)`
        #[pallet::weight(<T as Config>::WeightInfo::set_replace_period())]
        #[transactional]
        pub fn set_replace_period(origin: OriginFor<T>, period: T::BlockNumber) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            <ReplacePeriod<T>>::set(period);
            Ok(().into())
        }
    }
}

// "Internal" functions, callable by code.
#[cfg_attr(test, mockable)]
impl<T: Config> Pallet<T> {
    fn _request_replace(
        vault_id: T::AccountId,
        amount_btc: Wrapped<T>,
        griefing_collateral: Collateral<T>,
    ) -> DispatchResult {
        // check vault is not banned
        ext::vault_registry::ensure_not_banned::<T>(&vault_id)?;

        ensure!(
            !ext::nomination::is_nominatable::<T>(&vault_id)?,
            Error::<T>::VaultHasEnabledNomination
        );

        let requestable_tokens = ext::vault_registry::requestable_to_be_replaced_tokens::<T>(&vault_id)?;
        let to_be_replaced_increase = amount_btc.min(requestable_tokens);
        let replace_collateral_increase = if amount_btc.is_zero() {
            griefing_collateral
        } else {
            ext::vault_registry::calculate_collateral::<T>(griefing_collateral, to_be_replaced_increase, amount_btc)?
        };

        // increase to-be-replaced tokens. This will fail if the vault does not have enough tokens available
        let (total_to_be_replaced, total_griefing_collateral) =
            ext::vault_registry::try_increase_to_be_replaced_tokens::<T>(
                &vault_id,
                to_be_replaced_increase,
                replace_collateral_increase,
            )?;

        // check that total-to-be-replaced is above the minimum. NOTE: this means that even
        // a request with amount=0 is valid, as long the _total_ is above DUST. This might
        // be the case if the vault just wants to increase the griefing collateral, for example.
        let dust_value = <ReplaceBtcDustValue<T>>::get();
        ensure!(total_to_be_replaced >= dust_value, Error::<T>::AmountBelowDustAmount);

        // check that that the total griefing collateral is sufficient to back the total to-be-replaced amount
        let required_collateral = ext::fee::get_replace_griefing_collateral::<T>(
            ext::oracle::wrapped_to_collateral::<T>(total_to_be_replaced)?,
        )?;
        ensure!(
            total_griefing_collateral >= required_collateral,
            Error::<T>::InsufficientCollateral
        );

        // Lock the oldVault’s griefing collateral. Note that this directly locks the amount
        // without going through the vault registry, so that this amount can not be used to
        // back issued tokens
        ext::collateral::lock_collateral::<T>(&vault_id, replace_collateral_increase)?;

        // Emit RequestReplace event
        Self::deposit_event(<Event<T>>::RequestReplace(
            vault_id,
            to_be_replaced_increase,
            replace_collateral_increase,
        ));
        Ok(())
    }

    fn _withdraw_replace_request(vault_id: T::AccountId, amount: Wrapped<T>) -> Result<(), DispatchError> {
        // decrease to-be-replaced tokens, so that the vault is free to use its issued tokens again.
        let (withdrawn_tokens, to_withdraw_collateral) =
            ext::vault_registry::decrease_to_be_replaced_tokens::<T>(&vault_id, amount)?;

        // release the used collateral
        ext::collateral::release_collateral::<T>(&vault_id, to_withdraw_collateral)?;

        if withdrawn_tokens.is_zero() {
            return Err(Error::<T>::NoPendingRequest.into());
        }

        // Emit WithdrawReplaceRequest event.
        Self::deposit_event(<Event<T>>::WithdrawReplace(
            vault_id,
            withdrawn_tokens,
            to_withdraw_collateral,
        ));
        Ok(())
    }

    fn _accept_replace(
        old_vault_id: T::AccountId,
        new_vault_id: T::AccountId,
        amount_btc: Wrapped<T>,
        collateral: Collateral<T>,
        btc_address: BtcAddress,
    ) -> Result<(), DispatchError> {
        // don't allow vaults to replace themselves
        ensure!(old_vault_id != new_vault_id, Error::<T>::ReplaceSelfNotAllowed);

        // Check that new vault is not currently banned
        ext::vault_registry::ensure_not_banned::<T>(&new_vault_id)?;

        // Add the new replace address to the vault's wallet,
        // this should also verify that the vault exists
        ext::vault_registry::insert_vault_deposit_address::<T>(&new_vault_id, btc_address)?;

        // decrease old-vault's to-be-replaced tokens
        let (redeemable_tokens, griefing_collateral) =
            ext::vault_registry::decrease_to_be_replaced_tokens::<T>(&old_vault_id, amount_btc)?;

        // check amount_btc is above the minimum
        let dust_value = <ReplaceBtcDustValue<T>>::get();
        ensure!(redeemable_tokens >= dust_value, Error::<T>::AmountBelowDustAmount);

        // Calculate and lock the new-vault's additional collateral
        let actual_new_vault_collateral =
            ext::vault_registry::calculate_collateral::<T>(collateral, redeemable_tokens, amount_btc)?;

        ext::vault_registry::try_deposit_collateral::<T>(&new_vault_id, actual_new_vault_collateral)?;

        // increase old-vault's to-be-redeemed tokens - this should never fail
        ext::vault_registry::try_increase_to_be_redeemed_tokens::<T>(&old_vault_id, redeemable_tokens)?;

        // increase new-vault's to-be-issued tokens - this will fail if there is insufficient collateral
        ext::vault_registry::try_increase_to_be_issued_tokens::<T>(&new_vault_id, redeemable_tokens)?;

        let replace_id = ext::security::get_secure_id::<T>(&old_vault_id);

        let replace = ReplaceRequest {
            old_vault: old_vault_id,
            new_vault: new_vault_id,
            accept_time: ext::security::active_block_number::<T>(),
            collateral: actual_new_vault_collateral,
            btc_address,
            griefing_collateral,
            amount: redeemable_tokens,
            period: Self::replace_period(),
            btc_height: ext::btc_relay::get_best_block_height::<T>(),
            status: ReplaceRequestStatus::Pending,
        };

        Self::insert_replace_request(&replace_id, &replace);

        Self::insert_replace_request(&replace_id, &replace);

        // Emit AcceptReplace event
        Self::deposit_event(<Event<T>>::AcceptReplace(
            replace_id,
            replace.old_vault,
            replace.new_vault,
            replace.amount,
            replace.collateral,
            replace.btc_address,
        ));

        Ok(())
    }

    fn _execute_replace(replace_id: H256, raw_merkle_proof: Vec<u8>, raw_tx: Vec<u8>) -> Result<(), DispatchError> {
        // Retrieve the ReplaceRequest as per the replaceId parameter from Vaults in the VaultRegistry
        let replace = Self::get_open_replace_request(&replace_id)?;

        // NOTE: anyone can call this method provided the proof is correct
        let new_vault_id = replace.new_vault;
        let old_vault_id = replace.old_vault;

        // check the transaction inclusion and validity
        let transaction = ext::btc_relay::parse_transaction::<T>(&raw_tx)?;
        let merkle_proof = ext::btc_relay::parse_merkle_proof::<T>(&raw_merkle_proof)?;
        ext::btc_relay::verify_and_validate_op_return_transaction::<T, _>(
            merkle_proof,
            transaction,
            replace.btc_address,
            replace.amount,
            replace_id,
        )?;

        // decrease old-vault's issued & to-be-redeemed tokens, and
        // change new-vault's to-be-issued tokens to issued tokens
        ext::vault_registry::replace_tokens::<T>(
            old_vault_id.clone(),
            new_vault_id.clone(),
            replace.amount,
            replace.collateral,
        )?;

        // if the old vault has not been liquidated, give it back its griefing collateral
        ext::collateral::release_collateral::<T>(&old_vault_id, replace.griefing_collateral)?;

        // Emit ExecuteReplace event.
        Self::deposit_event(<Event<T>>::ExecuteReplace(replace_id, old_vault_id, new_vault_id));

        // Remove replace request
        Self::set_replace_status(&replace_id, ReplaceRequestStatus::Completed);
        Ok(())
    }

    fn _cancel_replace(caller: T::AccountId, replace_id: H256) -> Result<(), DispatchError> {
        // Retrieve the ReplaceRequest as per the replaceId parameter from Vaults in the VaultRegistry
        let replace = Self::get_open_replace_request(&replace_id)?;

        // only cancellable after the request has expired
        ensure!(
            ext::security::has_expired::<T>(replace.accept_time, Self::replace_period().max(replace.period))?,
            Error::<T>::ReplacePeriodNotExpired
        );

        let new_vault_id = replace.new_vault;

        // only cancellable by new_vault
        ensure!(caller == new_vault_id, Error::<T>::UnauthorizedVault);

        // decrease old-vault's to-be-redeemed tokens, and
        // decrease new-vault's to-be-issued tokens
        ext::vault_registry::cancel_replace_tokens::<T>(&replace.old_vault, &new_vault_id, replace.amount)?;

        // slash old-vault's griefing collateral
        if !ext::vault_registry::is_vault_liquidated::<T>(&new_vault_id)? {
            // new-vault is not liquidated - give it the griefing collateral
            ext::vault_registry::transfer_funds::<T>(
                CurrencySource::Griefing(replace.old_vault.clone()),
                CurrencySource::Collateral(new_vault_id.clone()),
                replace.griefing_collateral,
            )?;
        } else {
            // new-vault is liquidated - slash to its free balance
            ext::vault_registry::transfer_funds::<T>(
                CurrencySource::Griefing(replace.old_vault.clone()),
                CurrencySource::FreeBalance(new_vault_id.clone()),
                replace.griefing_collateral,
            )?;
        }

        // if the new_vault locked additional collateral especially for this replace,
        // release it if it does not cause him to be undercollateralized
        if !ext::vault_registry::is_vault_liquidated::<T>(&new_vault_id)?
            && ext::vault_registry::is_allowed_to_withdraw_collateral::<T>(&new_vault_id, replace.collateral)?
        {
            ext::vault_registry::force_withdraw_collateral::<T>(&new_vault_id, replace.collateral)?;
        }

        // Remove the ReplaceRequest from ReplaceRequests
        Self::set_replace_status(&replace_id, ReplaceRequestStatus::Cancelled);

        // Emit CancelReplace event.
        Self::deposit_event(<Event<T>>::CancelReplace(
            replace_id,
            new_vault_id,
            replace.old_vault,
            replace.griefing_collateral,
        ));
        Ok(())
    }

    /// Fetch all replace requests from the specified vault.
    ///
    /// # Arguments
    ///
    /// * `account_id` - user account id
    pub fn get_replace_requests_for_old_vault(
        account_id: T::AccountId,
    ) -> Vec<(
        H256,
        ReplaceRequest<T::AccountId, T::BlockNumber, Wrapped<T>, Collateral<T>>,
    )> {
        <ReplaceRequests<T>>::iter()
            .filter(|(_, request)| request.old_vault == account_id)
            .collect::<Vec<_>>()
    }

    /// Fetch all replace requests to the specified vault.
    ///
    /// # Arguments
    ///
    /// * `account_id` - user account id
    pub fn get_replace_requests_for_new_vault(
        account_id: T::AccountId,
    ) -> Vec<(
        H256,
        ReplaceRequest<T::AccountId, T::BlockNumber, Wrapped<T>, Collateral<T>>,
    )> {
        <ReplaceRequests<T>>::iter()
            .filter(|(_, request)| request.new_vault == account_id)
            .collect::<Vec<_>>()
    }

    /// Get a replace request by id. Completed or cancelled requests are not returned.
    pub fn get_open_replace_request(
        id: &H256,
    ) -> Result<ReplaceRequest<T::AccountId, T::BlockNumber, Wrapped<T>, Collateral<T>>, DispatchError> {
        if !<ReplaceRequests<T>>::contains_key(id) {
            return Err(Error::<T>::ReplaceIdNotFound.into());
        }
        let request = <ReplaceRequests<T>>::get(id);
        // NOTE: temporary workaround until we delete
        match request.status {
            ReplaceRequestStatus::Pending => Ok(request),
            ReplaceRequestStatus::Completed => Err(Error::<T>::ReplaceCompleted.into()),
            ReplaceRequestStatus::Cancelled => Err(Error::<T>::ReplaceCancelled.into()),
        }
    }

    /// Get a open or completed replace request by id. Cancelled requests are not returned.
    pub fn get_open_or_completed_replace_request(
        id: &H256,
    ) -> Result<ReplaceRequest<T::AccountId, T::BlockNumber, Wrapped<T>, Collateral<T>>, DispatchError> {
        if !<ReplaceRequests<T>>::contains_key(id) {
            return Err(Error::<T>::ReplaceIdNotFound.into());
        }
        let request = <ReplaceRequests<T>>::get(id);
        match request.status {
            ReplaceRequestStatus::Pending | ReplaceRequestStatus::Completed => Ok(request),
            ReplaceRequestStatus::Cancelled => Err(Error::<T>::ReplaceCancelled.into()),
        }
    }

    fn insert_replace_request(
        key: &H256,
        value: &ReplaceRequest<T::AccountId, T::BlockNumber, Wrapped<T>, Collateral<T>>,
    ) {
        <ReplaceRequests<T>>::insert(key, value)
    }

    fn set_replace_status(key: &H256, status: ReplaceRequestStatus) {
        // TODO: delete old replace request from storage
        <ReplaceRequests<T>>::mutate(key, |request| {
            request.status = status;
        });
    }
}
