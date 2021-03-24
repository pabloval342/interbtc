mod mock;

use mock::{redeem_testing_utils::*, *};

fn test_with<R>(execute: impl FnOnce() -> R) -> R {
    ExtBuilder::build().execute_with(|| {
        SystemModule::set_block_number(1);
        assert_ok!(ExchangeRateOracleModule::_set_exchange_rate(FixedU128::one()));
        set_default_thresholds();
        UserData::force_to(USER, default_user_state());
        CoreVaultData::force_to(VAULT, default_vault_state());
        execute()
    })
}

#[test]
fn integration_test_redeem_should_fail_if_not_running() {
    test_with(|| {
        SecurityModule::set_status(StatusCode::Shutdown);

        assert_noop!(
            Call::Redeem(RedeemCall::request_redeem(
                1000,
                BtcAddress::P2PKH(H160([0u8; 20])),
                account_of(BOB),
            ))
            .dispatch(origin_of(account_of(ALICE))),
            SecurityError::ParachainNotRunning,
        );
    });
}

#[test]
fn integration_test_redeem_polka_btc_execute() {
    test_with(|| {
        let polka_btc = 1_000;
        let collateral_vault = 1_000_000;

        let redeem_id = setup_redeem(polka_btc, USER, VAULT, collateral_vault);
        let redeem = RedeemModule::get_open_redeem_request_from_id(&redeem_id).unwrap();

        execute_redeem(polka_btc, redeem_id);

        assert_eq!(
            ParachainState::get(),
            ParachainState::default().with_changes(|user, vault, _, fee_pool| {
                vault.issued -= redeem.amount_btc;
                user.free_tokens -= polka_btc;
                fee_pool.tokens += redeem.fee;
            })
        );
    });
}

#[test]
fn integration_test_premium_redeem_polka_btc_execute() {
    test_with(|| {
        let polka_btc = 10_000;

        let user_btc_address = BtcAddress::P2PKH(H160([2; 20]));

        // make vault undercollateralized. Note that we place it under the liquidation threshold
        // as well, but as long as we don't call liquidate that's ok
        assert_ok!(ExchangeRateOracleModule::_set_exchange_rate(FixedU128::from(100)));

        // alice requests to redeem polka_btc from Bob
        assert_ok!(Call::Redeem(RedeemCall::request_redeem(
            polka_btc,
            user_btc_address,
            account_of(VAULT)
        ))
        .dispatch(origin_of(account_of(USER))));

        // assert that request happened and extract the id
        let redeem_id = assert_redeem_request_event();
        let redeem = RedeemModule::get_open_redeem_request_from_id(&redeem_id).unwrap();

        // send the btc from the vault to the user
        let (tx_id, _tx_block_height, merkle_proof, raw_tx) =
            generate_transaction_and_mine(user_btc_address, polka_btc, Some(redeem_id));

        SystemModule::set_block_number(1 + CONFIRMATIONS);

        assert_ok!(
            Call::Redeem(RedeemCall::execute_redeem(redeem_id, tx_id, merkle_proof, raw_tx))
                .dispatch(origin_of(account_of(VAULT)))
        );

        assert_eq!(
            ParachainState::get(),
            ParachainState::default().with_changes(|user, vault, _, fee_pool| {
                // fee moves from user to fee_pool
                user.free_tokens -= redeem.fee;
                fee_pool.tokens += redeem.fee;
                // amount_btc is burned from user and decreased on vault
                vault.issued -= redeem.amount_btc;
                user.free_tokens -= redeem.amount_btc;
                // premium dot is moved from vault to user
                vault.backing_collateral -= redeem.premium_dot;
                user.free_balance += redeem.premium_dot;
            })
        );

        assert!(redeem.premium_dot > 0); // sanity check that our test is useful
    });
}

