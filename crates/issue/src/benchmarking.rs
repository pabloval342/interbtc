use super::*;
use bitcoin::types::{
    BlockBuilder, TransactionBuilder, TransactionInputBuilder, TransactionInputSource, TransactionOutput,
};
use btc_relay::{BtcAddress, BtcPublicKey};
use currency::getters::{get_relay_chain_currency_id as get_collateral_currency_id, *};
use frame_benchmarking::v2::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use orml_traits::MultiCurrency;
use primitives::{CurrencyId, VaultId};
use sp_core::{H256, U256};
use sp_runtime::{traits::One, FixedPointNumber};
use sp_std::prelude::*;

// Pallets
use crate::Pallet as Issue;
use bitcoin::types::Block;
use btc_relay::Pallet as BtcRelay;
use oracle::Pallet as Oracle;
use security::Pallet as Security;
use vault_registry::Pallet as VaultRegistry;

fn deposit_tokens<T: crate::Config>(currency_id: CurrencyId, account_id: &T::AccountId, amount: BalanceOf<T>) {
    assert_ok!(<orml_tokens::Pallet<T>>::deposit(currency_id, account_id, amount));
}

fn mint_collateral<T: crate::Config>(account_id: &T::AccountId, amount: BalanceOf<T>) {
    deposit_tokens::<T>(get_collateral_currency_id::<T>(), account_id, amount);
    deposit_tokens::<T>(get_native_currency_id::<T>(), account_id, amount);
}

fn get_vault_id<T: crate::Config>() -> DefaultVaultId<T> {
    VaultId::new(
        account("Vault", 0, 0),
        get_collateral_currency_id::<T>(),
        get_wrapped_currency_id::<T>(),
    )
}

fn setup_chain<T: crate::Config>() {
    let dummy_vault = get_vault_id::<T>();

    Oracle::<T>::_set_exchange_rate(
        get_native_currency_id::<T>(), // for griefing collateral
        <T as currency::Config>::UnsignedFixedPoint::one(),
    )
    .unwrap();
    Oracle::<T>::_set_exchange_rate(
        dummy_vault.collateral_currency(),
        <T as currency::Config>::UnsignedFixedPoint::one(),
    )
    .unwrap();

    VaultRegistry::<T>::set_minimum_collateral(
        RawOrigin::Root.into(),
        dummy_vault.collateral_currency(),
        100_000u32.into(),
    )
    .unwrap();
    VaultRegistry::<T>::_set_system_collateral_ceiling(dummy_vault.currencies.clone(), 1_000_000_000u32.into());

    VaultRegistry::<T>::_set_secure_collateral_threshold(
        dummy_vault.currencies.clone(),
        <T as currency::Config>::UnsignedFixedPoint::checked_from_rational(1, 100000).unwrap(),
    );
    VaultRegistry::<T>::_set_premium_redeem_threshold(
        dummy_vault.currencies.clone(),
        <T as currency::Config>::UnsignedFixedPoint::checked_from_rational(1, 200000).unwrap(),
    );
    VaultRegistry::<T>::_set_liquidation_collateral_threshold(
        dummy_vault.currencies.clone(),
        <T as currency::Config>::UnsignedFixedPoint::checked_from_rational(1, 300000).unwrap(),
    );
}

fn register_vault<T: crate::Config>(vault_id: DefaultVaultId<T>) {
    let origin = RawOrigin::Signed(vault_id.account_id.clone());
    mint_collateral::<T>(&vault_id.account_id.clone(), (1u32 << 31).into());

    assert_ok!(VaultRegistry::<T>::register_public_key(
        origin.into(),
        BtcPublicKey::dummy()
    ));
    assert_ok!(VaultRegistry::<T>::_register_vault(
        vault_id.clone(),
        100000000u32.into()
    ));
}

fn expire_issue<T: crate::Config>(chain_state: &mut ChainState<T>) {
    let period = Issue::<T>::issue_period().max(chain_state.issue_request.period);
    let expiry_height = BtcRelay::<T>::bitcoin_expiry_height(chain_state.issue_request.btc_height, period).unwrap();
    Security::<T>::set_active_block_number(
        chain_state.issue_request.opentime + Issue::<T>::issue_period() + 100u32.into(),
    );

    mine_blocks::<T>(chain_state, expiry_height + 100);
}

