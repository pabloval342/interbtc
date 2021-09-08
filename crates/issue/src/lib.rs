//! # Issue Pallet
//! Based on the [specification](https://interlay.gitlab.io/polkabtc-spec/spec/issue.html).

#![deny(warnings)]
#![cfg_attr(test, feature(proc_macro_hygiene))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

mod default_weights;
pub use default_weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(test)]
extern crate mocktopus;

#[cfg(test)]
use mocktopus::macros::mockable;

mod ext;
pub mod types;

#[doc(inline)]
pub use crate::types::{DefaultIssueRequest, IssueRequest, IssueRequestStatus};
use types::IssueRequestExt;

use crate::types::{BalanceOf, Collateral, Version, Wrapped};
use btc_relay::{BtcAddress, BtcPublicKey};
use currency::Amount;
use frame_support::{dispatch::DispatchError, ensure, traits::Get, transactional};
use frame_system::{ensure_root, ensure_signed};
pub use pallet::*;
use sp_core::H256;
use sp_std::vec::Vec;
use vault_registry::{CurrencySource, VaultStatus};

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
        + oracle::Config
        + fee::Config<UnsignedInner = BalanceOf<Self>>
        + refund::Config
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
        RequestIssue(
            H256,          // issue_id
            T::AccountId,  // requester
            Wrapped<T>,    // amount
            Wrapped<T>,    // fee
            Collateral<T>, // griefing_collateral
            T::AccountId,  // vault_id
            BtcAddress,    // vault deposit address
            BtcPublicKey,  // vault public key
        ),
        // issue_id, amount, fee, confiscated_griefing_collateral
        IssueAmountChange(H256, Wrapped<T>, Wrapped<T>, Collateral<T>),
        // [issue_id, requester, executed_amount, vault, fee]
        ExecuteIssue(H256, T::AccountId, Wrapped<T>, T::AccountId, Wrapped<T>),
        // [issue_id, requester, griefing_collateral]
        CancelIssue(H256, T::AccountId, Collateral<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientCollateral,
        IssueIdNotFound,
        CommitPeriodExpired,
        TimeNotExpired,
        IssueCompleted,
        IssueCancelled,
        VaultNotAcceptingNewIssues,
        WaitingForRelayerInitialization,
        /// Unable to convert value
        TryIntoIntError,
        ArithmeticUnderflow,
        ArithmeticOverflow,
        InvalidExecutor,
        AmountBelowDustAmount,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    /// Users create issue requests to issue tokens. This mapping provides access
    /// from a unique hash `IssueId` to an `IssueRequest` struct.
    #[pallet::storage]
    pub(super) type IssueRequests<T: Config> =
        StorageMap<_, Blake2_128Concat, H256, DefaultIssueRequest<T>, ValueQuery>;

    /// The time difference in number of blocks between an issue request is created
    /// and required completion time by a user. The issue period has an upper limit
    /// to prevent griefing of vault collateral.
    #[pallet::storage]
    #[pallet::getter(fn issue_period)]
    pub(super) type IssuePeriod<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    /// The minimum amount of btc that is required for issue requests; lower values would
    /// risk the rejection of payment on Bitcoin.
    #[pallet::storage]
    pub(super) type IssueBtcDustValue<T: Config> = StorageValue<_, Wrapped<T>, ValueQuery>;

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
        pub issue_period: T::BlockNumber,
        pub issue_btc_dust_value: Wrapped<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                issue_period: Default::default(),
                issue_btc_dust_value: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            IssuePeriod::<T>::put(self.issue_period);
            IssueBtcDustValue::<T>::put(self.issue_btc_dust_value);
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Request the issuance of tokens
        ///
        /// # Arguments
        ///
        /// * `origin` - sender of the transaction
        /// * `amount` - amount of BTC the user wants to convert to issued tokens. Note that the
        /// amount of issued tokens received will be less, because a fee is subtracted.
        /// * `vault` - address of the vault
        /// * `griefing_collateral` - amount of collateral
        #[pallet::weight(<T as Config>::WeightInfo::request_issue())]
        #[transactional]
        pub fn request_issue(
            origin: OriginFor<T>,
            #[pallet::compact] amount: Wrapped<T>,
            vault_id: T::AccountId,
            #[pallet::compact] griefing_collateral: Collateral<T>,
        ) -> DispatchResultWithPostInfo {
            let requester = ensure_signed(origin)?;
            Self::_request_issue(requester, amount, vault_id, griefing_collateral)?;
            Ok(().into())
        }

        /// Finalize the issuance of tokens
        ///
        /// # Arguments
        ///
        /// * `origin` - sender of the transaction
        /// * `issue_id` - identifier of issue request as output from request_issue
        /// * `tx_block_height` - block number of collateral chain
        /// * `merkle_proof` - raw bytes
        /// * `raw_tx` - raw bytes
        #[pallet::weight(<T as Config>::WeightInfo::execute_issue())]
        #[transactional]
        pub fn execute_issue(
            origin: OriginFor<T>,
            issue_id: H256,
            merkle_proof: Vec<u8>,
            raw_tx: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let executor = ensure_signed(origin)?;
            Self::_execute_issue(executor, issue_id, merkle_proof, raw_tx)?;
            Ok(().into())
        }

        /// Cancel the issuance of tokens if expired
        ///
        /// # Arguments
        ///
        /// * `origin` - sender of the transaction
        /// * `issue_id` - identifier of issue request as output from request_issue
        #[pallet::weight(<T as Config>::WeightInfo::cancel_issue())]
        #[transactional]
        pub fn cancel_issue(origin: OriginFor<T>, issue_id: H256) -> DispatchResultWithPostInfo {
            let requester = ensure_signed(origin)?;
            Self::_cancel_issue(requester, issue_id)?;
            Ok(().into())
        }

        /// Set the default issue period for tx verification.
        ///
        /// # Arguments
        ///
        /// * `origin` - the dispatch origin of this call (must be _Root_)
        /// * `period` - default period for new requests
        ///
        /// # Weight: `O(1)`
        #[pallet::weight(<T as Config>::WeightInfo::set_issue_period())]
        #[transactional]
        pub fn set_issue_period(origin: OriginFor<T>, period: T::BlockNumber) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            <IssuePeriod<T>>::set(period);
            Ok(().into())
        }
    }
}