#[test]
fn integration_test_redeem_polka_btc_liquidation_redeem() {
    test_with(|| {
        let issued = 400;
        let to_be_issued = 100;
        let to_be_redeemed = 50;
        let liquidation_redeem_amount = 325;

        CoreVaultData::force_to(
            VAULT,
            CoreVaultData {
                issued,
                to_be_issued,
                to_be_redeemed,
                backing_collateral: 10_000,
                ..Default::default()
            },
        );

        // create tokens for the vault and user
        drop_exchange_rate_and_liquidate(VAULT);

        let post_liquidation_state = ParachainState::get();

        assert_noop!(
            Call::Redeem(RedeemCall::liquidation_redeem(351)).dispatch(origin_of(account_of(USER))),
            VaultRegistryError::InsufficientTokensCommitted
        );

        assert_ok!(Call::Redeem(RedeemCall::liquidation_redeem(liquidation_redeem_amount))
            .dispatch(origin_of(account_of(USER))));

        // NOTE: changes are relative the the post liquidation state
        assert_eq!(
            ParachainState::get(),
            post_liquidation_state.with_changes(|user, _vault, liquidation_vault, _fee_pool| {
                let reward = (liquidation_vault.backing_collateral * liquidation_redeem_amount)
                    / (liquidation_vault.issued + liquidation_vault.to_be_issued);

                user.free_tokens -= liquidation_redeem_amount;
                user.free_balance += reward;

                liquidation_vault.issued -= liquidation_redeem_amount;
                liquidation_vault.backing_collateral -= reward;
            })
        );
    });
}

#[test]
fn integration_test_redeem_polka_btc_cancel_reimburse_sufficient_collateral_for_polkabtc() {
    test_with(|| {
        let amount_btc = 10_000;

        let redeem_id = setup_cancelable_redeem(USER, VAULT, 100000000, amount_btc);
        let redeem = RedeemModule::get_open_redeem_request_from_id(&redeem_id).unwrap();
        let amount_without_fee_dot = ExchangeRateOracleModule::btc_to_dots(redeem.amount_btc).unwrap();

        let punishment_fee = FeeModule::get_punishment_fee(amount_without_fee_dot).unwrap();
        assert!(punishment_fee > 0);

        SlaModule::set_vault_sla(&account_of(VAULT), FixedI128::from(80));
        // alice cancels redeem request and chooses to reimburse
        assert_ok!(Call::Redeem(RedeemCall::cancel_redeem(redeem_id, true)).dispatch(origin_of(account_of(USER))));

        assert_eq!(
            ParachainState::get(),
            ParachainState::default().with_changes(|user, vault, _, fee_pool| {
                // with sla of 80, vault gets slashed for 115%: 110 to user, 5 to fee pool

                fee_pool.balance += amount_without_fee_dot / 20;
                fee_pool.tokens += redeem.fee;

                vault.backing_collateral -= amount_without_fee_dot + punishment_fee + amount_without_fee_dot / 20;
                vault.free_tokens += redeem.amount_btc;

                user.free_balance += amount_without_fee_dot + punishment_fee;
                user.free_tokens -= amount_btc;
            })
        );
    });
}

