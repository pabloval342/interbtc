#[cfg(test)]
use mocktopus::macros::mockable;

#[cfg_attr(test, mockable)]
pub(crate) mod btc_relay {
    use bitcoin::types::{MerkleProof, Transaction, Value};
    use btc_relay::BtcAddress;
    use frame_support::dispatch::DispatchError;
    use sp_core::H256;
    use sp_std::convert::TryInto;

    pub fn verify_and_validate_op_return_transaction<T: crate::Config, V: TryInto<Value>>(
        merkle_proof: MerkleProof,
        transaction: Transaction,
        recipient_btc_address: BtcAddress,
        expected_btc: V,
        op_return_id: H256,
    ) -> Result<(), DispatchError> {
        <btc_relay::Pallet<T>>::verify_and_validate_op_return_transaction(
            merkle_proof,
            transaction,
            recipient_btc_address,
            expected_btc,
            op_return_id,
        )
    }

    pub fn get_best_block_height<T: crate::Config>() -> u32 {
        <btc_relay::Pallet<T>>::get_best_block_height()
    }

    pub fn parse_transaction<T: btc_relay::Config>(raw_tx: &[u8]) -> Result<Transaction, DispatchError> {
        <btc_relay::Pallet<T>>::parse_transaction(raw_tx)
    }

    pub fn parse_merkle_proof<T: btc_relay::Config>(raw_merkle_proof: &[u8]) -> Result<MerkleProof, DispatchError> {
        <btc_relay::Pallet<T>>::parse_merkle_proof(raw_merkle_proof)
    }

