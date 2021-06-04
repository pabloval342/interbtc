#[cfg(test)]
use mocktopus::macros::mockable;

#[cfg_attr(test, mockable)]
pub(crate) mod btc_relay {
    use btc_relay::BtcAddress;
    use frame_support::dispatch::DispatchError;
    use sp_core::H256;
    use sp_std::vec::Vec;

    pub fn verify_and_validate_transaction<T: btc_relay::Config>(
        raw_merkle_proof: Vec<u8>,
        raw_tx: Vec<u8>,
        recipient_btc_address: BtcAddress,
        expected_btc: Option<i64>,
        op_return_id: Option<H256>,
        confirmations: Option<u32>,
    ) -> Result<(BtcAddress, i64), DispatchError> {
        <btc_relay::Pallet<T>>::_verify_and_validate_transaction(
            raw_merkle_proof,
            raw_tx,
            recipient_btc_address,
            expected_btc,
            op_return_id,
            confirmations,
        )
    }

    pub fn get_best_block_height<T: btc_relay::Config>() -> u32 {
        <btc_relay::Pallet<T>>::get_best_block_height()
    }

    pub fn is_fully_initialized<T: btc_relay::Config>() -> Result<bool, DispatchError> {
        <btc_relay::Pallet<T>>::is_fully_initialized()
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod vault_registry {
    use crate::types::{Collateral, Wrapped};
    use btc_relay::BtcAddress;
    use frame_support::dispatch::{DispatchError, DispatchResult};
    use sp_core::H256;
    use vault_registry::types::{CurrencySource, Vault};

    pub fn transfer_funds<T: vault_registry::Config>(
        from: CurrencySource<T>,
        to: CurrencySource<T>,
        amount: Collateral<T>,
    ) -> DispatchResult {
        <vault_registry::Pallet<T>>::transfer_funds(from, to, amount)
    }

    pub fn is_vault_liquidated<T: vault_registry::Config>(vault_id: &T::AccountId) -> Result<bool, DispatchError> {
        <vault_registry::Pallet<T>>::is_vault_liquidated(vault_id)
    }

    pub fn get_active_vault_from_id<T: vault_registry::Config>(
        vault_id: &T::AccountId,
    ) -> Result<
        Vault<T::AccountId, T::BlockNumber, Wrapped<T>, Collateral<T>, <T as vault_registry::Config>::SignedFixedPoint>,
        DispatchError,
    > {
        <vault_registry::Pallet<T>>::get_active_vault_from_id(vault_id)
    }

    pub fn try_increase_to_be_issued_tokens<T: vault_registry::Config>(
        vault_id: &T::AccountId,
        amount: Wrapped<T>,
    ) -> Result<(), DispatchError> {
        <vault_registry::Pallet<T>>::try_increase_to_be_issued_tokens(vault_id, amount)
    }

    pub fn register_deposit_address<T: vault_registry::Config>(
        vault_id: &T::AccountId,
        secure_id: H256,
    ) -> Result<BtcAddress, DispatchError> {
        <vault_registry::Pallet<T>>::register_deposit_address(vault_id, secure_id)
    }

    pub fn issue_tokens<T: vault_registry::Config>(vault_id: &T::AccountId, amount: Wrapped<T>) -> DispatchResult {
        <vault_registry::Pallet<T>>::issue_tokens(vault_id, amount)
    }

    pub fn ensure_not_banned<T: vault_registry::Config>(vault: &T::AccountId) -> DispatchResult {
        <vault_registry::Pallet<T>>::_ensure_not_banned(vault)
    }

    pub fn decrease_to_be_issued_tokens<T: vault_registry::Config>(
        vault_id: &T::AccountId,
        tokens: Wrapped<T>,
    ) -> DispatchResult {
        <vault_registry::Pallet<T>>::decrease_to_be_issued_tokens(vault_id, tokens)
    }

    pub fn calculate_collateral<T: vault_registry::Config>(
        collateral: Collateral<T>,
        numerator: Wrapped<T>,
        denominator: Wrapped<T>,
    ) -> Result<Collateral<T>, DispatchError> {
        <vault_registry::Pallet<T>>::calculate_collateral(collateral, numerator, denominator)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod collateral {
    use crate::types::Collateral;
    use frame_support::dispatch::DispatchResult;

    type CollateralPallet<T> = currency::Pallet<T, currency::Collateral>;

    pub fn lock_collateral<T: currency::Config<currency::Collateral>>(
        sender: &T::AccountId,
        amount: Collateral<T>,
    ) -> DispatchResult {
        CollateralPallet::<T>::lock(sender, amount)
    }

    pub fn release_collateral<T: currency::Config<currency::Collateral>>(
        sender: &T::AccountId,
        amount: Collateral<T>,
    ) -> DispatchResult {
        CollateralPallet::<T>::release(sender, amount)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod treasury {
    use crate::types::Wrapped;

    type TreasuryPallet<T> = currency::Pallet<T, currency::Wrapped>;

    pub fn mint<T: currency::Config<currency::Wrapped>>(requester: T::AccountId, amount: Wrapped<T>) {
        TreasuryPallet::<T>::mint(requester, amount)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod security {
    use frame_support::dispatch::{DispatchError, DispatchResult};
    use sp_core::H256;

    pub fn get_secure_id<T: security::Config>(id: &T::AccountId) -> H256 {
        <security::Pallet<T>>::get_secure_id(id)
    }

    pub fn ensure_parachain_status_not_shutdown<T: security::Config>() -> DispatchResult {
        <security::Pallet<T>>::ensure_parachain_status_not_shutdown()
    }

    pub fn active_block_number<T: security::Config>() -> T::BlockNumber {
        <security::Pallet<T>>::active_block_number()
    }

    pub fn has_expired<T: security::Config>(
        opentime: T::BlockNumber,
        period: T::BlockNumber,
    ) -> Result<bool, DispatchError> {
        <security::Pallet<T>>::has_expired(opentime, period)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod oracle {
    use crate::types::{Collateral, Wrapped};
    use frame_support::dispatch::DispatchError;

    pub fn wrapped_to_collateral<T: exchange_rate_oracle::Config>(
        amount: Wrapped<T>,
    ) -> Result<Collateral<T>, DispatchError> {
        <exchange_rate_oracle::Pallet<T>>::wrapped_to_collateral(amount)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod sla {
    use crate::types::{Collateral, Wrapped};
    use frame_support::dispatch::DispatchError;
    pub use sla::types::VaultEvent;

    pub fn event_update_vault_sla<T: sla::Config>(
        vault_id: &T::AccountId,
        event: VaultEvent<Wrapped<T>, Collateral<T>>,
    ) -> Result<(), DispatchError> {
        <sla::Pallet<T>>::event_update_vault_sla(vault_id, event)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod fee {
    use crate::types::{Collateral, Wrapped};
    use frame_support::dispatch::{DispatchError, DispatchResult};

    pub fn distribute_collateral_rewards<T: fee::Config>(amount: Collateral<T>) -> DispatchResult {
        <fee::Pallet<T>>::distribute_collateral_rewards(amount)
    }

    pub fn fee_pool_account_id<T: fee::Config>() -> T::AccountId {
        <fee::Pallet<T>>::fee_pool_account_id()
    }

    pub fn get_issue_fee<T: fee::Config>(amount: Wrapped<T>) -> Result<Wrapped<T>, DispatchError> {
        <fee::Pallet<T>>::get_issue_fee(amount)
    }

    pub fn get_issue_griefing_collateral<T: fee::Config>(
        amount: Collateral<T>,
    ) -> Result<Collateral<T>, DispatchError> {
        <fee::Pallet<T>>::get_issue_griefing_collateral(amount)
    }

    pub fn distribute_wrapped_rewards<T: fee::Config>(amount: Wrapped<T>) -> DispatchResult {
        <fee::Pallet<T>>::distribute_wrapped_rewards(amount)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod refund {
    use crate::types::Wrapped;
    use btc_relay::BtcAddress;
    use frame_support::dispatch::DispatchError;
    use sp_core::H256;

    pub fn request_refund<T: refund::Config>(
        total_amount_btc: Wrapped<T>,
        vault_id: T::AccountId,
        issuer: T::AccountId,
        btc_address: BtcAddress,
        issue_id: H256,
    ) -> Result<Option<H256>, DispatchError> {
        <refund::Pallet<T>>::request_refund(total_amount_btc, vault_id, issuer, btc_address, issue_id)
    }
}