#[test]
fn integration_test_redeem_polka_btc_cancel_reimburse_insufficient_collateral_for_polkabtc() {
    test_with(|| {
        let amount_btc = 10_000;

        // set collateral to the minimum amount required, such that the vault can not afford to both
        // reimburse and keep backing his current tokens
        let required_collateral =
            VaultRegistryModule::get_required_collateral_for_polkabtc(DEFAULT_VAULT_ISSUED).unwrap();
        CoreVaultData::force_to(
            VAULT,
            CoreVaultData {
                backing_collateral: required_collateral,
                ..CoreVaultData::vault(VAULT)
            },
        );
        let initial_state = ParachainState::get();

        let redeem_id = setup_cancelable_redeem(USER, VAULT, 100000000, amount_btc);
        let redeem = RedeemModule::get_open_redeem_request_from_id(&redeem_id).unwrap();
        let amount_without_fee_dot = ExchangeRateOracleModule::btc_to_dots(redeem.amount_btc).unwrap();

        let punishment_fee = FeeModule::get_punishment_fee(amount_without_fee_dot).unwrap();
        assert!(punishment_fee > 0);

        SlaModule::set_vault_sla(&account_of(VAULT), FixedI128::from(80));
        // alice cancels redeem request and chooses to reimburse
        assert_ok!(Call::Redeem(RedeemCall::cancel_redeem(redeem_id, true)).dispatch(origin_of(account_of(USER))));

        assert_eq!(
            ParachainState::get(),
            initial_state.with_changes(|user, vault, _, fee_pool| {
                // with sla of 80, vault gets slashed for 115%: 110 to user, 5 to fee pool

                fee_pool.balance += amount_without_fee_dot / 20;
                fee_pool.tokens += redeem.fee;

                vault.backing_collateral -= amount_without_fee_dot + punishment_fee + amount_without_fee_dot / 20;
                // vault free tokens does not change, and issued tokens is reduced
                vault.issued -= redeem.amount_btc;

                user.free_balance += amount_without_fee_dot + punishment_fee;
                user.free_tokens -= amount_btc;
            })
        );

        SystemModule::set_block_number(100000000);
        CoreVaultData::force_to(
            VAULT,
            CoreVaultData {
                backing_collateral: required_collateral + amount_btc * 2,
                ..CoreVaultData::vault(VAULT)
            },
        );
        let pre_minting_state = ParachainState::get();

        assert_ok!(Call::Redeem(RedeemCall::mint_tokens_for_reimbursed_redeem(redeem_id))
            .dispatch(origin_of(account_of(VAULT))));
        assert_eq!(
            ParachainState::get(),
            pre_minting_state.with_changes(|_user, vault, _, _fee_pool| {
                vault.issued += redeem.amount_btc;
                vault.free_tokens += redeem.amount_btc;
            })
        );
    });
}

#[test]
fn integration_test_redeem_polka_btc_cancel_no_reimburse() {
    test_with(|| {
        let amount_btc = 10_000;

        let redeem_id = setup_cancelable_redeem(USER, VAULT, 100000000, amount_btc);
        let redeem = RedeemModule::get_open_redeem_request_from_id(&redeem_id).unwrap();
        let amount_without_fee_dot = ExchangeRateOracleModule::btc_to_dots(redeem.amount_btc).unwrap();

        let punishment_fee = FeeModule::get_punishment_fee(amount_without_fee_dot).unwrap();
        assert!(punishment_fee > 0);

        SlaModule::set_vault_sla(&account_of(VAULT), FixedI128::from(80));
        // alice cancels redeem request and chooses not to reimburse
        assert_ok!(Call::Redeem(RedeemCall::cancel_redeem(redeem_id, false)).dispatch(origin_of(account_of(USER))));

        assert_eq!(
            ParachainState::get(),
            ParachainState::default().with_changes(|user, vault, _, fee_pool| {
                // with sla of 80, vault gets slashed for 15%: punishment of 10 to user, 5 to fee pool

                fee_pool.balance += amount_without_fee_dot / 20;

                vault.backing_collateral -= punishment_fee + amount_without_fee_dot / 20;

                user.free_balance += punishment_fee;
            })
        );
    });
}

