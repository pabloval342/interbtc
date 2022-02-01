use crate::*;
use currency::Amount;

pub fn request_replace(
    old_vault_id: &VaultId,
    amount: Amount<Runtime>,
    griefing_collateral: Amount<Runtime>,
) -> DispatchResultWithPostInfo {
    Call::Replace(ReplaceCall::request_replace {
        currency_pair: old_vault_id.currencies.clone(),
        amount: amount.amount(),
        griefing_collateral: griefing_collateral.amount(),
    })
    .dispatch(origin_of(old_vault_id.account_id.clone()))
}

pub fn setup_replace(
    old_vault_id: &VaultId,
    new_vault_id: &VaultId,
    issued_tokens: Amount<Runtime>,
) -> (ReplaceRequest<AccountId32, BlockNumber, Balance, CurrencyId>, H256) {
    let new_vault_btc_address = BtcAddress::P2PKH(H160([2; 20]));

    assert_ok!(request_replace(
        old_vault_id,
        issued_tokens,
        DEFAULT_GRIEFING_COLLATERAL
    ));

    let (id, request) = accept_replace(
        &old_vault_id,
        &new_vault_id,
        issued_tokens,
        griefing(0),
        new_vault_btc_address,
    )
    .unwrap();
    (request, id)
}

pub fn assert_accept_replace_event() -> H256 {
    SystemPallet::events()
        .iter()
        .rev()
        .find_map(|record| match record.event {
            Event::Replace(ReplaceEvent::AcceptReplace { replace_id, .. }) => Some(replace_id),
            _ => None,
        })
        .unwrap()
}

pub fn accept_replace(
    old_vault_id: &VaultId,
    new_vault_id: &VaultId,
    amount_btc: Amount<Runtime>,
    collateral: Amount<Runtime>,
    btc_address: BtcAddress,
) -> Result<(H256, ReplaceRequest<AccountId32, BlockNumber, Balance, CurrencyId>), sp_runtime::DispatchError> {
    // assert_replace_request_event();

    Call::Replace(ReplaceCall::accept_replace {
        currency_pair: new_vault_id.currencies.clone(),
        old_vault: old_vault_id.clone(),
        amount_btc: amount_btc.amount(),
        collateral: collateral.amount(),
        btc_address: btc_address,
    })
    .dispatch(origin_of(new_vault_id.account_id.clone()))
    .map_err(|err| err.error)?;

    let replace_id = assert_accept_replace_event();
    let replace = ReplacePallet::get_open_replace_request(&replace_id).unwrap();
    Ok((replace_id, replace))
}