// "Internal" functions, callable by code.
#[cfg_attr(test, mockable)]
impl<T: Config> Pallet<T> {
    /// Requests CBA issuance, returns unique tracking ID.
    fn _request_issue(
        requester: T::AccountId,
        amount_requested: Wrapped<T>,
        vault_id: T::AccountId,
        griefing_collateral: Collateral<T>,
    ) -> Result<H256, DispatchError> {
        let amount_requested = Amount::new(amount_requested, T::GetWrappedCurrencyId::get());
        let griefing_collateral = Amount::new(griefing_collateral, T::GetGriefingCollateralCurrencyId::get());

        // Check that Parachain is RUNNING
        ext::security::ensure_parachain_status_not_shutdown::<T>()?;

        ensure!(
            ext::btc_relay::is_fully_initialized::<T>()?,
            Error::<T>::WaitingForRelayerInitialization
        );

        let vault = ext::vault_registry::get_active_vault_from_id::<T>(&vault_id)?;

        // ensure that the vault is accepting new issues
        ensure!(
            vault.status == VaultStatus::Active(true),
            Error::<T>::VaultNotAcceptingNewIssues
        );

        // Check that the vault is currently not banned
        ext::vault_registry::ensure_not_banned::<T>(&vault_id)?;

        // calculate griefing collateral based on the total amount of tokens to be issued
        let amount_collateral = amount_requested.convert_to(T::GetGriefingCollateralCurrencyId::get())?;
        let expected_griefing_collateral = ext::fee::get_issue_griefing_collateral::<T>(&amount_collateral)?;

        ensure!(
            griefing_collateral.ge(&expected_griefing_collateral)?,
            Error::<T>::InsufficientCollateral
        );
        griefing_collateral.lock(&requester)?;

        // only continue if the payment is above the dust value
        ensure!(
            amount_requested.ge(&Self::issue_btc_dust_value())?,
            Error::<T>::AmountBelowDustAmount
        );

        ext::vault_registry::try_increase_to_be_issued_tokens::<T>(&vault_id, &amount_requested)?;

        let fee = ext::fee::get_issue_fee::<T>(&amount_requested)?;
        // calculate the amount of tokens that will be transferred to the user upon execution
        let amount_user = amount_requested.checked_sub(&fee)?;

        let issue_id = ext::security::get_secure_id::<T>(&requester);
        let btc_address = ext::vault_registry::register_deposit_address::<T>(&vault_id, issue_id)?;

        let request = IssueRequest {
            vault: vault_id,
            opentime: ext::security::active_block_number::<T>(),
            requester,
            btc_address,
            btc_public_key: vault.wallet.public_key,
            amount: amount_user.amount(),
            fee: fee.amount(),
            griefing_collateral: griefing_collateral.amount(),
            period: Self::issue_period(),
            btc_height: ext::btc_relay::get_best_block_height::<T>(),
            status: IssueRequestStatus::Pending,
        };
        Self::insert_issue_request(&issue_id, &request);

        Self::deposit_event(<Event<T>>::RequestIssue(
            issue_id,
            request.requester,
            request.amount,
            request.fee,
            request.griefing_collateral,
            request.vault,
            request.btc_address,
            request.btc_public_key,
        ));
        Ok(issue_id)
    }