#[test]
fn integration_test_redeem_polka_btc_cancel_liquidated_no_reimburse() {
    test_with(|| {
        let polka_btc = 1_000;
        let collateral_vault = 1_000_000;
        let redeem_id = setup_cancelable_redeem(USER, VAULT, collateral_vault, polka_btc);
        let redeem = RedeemModule::get_open_redeem_request_from_id(&redeem_id).unwrap();

        // setup vault state such that 1/4th of its collateral is freed after successful redeem
        CoreVaultData::force_to(
            VAULT,
            CoreVaultData {
                issued: redeem.amount_btc * 4,
                to_be_issued: 0,
                to_be_redeemed: redeem.amount_btc * 4,
                backing_collateral: collateral_vault,
                ..default_vault_state()
            },
        );

        drop_exchange_rate_and_liquidate(VAULT);

        let post_liquidation_state = ParachainState::get();

        assert_ok!(Call::Redeem(RedeemCall::cancel_redeem(redeem_id, false)).dispatch(origin_of(account_of(USER))));

        // NOTE: changes are relative the the post liquidation state
        assert_eq!(
            ParachainState::get(),
            post_liquidation_state.with_changes(|user, vault, liquidation_vault, _fee_pool| {
                // to-be-redeemed decreased, forwarding to liquidation vault
                vault.to_be_redeemed -= redeem.amount_btc;
                liquidation_vault.to_be_redeemed -= redeem.amount_btc;

                // the backing that remained with the vault to back this redeem is now transferred to the liquidation
                // vault
                let backing_for_this_redeem = collateral_vault / 4;
                vault.backing_collateral -= backing_for_this_redeem;
                liquidation_vault.backing_collateral += backing_for_this_redeem;

                // user's tokens get unlocked
                user.locked_tokens -= redeem.amount_btc + redeem.fee;
                user.free_tokens += redeem.amount_btc + redeem.fee;

                // Note that no punishment is taken from vault, because it's already liquidated
            })
        );
    });
}

#[test]
fn integration_test_redeem_polka_btc_cancel_liquidated_reimburse() {
    test_with(|| {
        let polka_btc = 1_000;
        let collateral_vault = 1_000_000;
        let redeem_id = setup_cancelable_redeem(USER, VAULT, collateral_vault, polka_btc);
        let redeem = RedeemModule::get_open_redeem_request_from_id(&redeem_id).unwrap();

        // setup vault state such that 1/4th of its collateral is freed after successful redeem
        CoreVaultData::force_to(
            VAULT,
            CoreVaultData {
                issued: redeem.amount_btc * 4,
                to_be_issued: 0,
                to_be_redeemed: redeem.amount_btc * 4,
                backing_collateral: collateral_vault,
                ..default_vault_state()
            },
        );

        drop_exchange_rate_and_liquidate(VAULT);

        let post_liquidation_state = ParachainState::get();

        assert_ok!(Call::Redeem(RedeemCall::cancel_redeem(redeem_id, true)).dispatch(origin_of(account_of(USER))));

        // NOTE: changes are relative the the post liquidation state
        assert_eq!(
            ParachainState::get(),
            post_liquidation_state.with_changes(|user, vault, liquidation_vault, fee_pool| {
                // to-be-redeemed decreased, forwarding to liquidation vault
                vault.to_be_redeemed -= redeem.amount_btc;
                liquidation_vault.to_be_redeemed -= redeem.amount_btc;

                // tokens are given to the vault, minus a fee that is given to the fee pool
                vault.free_tokens += redeem.amount_btc;
                fee_pool.tokens += redeem.fee;

                // the backing that remained with the vault to back this redeem is transferred to the user
                let backing_for_this_redeem = collateral_vault / 4;
                vault.backing_collateral -= backing_for_this_redeem;
                user.free_balance += backing_for_this_redeem;

                // user's tokens get burned
                user.locked_tokens -= polka_btc;

                // Note that no punishment is taken from vault, because it's already liquidated
            })
        );
    });
}