fn mine_blocks<T: crate::Config>(chain_state: &mut ChainState<T>, end_height: u32) {
    let relayer_id: T::AccountId = account("Relayer", 0, 0);

    let transaction = TransactionBuilder::new()
        .with_version(2)
        .add_input(
            TransactionInputBuilder::new()
                .with_source(TransactionInputSource::FromOutput(
                    chain_state.newest_bitcoin_block.transactions[0].hash(),
                    0,
                ))
                .with_script(&[
                    0, 71, 48, 68, 2, 32, 91, 128, 41, 150, 96, 53, 187, 63, 230, 129, 53, 234, 210, 186, 21, 187, 98,
                    38, 255, 112, 30, 27, 228, 29, 132, 140, 155, 62, 123, 216, 232, 168, 2, 32, 72, 126, 179, 207,
                    142, 8, 99, 8, 32, 78, 244, 166, 106, 160, 207, 227, 61, 210, 172, 234, 234, 93, 59, 159, 79, 12,
                    194, 240, 212, 3, 120, 50, 1, 71, 81, 33, 3, 113, 209, 131, 177, 9, 29, 242, 229, 15, 217, 247,
                    165, 78, 111, 80, 79, 50, 200, 117, 80, 30, 233, 210, 167, 133, 175, 62, 253, 134, 127, 212, 51,
                    33, 2, 128, 200, 184, 235, 148, 25, 43, 34, 28, 173, 55, 54, 189, 164, 187, 243, 243, 152, 7, 84,
                    210, 85, 156, 238, 77, 97, 188, 240, 162, 197, 105, 62, 82, 174,
                ])
                .build(),
        )
        .build();

    let mut prev_hash = chain_state.newest_bitcoin_block.header.hash;
    for _ in 0..end_height {
        let block = BlockBuilder::new()
            .with_previous_hash(prev_hash)
            .with_version(4)
            .with_coinbase(&BtcAddress::dummy(), 50, 3)
            .with_timestamp(1588813835)
            .add_transaction(transaction.clone())
            .mine(U256::from(2).pow(254.into()))
            .unwrap();
        prev_hash = block.header.hash;

        BtcRelay::<T>::_store_block_header(&relayer_id, block.header).unwrap();
        chain_state.newest_bitcoin_block = block;
    }
}

enum PaymentType {
    Underpayment,
    Exact,
    Overpayment,
}

struct ChainState<T: Config> {
    issue_id: H256,
    merkle_proof: MerkleProof,
    transaction: Transaction,
    issue_request: DefaultIssueRequest<T>,
    newest_bitcoin_block: Block,
}