    pub fn has_request_expired<T: crate::Config>(
        opentime: T::BlockNumber,
        btc_open_height: u32,
        period: T::BlockNumber,
    ) -> Result<bool, DispatchError> {
        <btc_relay::Pallet<T>>::has_request_expired(opentime, btc_open_height, period)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod vault_registry {
    use crate::types::{BalanceOf, Collateral, Wrapped};
    use frame_support::dispatch::{DispatchError, DispatchResult};
    use vault_registry::types::{CurrencySource, Vault};

    pub fn get_liquidated_collateral<T: crate::Config>(
        vault_id: &T::AccountId,
    ) -> Result<Collateral<T>, DispatchError> {
        <vault_registry::Pallet<T>>::get_liquidated_collateral(vault_id)
    }

    pub fn transfer_funds<T: crate::Config>(
        from: CurrencySource<T>,
        to: CurrencySource<T>,
        amount: Collateral<T>,
    ) -> DispatchResult {
        <vault_registry::Pallet<T>>::transfer_funds(from, to, amount)
    }

    pub fn transfer_funds_saturated<T: crate::Config>(
        from: CurrencySource<T>,
        to: CurrencySource<T>,
        amount: Collateral<T>,
    ) -> Result<Collateral<T>, DispatchError> {
        <vault_registry::Pallet<T>>::transfer_funds_saturated(from, to, amount)
    }

    pub fn get_vault_from_id<T: crate::Config>(
        vault_id: &T::AccountId,
    ) -> Result<Vault<T::AccountId, T::BlockNumber, BalanceOf<T>>, DispatchError> {
        <vault_registry::Pallet<T>>::get_vault_from_id(vault_id)
    }

    pub fn try_increase_to_be_redeemed_tokens<T: crate::Config>(
        vault_id: &T::AccountId,
        amount: Wrapped<T>,
    ) -> DispatchResult {
        <vault_registry::Pallet<T>>::try_increase_to_be_redeemed_tokens(vault_id, amount)
    }

    pub fn redeem_tokens<T: crate::Config>(
        vault_id: &T::AccountId,
        tokens: Wrapped<T>,
        premium: Collateral<T>,
        redeemer_id: &T::AccountId,
    ) -> DispatchResult {
        <vault_registry::Pallet<T>>::redeem_tokens(vault_id, tokens, premium, redeemer_id)
    }

    pub fn decrease_tokens<T: crate::Config>(
        vault_id: &T::AccountId,
        user_id: &T::AccountId,
        tokens: Wrapped<T>,
    ) -> DispatchResult {
        <vault_registry::Pallet<T>>::decrease_tokens(vault_id, user_id, tokens)
    }

    pub fn decrease_liquidated_collateral<T: crate::Config>(
        vault_id: &T::AccountId,
        amount: Collateral<T>,
    ) -> DispatchResult {
        <vault_registry::Pallet<T>>::decrease_liquidated_collateral(vault_id, amount)
    }

    pub fn redeem_tokens_liquidation<T: crate::Config>(
        redeemer_id: &T::AccountId,
        amount: Wrapped<T>,
    ) -> DispatchResult {
        <vault_registry::Pallet<T>>::redeem_tokens_liquidation(redeemer_id, amount)
    }

    pub fn ban_vault<T: crate::Config>(vault_id: T::AccountId) -> DispatchResult {
        <vault_registry::Pallet<T>>::ban_vault(vault_id)
    }

    pub fn ensure_not_banned<T: crate::Config>(vault: &T::AccountId) -> DispatchResult {
        <vault_registry::Pallet<T>>::_ensure_not_banned(vault)
    }

    pub fn is_vault_below_premium_threshold<T: crate::Config>(vault_id: &T::AccountId) -> Result<bool, DispatchError> {
        <vault_registry::Pallet<T>>::is_vault_below_premium_threshold(vault_id)
    }

    pub fn is_vault_below_secure_threshold<T: crate::Config>(vault_id: &T::AccountId) -> Result<bool, DispatchError> {
        <vault_registry::Pallet<T>>::is_vault_below_secure_threshold(vault_id)
    }

    pub fn decrease_to_be_redeemed_tokens<T: crate::Config>(
        vault_id: &T::AccountId,
        tokens: Wrapped<T>,
    ) -> DispatchResult {
        <vault_registry::Pallet<T>>::decrease_to_be_redeemed_tokens(vault_id, tokens)
    }

    pub fn calculate_collateral<T: crate::Config>(
        collateral: Collateral<T>,
        numerator: Wrapped<T>,
        denominator: Wrapped<T>,
    ) -> Result<Collateral<T>, DispatchError> {
        <vault_registry::Pallet<T>>::calculate_collateral(collateral, numerator, denominator)
    }

    pub fn try_increase_to_be_issued_tokens<T: crate::Config>(
        vault_id: &T::AccountId,
        amount: Wrapped<T>,
    ) -> Result<(), DispatchError> {
        <vault_registry::Pallet<T>>::try_increase_to_be_issued_tokens(vault_id, amount)
    }

    pub fn issue_tokens<T: crate::Config>(vault_id: &T::AccountId, amount: Wrapped<T>) -> DispatchResult {
        <vault_registry::Pallet<T>>::issue_tokens(vault_id, amount)
    }

    pub fn decrease_to_be_replaced_tokens<T: crate::Config>(
        vault_id: &T::AccountId,
        tokens: Wrapped<T>,
    ) -> Result<(Wrapped<T>, Collateral<T>), DispatchError> {
        <vault_registry::Pallet<T>>::decrease_to_be_replaced_tokens(vault_id, tokens)
    }

    pub fn calculate_slashed_amount<T: crate::Config>(
        vault_id: &T::AccountId,
        stake: Collateral<T>,
        reimburse: bool,
    ) -> Result<Collateral<T>, DispatchError> {
        <vault_registry::Pallet<T>>::calculate_slashed_amount(vault_id, stake, reimburse)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod sla {
    use crate::types::BalanceOf;
    use frame_support::dispatch::DispatchError;
    pub use sla::Action;

    pub fn event_update_vault_sla<T: crate::Config>(
        vault_id: &T::AccountId,
        action: Action<BalanceOf<T>>,
    ) -> Result<(), DispatchError> {
        <sla::Pallet<T>>::event_update_vault_sla(vault_id, action)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod treasury {
    use crate::types::Wrapped;
    use currency::ParachainCurrency;
    use frame_support::dispatch::DispatchResult;

    pub fn get_balance<T: crate::Config>(account: &T::AccountId) -> Wrapped<T> {
        <T as vault_registry::Config>::Wrapped::get_free_balance(account)
    }

    pub fn lock<T: crate::Config>(redeemer: &T::AccountId, amount: Wrapped<T>) -> DispatchResult {
        <T as vault_registry::Config>::Wrapped::lock(redeemer, amount)
    }

    pub fn unlock<T: crate::Config>(account: &T::AccountId, amount: Wrapped<T>) -> DispatchResult {
        <T as vault_registry::Config>::Wrapped::unlock(account, amount)
    }

    pub fn burn<T: crate::Config>(redeemer: &T::AccountId, amount: Wrapped<T>) -> DispatchResult {
        <T as vault_registry::Config>::Wrapped::burn(redeemer, amount)
    }

    pub fn unlock_and_transfer<T: crate::Config>(
        source: &T::AccountId,
        destination: &T::AccountId,
        amount: Wrapped<T>,
    ) -> DispatchResult {
        <T as vault_registry::Config>::Wrapped::unlock_and_transfer(source, destination, amount)
    }

    pub fn mint<T: crate::Config>(requester: &T::AccountId, amount: Wrapped<T>) -> DispatchResult {
        <T as vault_registry::Config>::Wrapped::mint(requester, amount)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod security {
    use frame_support::dispatch::DispatchError;
    use sp_core::H256;

    pub fn get_secure_id<T: crate::Config>(id: &T::AccountId) -> H256 {
        <security::Pallet<T>>::get_secure_id(id)
    }

    pub fn ensure_parachain_status_not_shutdown<T: crate::Config>() -> Result<(), DispatchError> {
        <security::Pallet<T>>::ensure_parachain_status_not_shutdown()
    }

    pub fn active_block_number<T: crate::Config>() -> T::BlockNumber {
        <security::Pallet<T>>::active_block_number()
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod oracle {
    use crate::types::{Collateral, Wrapped};
    use exchange_rate_oracle::BtcTxFeesPerByte;
    use frame_support::dispatch::DispatchError;

    pub fn satoshi_per_bytes<T: crate::Config>() -> BtcTxFeesPerByte {
        <exchange_rate_oracle::Pallet<T>>::satoshi_per_bytes()
    }

    pub fn wrapped_to_collateral<T: crate::Config>(amount: Wrapped<T>) -> Result<Collateral<T>, DispatchError> {
        <exchange_rate_oracle::Pallet<T>>::wrapped_to_collateral(amount)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod fee {
    use crate::types::{Collateral, Wrapped};
    use frame_support::dispatch::{DispatchError, DispatchResult};

    pub fn fee_pool_account_id<T: crate::Config>() -> T::AccountId {
        <fee::Pallet<T>>::fee_pool_account_id()
    }

    pub fn get_redeem_fee<T: crate::Config>(amount: Wrapped<T>) -> Result<Wrapped<T>, DispatchError> {
        <fee::Pallet<T>>::get_redeem_fee(amount)
    }

    pub fn distribute_rewards<T: crate::Config>(amount: Wrapped<T>) -> DispatchResult {
        <fee::Pallet<T>>::distribute_rewards(amount)
    }

    pub fn get_punishment_fee<T: crate::Config>(amount: Collateral<T>) -> Result<Collateral<T>, DispatchError> {
        <fee::Pallet<T>>::get_punishment_fee(amount)
    }

    pub fn get_premium_redeem_fee<T: crate::Config>(amount: Collateral<T>) -> Result<Collateral<T>, DispatchError> {
        <fee::Pallet<T>>::get_premium_redeem_fee(amount)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod collateral {
    use crate::Collateral;
    use currency::ParachainCurrency;
    use frame_support::dispatch::DispatchResult;

    pub fn release_collateral<T: crate::Config>(sender: &T::AccountId, amount: Collateral<T>) -> DispatchResult {
        <T as vault_registry::Config>::Collateral::unlock(sender, amount)
    }
}