    /// Completes CBA issuance, removing request from storage and minting token.
    fn _execute_issue(
        executor: T::AccountId,
        issue_id: H256,
        raw_merkle_proof: Vec<u8>,
        raw_tx: Vec<u8>,
    ) -> Result<(), DispatchError> {
        // Check that Parachain is RUNNING
        ext::security::ensure_parachain_status_not_shutdown::<T>()?;

        let mut issue = Self::get_issue_request_from_id(&issue_id)?;
        let mut maybe_refund_id = None;
        // allow anyone to complete issue request
        let requester = issue.requester.clone();

        // only executable before the request has expired
        ensure!(
            !ext::btc_relay::has_request_expired::<T>(
                issue.opentime,
                issue.btc_height,
                Self::issue_period().max(issue.period)
            )?,
            Error::<T>::CommitPeriodExpired
        );

        let transaction = ext::btc_relay::parse_transaction::<T>(&raw_tx)?;
        let merkle_proof = ext::btc_relay::parse_merkle_proof::<T>(&raw_merkle_proof)?;
        let (refund_address, amount_transferred) = ext::btc_relay::get_and_verify_issue_payment::<T, Wrapped<T>>(
            merkle_proof,
            transaction,
            issue.btc_address,
        )?;
        let amount_transferred = Amount::new(amount_transferred, T::GetWrappedCurrencyId::get());

        let expected_total_amount = issue.amount().checked_add(&issue.fee())?;

        // check for unexpected bitcoin amounts, and update the issue struct
        if amount_transferred.lt(&expected_total_amount)? {
            // only the requester of the issue can execute payments with insufficient amounts
            ensure!(requester == executor, Error::<T>::InvalidExecutor);

            // decrease the to-be-issued tokens that will not be issued after all
            let deficit = expected_total_amount.checked_sub(&amount_transferred)?;
            ext::vault_registry::decrease_to_be_issued_tokens::<T>(&issue.vault, &deficit)?;

            // slash/release griefing collateral proportionally to the amount sent
            let released_collateral = ext::vault_registry::calculate_collateral::<T>(
                &issue.griefing_collateral(),
                &amount_transferred,
                &expected_total_amount,
            )?;
            released_collateral.unlock(&requester)?;
            let slashed_collateral = issue.griefing_collateral().checked_sub(&released_collateral)?;
            ext::vault_registry::transfer_funds::<T>(
                CurrencySource::Griefing(issue.requester.clone()),
                CurrencySource::FreeBalance(issue.vault.clone()),
                &slashed_collateral,
            )?;

            Self::update_issue_amount(&issue_id, &mut issue, amount_transferred, slashed_collateral)?;
        } else {
            // release griefing collateral
            let griefing_collateral: Amount<T> = issue.griefing_collateral();
            griefing_collateral.unlock(&requester)?;

            if amount_transferred.gt(&expected_total_amount)?
                && !ext::vault_registry::is_vault_liquidated::<T>(&issue.vault)?
            {
                let surplus_btc = amount_transferred.checked_sub(&expected_total_amount)?;

                match ext::vault_registry::try_increase_to_be_issued_tokens::<T>(&issue.vault, &surplus_btc) {
                    Ok(_) => {
                        // Current vault can handle the surplus; update the issue request
                        Self::update_issue_amount(
                            &issue_id,
                            &mut issue,
                            amount_transferred,
                            Amount::zero(T::GetGriefingCollateralCurrencyId::get()),
                        )?;
                    }
                    Err(_) => {
                        // vault does not have enough collateral to accept the over payment, so refund.
                        maybe_refund_id = ext::refund::request_refund::<T>(
                            &surplus_btc,
                            issue.vault.clone(),
                            issue.requester.clone(),
                            refund_address,
                            issue_id,
                        )?;
                    }
                }
            }
        };

        // issue struct may have been update above; recalculate the total
        let issue_amount = issue.amount();
        let issue_fee = issue.fee();
        let total = issue_amount.checked_add(&issue_fee)?;
        ext::vault_registry::issue_tokens::<T>(&issue.vault, &total)?;

        // mint issued tokens
        issue_amount.mint(&requester)?;

        // mint wrapped fees
        issue_fee.mint(&ext::fee::fee_pool_account_id::<T>())?;

        // distribute rewards
        ext::fee::distribute_rewards::<T>(&issue_fee)?;

        Self::set_issue_status(issue_id, IssueRequestStatus::Completed(maybe_refund_id));

        Self::deposit_event(<Event<T>>::ExecuteIssue(
            issue_id,
            requester,
            total.amount(),
            issue.vault,
            issue.fee,
        ));
        Ok(())
    }