fn setup_issue<T: crate::Config>(payment: PaymentType) -> ChainState<T> {
    let origin: T::AccountId = account("Origin", 0, 0);
    let vault_id = get_vault_id::<T>();
    let relayer_id: T::AccountId = account("Relayer", 0, 0);

    mint_collateral::<T>(&origin, (1u32 << 31).into());
    mint_collateral::<T>(&relayer_id, (1u32 << 31).into());
    setup_chain::<T>();

    let vault_btc_address = BtcAddress::dummy();
    let value: Amount<T> = Amount::new(2u32.into(), get_wrapped_currency_id::<T>());

    let issue_id = H256::zero();
    let issue_request = IssueRequest {
        requester: origin.clone(),
        vault: vault_id.clone(),
        btc_address: vault_btc_address,
        amount: value.amount(),
        btc_height: Default::default(),
        btc_public_key: Default::default(),
        fee: Default::default(),
        griefing_collateral: Default::default(),
        opentime: Default::default(),
        period: Default::default(),
        status: Default::default(),
    };
    Issue::<T>::insert_issue_request(&issue_id, &issue_request);

    let height = 0;
    let block = BlockBuilder::new()
        .with_version(4)
        .with_coinbase(&vault_btc_address, 50, 3)
        .with_timestamp(1588813835)
        .mine(U256::from(2).pow(254.into()))
        .unwrap();

    let block_hash = block.header.hash;

    Security::<T>::set_active_block_number(1u32.into());
    BtcRelay::<T>::_initialize(relayer_id.clone(), block.header, height).unwrap();

    let transaction = TransactionBuilder::new()
        .with_version(2)
        .add_input(
            TransactionInputBuilder::new()
                .with_source(TransactionInputSource::FromOutput(block.transactions[0].hash(), 0))
                .with_script(&[
                    0, 71, 48, 68, 2, 32, 91, 128, 41, 150, 96, 53, 187, 63, 230, 129, 53, 234, 210, 186, 21, 187, 98,
                    38, 255, 112, 30, 27, 228, 29, 132, 140, 155, 62, 123, 216, 232, 168, 2, 32, 72, 126, 179, 207,
                    142, 8, 99, 8, 32, 78, 244, 166, 106, 160, 207, 227, 61, 210, 172, 234, 234, 93, 59, 159, 79, 12,
                    194, 240, 212, 3, 120, 50, 1, 71, 81, 33, 3, 113, 209, 131, 177, 9, 29, 242, 229, 15, 217, 247,
                    165, 78, 111, 80, 79, 50, 200, 117, 80, 30, 233, 210, 167, 133, 175, 62, 253, 134, 127, 212, 51,
                    33, 2, 128, 200, 184, 235, 148, 25, 43, 34, 28, 173, 55, 54, 189, 164, 187, 243, 243, 152, 7, 84,
                    210, 85, 156, 238, 77, 97, 188, 240, 162, 197, 105, 62, 82, 174,
                ])
                .build(),
        )
        .add_output(TransactionOutput::payment(
            match payment {
                PaymentType::Underpayment => 1u32.into(),
                PaymentType::Exact => 2u32.into(),
                PaymentType::Overpayment => 3u32.into(),
            },
            &vault_btc_address,
        ))
        .add_output(TransactionOutput::op_return(0, H256::zero().as_bytes()))
        .build();

    let block = BlockBuilder::new()
        .with_previous_hash(block_hash)
        .with_version(4)
        .with_coinbase(&vault_btc_address, 50, 4)
        .with_timestamp(1588813835)
        .add_transaction(transaction.clone())
        .mine(U256::from(2).pow(254.into()))
        .unwrap();

    let tx_id = transaction.tx_id();
    let merkle_proof = block.merkle_proof(&[tx_id]).unwrap();

    BtcRelay::<T>::_store_block_header(&relayer_id, block.header).unwrap();
    Security::<T>::set_active_block_number(
        Security::<T>::active_block_number() + BtcRelay::<T>::parachain_confirmations(),
    );

    register_vault::<T>(vault_id.clone());

    VaultRegistry::<T>::try_increase_to_be_issued_tokens(&vault_id, &value).unwrap();
    let secure_id = Security::<T>::get_secure_id(&vault_id.account_id);
    VaultRegistry::<T>::register_deposit_address(&vault_id, secure_id).unwrap();

    ChainState {
        issue_id,
        merkle_proof,
        transaction,
        issue_request,
        newest_bitcoin_block: block,
    }
}

#[benchmarks]
pub mod benchmarks {
    use super::*;

    #[benchmark]
    fn request_issue() {
        let origin: T::AccountId = account("Origin", 0, 0);
        let amount = Issue::<T>::issue_btc_dust_value(get_wrapped_currency_id::<T>()).amount() + 1000u32.into();
        let vault_id = get_vault_id::<T>();
        let relayer_id: T::AccountId = account("Relayer", 0, 0);

        mint_collateral::<T>(&origin, (1u32 << 31).into());
        mint_collateral::<T>(&relayer_id, (1u32 << 31).into());

        setup_chain::<T>();
        register_vault::<T>(vault_id.clone());

        // initialize relay

        let height = 0;
        let block = BlockBuilder::new()
            .with_version(4)
            .with_coinbase(&BtcAddress::dummy(), 50, 3)
            .with_timestamp(1588813835)
            .mine(U256::from(2).pow(254.into()))
            .unwrap();
        let block_hash = block.header.hash;

        Security::<T>::set_active_block_number(1u32.into());
        BtcRelay::<T>::_initialize(relayer_id.clone(), block.header, height).unwrap();

        let vault_btc_address = BtcAddress::dummy();

        let transaction = TransactionBuilder::new()
            .with_version(2)
            .add_input(
                TransactionInputBuilder::new()
                    .with_source(TransactionInputSource::FromOutput(block.transactions[0].hash(), 0))
                    .with_script(&[
                        0, 71, 48, 68, 2, 32, 91, 128, 41, 150, 96, 53, 187, 63, 230, 129, 53, 234, 210, 186, 21, 187,
                        98, 38, 255, 112, 30, 27, 228, 29, 132, 140, 155, 62, 123, 216, 232, 168, 2, 32, 72, 126, 179,
                        207, 142, 8, 99, 8, 32, 78, 244, 166, 106, 160, 207, 227, 61, 210, 172, 234, 234, 93, 59, 159,
                        79, 12, 194, 240, 212, 3, 120, 50, 1, 71, 81, 33, 3, 113, 209, 131, 177, 9, 29, 242, 229, 15,
                        217, 247, 165, 78, 111, 80, 79, 50, 200, 117, 80, 30, 233, 210, 167, 133, 175, 62, 253, 134,
                        127, 212, 51, 33, 2, 128, 200, 184, 235, 148, 25, 43, 34, 28, 173, 55, 54, 189, 164, 187, 243,
                        243, 152, 7, 84, 210, 85, 156, 238, 77, 97, 188, 240, 162, 197, 105, 62, 82, 174,
                    ])
                    .build(),
            )
            .add_output(TransactionOutput::payment(123123, &vault_btc_address))
            .add_output(TransactionOutput::op_return(0, H256::zero().as_bytes()))
            .build();

        let block = BlockBuilder::new()
            .with_previous_hash(block_hash)
            .with_version(4)
            .with_timestamp(1588813835)
            .add_transaction(transaction)
            .mine(U256::from(2).pow(254.into()))
            .unwrap();

        BtcRelay::<T>::_store_block_header(&relayer_id, block.header).unwrap();
        Security::<T>::set_active_block_number(
            Security::<T>::active_block_number() + BtcRelay::<T>::parachain_confirmations(),
        );

        #[extrinsic_call]
        request_issue(RawOrigin::Signed(origin), amount, vault_id);
    }