#[test]
fn integration_test_redeem_polka_btc_execute_liquidated() {
    test_with(|| {
        let polka_btc = 1_000;
        let fee = FeeModule::get_redeem_fee(polka_btc).unwrap();
        let collateral_vault = 1_000_000;
        let amount_without_fee = polka_btc - fee;

        let redeem_id = setup_redeem(polka_btc, USER, VAULT, collateral_vault);
        let redeem = RedeemModule::get_open_redeem_request_from_id(&redeem_id).unwrap();

        // setup vault state such that 1/4th of its collateral is freed after successful redeem
        CoreVaultData::force_to(
            VAULT,
            CoreVaultData {
                issued: amount_without_fee * 4,
                to_be_redeemed: amount_without_fee * 4,
                backing_collateral: collateral_vault,
                ..Default::default()
            },
        );

        drop_exchange_rate_and_liquidate(VAULT);

        let post_liquidation_state = ParachainState::get();

        execute_redeem(polka_btc, redeem_id);

        // NOTE: changes are relative the the post liquidation state
        assert_eq!(
            ParachainState::get(),
            post_liquidation_state.with_changes(|user, vault, liquidation_vault, fee_pool| {
                // fee given to fee pool
                fee_pool.tokens += redeem.fee;

                // polkabtc burned from user
                user.locked_tokens -= polka_btc;

                // to-be-redeemed & issued decreased, forwarding to liquidation vault
                vault.to_be_redeemed -= redeem.amount_btc;
                liquidation_vault.to_be_redeemed -= redeem.amount_btc;
                liquidation_vault.issued -= redeem.amount_btc;

                // collateral released
                let released_collateral = vault.backing_collateral / 4;
                vault.backing_collateral -= released_collateral;
                vault.free_balance += released_collateral;
            })
        );
    });
}

#[test]
fn integration_test_redeem_banning() {
    test_with(|| {
        let new_vault = CAROL;

        let redeem_id = setup_cancelable_redeem(USER, VAULT, 10_000, 1_000);

        // make sure the vault & user have funds after the cancel_redeem
        CoreVaultData::force_to(
            VAULT,
            CoreVaultData {
                issued: 1000000,
                backing_collateral: 10000000,
                free_balance: 100, // to be used for griefing collateral
                ..CoreVaultData::vault(VAULT)
            },
        );
        UserData::force_to(
            USER,
            UserData {
                free_balance: 1000000,
                free_tokens: 10000000,
                ..UserData::get(USER)
            },
        );
        CoreVaultData::force_to(
            new_vault,
            CoreVaultData {
                issued: 1000000,
                backing_collateral: 10000000,
                ..Default::default()
            },
        );

        // can still make a replace request now
        assert_ok!(Call::Replace(ReplaceCall::request_replace(100, 100)).dispatch(origin_of(account_of(VAULT))));
        let replace_id = SystemModule::events()
            .iter()
            .find_map(|r| match r.event {
                Event::replace(ReplaceEvent::RequestReplace(id, _, _, _)) => Some(id.clone()),
                _ => None,
            })
            .unwrap();

        // cancel the redeem, this should ban the vault
        cancel_redeem(redeem_id, USER, true);

        // can not redeem with vault while banned
        assert_noop!(
            Call::Redeem(RedeemCall::request_redeem(
                50,
                BtcAddress::P2PKH(H160([0u8; 20])),
                account_of(VAULT),
            ))
            .dispatch(origin_of(account_of(USER))),
            VaultRegistryError::VaultBanned,
        );

        // can not issue with vault while banned
        assert_noop!(
            Call::Issue(IssueCall::request_issue(50, account_of(VAULT), 50)).dispatch(origin_of(account_of(USER))),
            VaultRegistryError::VaultBanned,
        );

        // can not request replace while banned
        assert_noop!(
            Call::Replace(ReplaceCall::request_replace(0, 0)).dispatch(origin_of(account_of(VAULT))),
            VaultRegistryError::VaultBanned,
        );

        // can not accept replace of banned vault
        assert_noop!(
            Call::Replace(ReplaceCall::accept_replace(replace_id, 1000, BtcAddress::default()))
                .dispatch(origin_of(account_of(VAULT))),
            VaultRegistryError::VaultBanned,
        );

        // check that the ban is not permanent
        SystemModule::set_block_number(100000000);
        assert_ok!(
            Call::Issue(IssueCall::request_issue(50, account_of(VAULT), 50)).dispatch(origin_of(account_of(USER)))
        );
    })
}