    /// Cancels CBA issuance if time has expired and slashes collateral.
    fn _cancel_issue(requester: T::AccountId, issue_id: H256) -> Result<(), DispatchError> {
        // Check that Parachain is RUNNING
        ext::security::ensure_parachain_status_not_shutdown::<T>()?;

        let issue = Self::get_issue_request_from_id(&issue_id)?;

        // only cancellable after the request has expired
        ensure!(
            ext::btc_relay::has_request_expired::<T>(
                issue.opentime,
                issue.btc_height,
                Self::issue_period().max(issue.period)
            )?,
            Error::<T>::TimeNotExpired
        );

        // Decrease to-be-redeemed tokens:
        let full_amount = issue.amount().checked_add(&issue.fee())?;

        ext::vault_registry::decrease_to_be_issued_tokens::<T>(&issue.vault, &full_amount)?;

        let griefing_collateral = issue.griefing_collateral();
        if ext::vault_registry::is_vault_liquidated::<T>(&issue.vault)? {
            griefing_collateral.unlock(&issue.requester)?;
        } else {
            ext::vault_registry::transfer_funds::<T>(
                CurrencySource::Griefing(issue.requester.clone()),
                CurrencySource::FreeBalance(issue.vault.clone()),
                &griefing_collateral,
            )?;
        }
        Self::set_issue_status(issue_id, IssueRequestStatus::Cancelled);

        Self::deposit_event(<Event<T>>::CancelIssue(issue_id, requester, issue.griefing_collateral));
        Ok(())
    }

    /// Fetch all issue requests for the specified account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - user account id
    pub fn get_issue_requests_for_account(account_id: T::AccountId) -> Vec<(H256, DefaultIssueRequest<T>)> {
        <IssueRequests<T>>::iter()
            .filter(|(_, request)| request.requester == account_id)
            .collect::<Vec<_>>()
    }

    /// Fetch all issue requests for the specified vault.
    ///
    /// # Arguments
    ///
    /// * `account_id` - vault account id
    pub fn get_issue_requests_for_vault(account_id: T::AccountId) -> Vec<(H256, DefaultIssueRequest<T>)> {
        <IssueRequests<T>>::iter()
            .filter(|(_, request)| request.vault == account_id)
            .collect::<Vec<_>>()
    }

    pub fn get_issue_request_from_id(issue_id: &H256) -> Result<DefaultIssueRequest<T>, DispatchError> {
        let request = IssueRequests::<T>::try_get(issue_id).or(Err(Error::<T>::IssueIdNotFound))?;

        // NOTE: temporary workaround until we delete
        match request.status {
            IssueRequestStatus::Completed(_) => Err(Error::<T>::IssueCompleted.into()),
            IssueRequestStatus::Cancelled => Err(Error::<T>::IssueCancelled.into()),
            IssueRequestStatus::Pending => Ok(request),
        }
    }

    /// update the fee & amount in an issue request based on the actually transferred amount
    fn update_issue_amount(
        issue_id: &H256,
        issue: &mut DefaultIssueRequest<T>,
        transferred_btc: Amount<T>,
        confiscated_griefing_collateral: Amount<T>,
    ) -> Result<(), DispatchError> {
        // Current vault can handle the surplus; update the issue request
        issue.fee = ext::fee::get_issue_fee::<T>(&transferred_btc)?.amount();
        issue.amount = transferred_btc.checked_sub(&issue.fee())?.amount();

        // update storage
        <IssueRequests<T>>::mutate(issue_id, |x| {
            x.fee = issue.fee;
            x.amount = issue.amount;
        });

        Self::deposit_event(<Event<T>>::IssueAmountChange(
            *issue_id,
            issue.amount,
            issue.fee,
            confiscated_griefing_collateral.amount(),
        ));

        Ok(())
    }

    fn insert_issue_request(key: &H256, value: &DefaultIssueRequest<T>) {
        <IssueRequests<T>>::insert(key, value)
    }

    fn set_issue_status(id: H256, status: IssueRequestStatus) {
        // TODO: delete issue request from storage
        <IssueRequests<T>>::mutate(id, |request| {
            request.status = status;
        });
    }

    fn issue_btc_dust_value() -> Amount<T> {
        Amount::new(IssueBtcDustValue::<T>::get(), T::GetWrappedCurrencyId::get())
    }
}