    #[benchmark]
    fn execute_issue_exact() {
        let origin: T::AccountId = account("Origin", 0, 0);
        let issue_data = setup_issue::<T>(PaymentType::Exact);

        #[extrinsic_call]
        execute_issue(
            RawOrigin::Signed(origin),
            issue_data.issue_id,
            issue_data.merkle_proof,
            issue_data.transaction,
        );
    }

    #[benchmark]
    fn execute_issue_overpayment() {
        let origin: T::AccountId = account("Origin", 0, 0);
        let issue_data = setup_issue::<T>(PaymentType::Overpayment);

        #[extrinsic_call]
        execute_issue(
            RawOrigin::Signed(origin),
            issue_data.issue_id,
            issue_data.merkle_proof,
            issue_data.transaction,
        );
    }

    #[benchmark]
    fn execute_issue_underpayment() {
        let origin: T::AccountId = account("Origin", 0, 0);
        let issue_data = setup_issue::<T>(PaymentType::Underpayment);

        #[extrinsic_call]
        execute_issue(
            RawOrigin::Signed(origin),
            issue_data.issue_id,
            issue_data.merkle_proof,
            issue_data.transaction,
        );
    }

    #[benchmark]
    fn execute_expired_issue_exact() {
        let origin: T::AccountId = account("Origin", 0, 0);
        let mut issue_data = setup_issue::<T>(PaymentType::Exact);
        expire_issue::<T>(&mut issue_data);

        #[extrinsic_call]
        execute_issue(
            RawOrigin::Signed(origin),
            issue_data.issue_id,
            issue_data.merkle_proof,
            issue_data.transaction,
        );
    }

    #[benchmark]
    fn execute_expired_issue_overpayment() {
        let origin: T::AccountId = account("Origin", 0, 0);
        let mut issue_data = setup_issue::<T>(PaymentType::Overpayment);
        expire_issue::<T>(&mut issue_data);

        #[extrinsic_call]
        execute_issue(
            RawOrigin::Signed(origin),
            issue_data.issue_id,
            issue_data.merkle_proof,
            issue_data.transaction,
        );
    }

    #[benchmark]
    fn execute_expired_issue_underpayment() {
        let origin: T::AccountId = account("Origin", 0, 0);
        let mut issue_data = setup_issue::<T>(PaymentType::Underpayment);
        expire_issue::<T>(&mut issue_data);

        #[extrinsic_call]
        execute_issue(
            RawOrigin::Signed(origin),
            issue_data.issue_id,
            issue_data.merkle_proof,
            issue_data.transaction,
        );
    }

    #[benchmark]
    fn cancel_issue() {
        let origin: T::AccountId = account("Origin", 0, 0);

        let mut issue_data = setup_issue::<T>(PaymentType::Exact);
        expire_issue::<T>(&mut issue_data);

        #[extrinsic_call]
        cancel_issue(RawOrigin::Signed(origin), issue_data.issue_id);
    }

    #[benchmark]
    fn set_issue_period() {
        #[extrinsic_call]
        set_issue_period(RawOrigin::Root, 1u32.into());
    }

    impl_benchmark_test_suite! {
        Issue,
        crate::mock::ExtBuilder::build_with(Default::default()),
        crate::mock::Test
    }
}
