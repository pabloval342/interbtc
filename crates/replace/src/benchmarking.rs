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
use primitives::VaultId;
use sp_core::{H256, U256};
use sp_runtime::{traits::One, FixedPointNumber};
use sp_std::prelude::*;

// Pallets
use crate::Pallet as Replace;
use btc_relay::Pallet as BtcRelay;
use oracle::Pallet as Oracle;
use security::Pallet as Security;
use vault_registry::Pallet as VaultRegistry;

struct Payment {
    amount: i64,
    output_address: BtcAddress,
    op_return: H256,
}

fn mine_blocks<T: crate::Config>(end_height: u32, tx: Option<Payment>) -> (Transaction, MerkleProof) {
    let relayer_id: T::AccountId = account("Relayer", 0, 0);

    assert_ok!(<orml_tokens::Pallet<T>>::deposit(
        get_collateral_currency_id::<T>(),
        &relayer_id,
        (1u32 << 31).into()
    ));
    assert_ok!(<orml_tokens::Pallet<T>>::deposit(
        get_native_currency_id::<T>(),
        &relayer_id,
        (1u32 << 31).into()
    ));

    let height = 0;
    let block = BlockBuilder::new()
        .with_version(4)
        .with_coinbase(&BtcAddress::dummy(), 50, 3)
        .with_timestamp(1588813835)
        .mine(U256::from(2).pow(254.into()))
        .unwrap();

    Security::<T>::set_active_block_number(1u32.into());
    BtcRelay::<T>::_initialize(relayer_id.clone(), block.header, height).unwrap();

    let mut builder = TransactionBuilder::new();
    builder.with_version(2).add_input(
        TransactionInputBuilder::new()
            .with_source(TransactionInputSource::FromOutput(block.transactions[0].hash(), 0))
            .with_script(&[
                0, 71, 48, 68, 2, 32, 91, 128, 41, 150, 96, 53, 187, 63, 230, 129, 53, 234, 210, 186, 21, 187, 98, 38,
                255, 112, 30, 27, 228, 29, 132, 140, 155, 62, 123, 216, 232, 168, 2, 32, 72, 126, 179, 207, 142, 8, 99,
                8, 32, 78, 244, 166, 106, 160, 207, 227, 61, 210, 172, 234, 234, 93, 59, 159, 79, 12, 194, 240, 212, 3,
                120, 50, 1, 71, 81, 33, 3, 113, 209, 131, 177, 9, 29, 242, 229, 15, 217, 247, 165, 78, 111, 80, 79, 50,
                200, 117, 80, 30, 233, 210, 167, 133, 175, 62, 253, 134, 127, 212, 51, 33, 2, 128, 200, 184, 235, 148,
                25, 43, 34, 28, 173, 55, 54, 189, 164, 187, 243, 243, 152, 7, 84, 210, 85, 156, 238, 77, 97, 188, 240,
                162, 197, 105, 62, 82, 174,
            ])
            .build(),
    );
    if let Some(tx) = tx {
        builder.add_output(TransactionOutput::payment(tx.amount as i64, &tx.output_address));
        builder.add_output(TransactionOutput::op_return(0, &tx.op_return.as_bytes()));
    }

    let transaction = builder.build();

    let mut ret = None;

    let mut prev_hash = block.header.hash;
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

        if let None = ret {
            let tx_id = transaction.tx_id();
            let merkle_proof = block.merkle_proof(&[tx_id]).unwrap();

            ret = Some((transaction.clone(), merkle_proof));
        }

        BtcRelay::<T>::_store_block_header(&relayer_id, block.header).unwrap();
    }
    ret.unwrap()
}

fn test_request<T: crate::Config>(
    new_vault_id: &DefaultVaultId<T>,
    old_vault_id: &DefaultVaultId<T>,
) -> DefaultReplaceRequest<T> {
    ReplaceRequest {
        new_vault: new_vault_id.clone(),
        old_vault: old_vault_id.clone(),
        period: Default::default(),
        accept_time: Default::default(),
        amount: Default::default(),
        griefing_collateral: 12345u32.into(), // non-zero to hit additional code paths
        btc_address: Default::default(),
        collateral: Default::default(),
        btc_height: Default::default(),
        status: Default::default(),
    }
}

fn get_vault_id<T: crate::Config>(name: &'static str) -> DefaultVaultId<T> {
    VaultId::new(
        account(name, 0, 0),
        get_collateral_currency_id::<T>(),
        get_wrapped_currency_id::<T>(),
    )
}

fn register_vault<T: crate::Config>(vault_id: &DefaultVaultId<T>, issued_tokens: Amount<T>, to_be_replaced: Amount<T>) {
    let origin = RawOrigin::Signed(vault_id.account_id.clone());

    assert_ok!(<orml_tokens::Pallet<T>>::deposit(
        get_collateral_currency_id::<T>(),
        &vault_id.account_id,
        (1u32 << 31).into()
    ));
    assert_ok!(<orml_tokens::Pallet<T>>::deposit(
        get_native_currency_id::<T>(),
        &vault_id.account_id,
        (1u32 << 31).into()
    ));

    assert_ok!(VaultRegistry::<T>::register_public_key(
        origin.into(),
        BtcPublicKey::dummy()
    ));
    assert_ok!(VaultRegistry::<T>::_register_vault(
        vault_id.clone(),
        100000000u32.into()
    ));

    VaultRegistry::<T>::try_increase_to_be_issued_tokens(vault_id, &issued_tokens).unwrap();
    VaultRegistry::<T>::issue_tokens(vault_id, &issued_tokens).unwrap();
    VaultRegistry::<T>::try_increase_to_be_replaced_tokens(vault_id, &to_be_replaced).unwrap();
}

struct ChainState<T: Config> {
    old_vault_id: DefaultVaultId<T>,
    new_vault_id: DefaultVaultId<T>,
    issued_tokens: Amount<T>,
    to_be_replaced: Amount<T>,
}

fn setup_chain<T: crate::Config>() -> ChainState<T> {
    let new_vault_id = get_vault_id::<T>("NewVault");
    let old_vault_id = get_vault_id::<T>("OldVault");

    Oracle::<T>::_set_exchange_rate(
        get_native_currency_id::<T>(), // for griefing collateral
        <T as currency::Config>::UnsignedFixedPoint::one(),
    )
    .unwrap();
    Oracle::<T>::_set_exchange_rate(
        old_vault_id.collateral_currency(),
        <T as currency::Config>::UnsignedFixedPoint::one(),
    )
    .unwrap();

    VaultRegistry::<T>::set_minimum_collateral(
        RawOrigin::Root.into(),
        old_vault_id.collateral_currency(),
        100_000u32.into(),
    )
    .unwrap();
    VaultRegistry::<T>::_set_system_collateral_ceiling(old_vault_id.currencies.clone(), 1_000_000_000u32.into());

    VaultRegistry::<T>::_set_secure_collateral_threshold(
        old_vault_id.currencies.clone(),
        <T as currency::Config>::UnsignedFixedPoint::checked_from_rational(1, 100000).unwrap(),
    );
    VaultRegistry::<T>::_set_premium_redeem_threshold(
        old_vault_id.currencies.clone(),
        <T as currency::Config>::UnsignedFixedPoint::checked_from_rational(1, 200000).unwrap(),
    );
    VaultRegistry::<T>::_set_liquidation_collateral_threshold(
        old_vault_id.currencies.clone(),
        <T as currency::Config>::UnsignedFixedPoint::checked_from_rational(1, 300000).unwrap(),
    );

    let issued_tokens = Amount::new(200000u32.into(), old_vault_id.wrapped_currency());
    let to_be_replaced = issued_tokens.clone().map(|x| x / 4u32.into());

    register_vault(&old_vault_id, issued_tokens.clone(), to_be_replaced.clone());
    register_vault(&new_vault_id, issued_tokens.clone(), to_be_replaced.clone());

    ChainState {
        old_vault_id,
        new_vault_id,
        issued_tokens,
        to_be_replaced,
    }
}

fn setup_replace<T: crate::Config>(
    old_vault_id: &DefaultVaultId<T>,
    new_vault_id: &DefaultVaultId<T>,
    to_be_replaced: Amount<T>,
) -> (H256, MerkleProof, Transaction) {
    let replace_id = H256::zero();
    let mut replace_request = test_request::<T>(&new_vault_id, &old_vault_id);
    replace_request.amount = to_be_replaced.amount();
    Replace::<T>::insert_replace_request(&replace_id, &replace_request);

    let payment = Payment {
        amount: to_be_replaced.amount().try_into().unwrap_or_default(),
        output_address: replace_request.btc_address,
        op_return: replace_id,
    };

    // simulate that the request has been accepted
    VaultRegistry::<T>::try_increase_to_be_redeemed_tokens(&old_vault_id, &to_be_replaced).unwrap();
    VaultRegistry::<T>::try_increase_to_be_issued_tokens(&new_vault_id, &to_be_replaced).unwrap();

    VaultRegistry::<T>::transfer_funds(
        CurrencySource::FreeBalance(old_vault_id.account_id.clone()),
        CurrencySource::ActiveReplaceCollateral(old_vault_id.clone()),
        &Amount::new(replace_request.griefing_collateral, get_native_currency_id::<T>()),
    )
    .unwrap();

    let period = Replace::<T>::replace_period().max(replace_request.period);
    let expiry_height = BtcRelay::<T>::bitcoin_expiry_height(replace_request.btc_height, period).unwrap();
    let (transaction, merkle_proof) = mine_blocks::<T>(expiry_height + 100, Some(payment));

    Security::<T>::set_active_block_number(
        Security::<T>::active_block_number() + Replace::<T>::replace_period() + 100u32.into(),
    );

    (replace_id, merkle_proof, transaction)
}
#[benchmarks]
pub mod benchmarks {
    use super::*;

    #[benchmark]
    fn request_replace() {
        let ChainState {
            old_vault_id,
            issued_tokens,
            to_be_replaced,
            ..
        } = setup_chain::<T>();

        let amount = (issued_tokens.checked_sub(&to_be_replaced).unwrap()).amount();

        #[extrinsic_call]
        request_replace(
            RawOrigin::Signed(old_vault_id.account_id.clone()),
            old_vault_id.currencies.clone(),
            amount,
        );
    }

    #[benchmark]
    fn withdraw_replace() {
        let ChainState {
            old_vault_id,
            to_be_replaced,
            ..
        } = setup_chain::<T>();

        #[extrinsic_call]
        withdraw_replace(
            RawOrigin::Signed(old_vault_id.account_id.clone()),
            old_vault_id.currencies.clone(),
            to_be_replaced.amount(),
        );
    }

    #[benchmark]
    fn accept_replace() {
        let ChainState {
            old_vault_id,
            new_vault_id,
            to_be_replaced,
            ..
        } = setup_chain::<T>();

        let replace_id = H256::zero();
        let mut replace_request = test_request::<T>(&new_vault_id, &old_vault_id);
        replace_request.amount = to_be_replaced.amount();
        Replace::<T>::insert_replace_request(&replace_id, &replace_request);

        let new_vault_btc_address = BtcAddress::dummy();
        let griefing = 100000000u32.into();

        #[extrinsic_call]
        accept_replace(
            RawOrigin::Signed(new_vault_id.account_id.clone()),
            new_vault_id.currencies.clone(),
            old_vault_id,
            to_be_replaced.amount(),
            griefing,
            new_vault_btc_address,
        );
    }

    #[benchmark]
    fn execute_pending_replace() {
        let ChainState {
            old_vault_id,
            new_vault_id,
            to_be_replaced,
            ..
        } = setup_chain::<T>();
        let (replace_id, merkle_proof, transaction) = setup_replace::<T>(&old_vault_id, &new_vault_id, to_be_replaced);

        #[extrinsic_call]
        execute_replace(
            RawOrigin::Signed(old_vault_id.account_id),
            replace_id,
            merkle_proof,
            transaction,
        );
    }

    #[benchmark]
    fn execute_cancelled_replace() {
        let ChainState {
            old_vault_id,
            new_vault_id,
            to_be_replaced,
            ..
        } = setup_chain::<T>();
        let (replace_id, merkle_proof, transaction) = setup_replace::<T>(&old_vault_id, &new_vault_id, to_be_replaced);

        assert_ok!(Pallet::<T>::cancel_replace(
            RawOrigin::Signed(new_vault_id.account_id).into(),
            replace_id
        ));

        #[extrinsic_call]
        execute_replace(
            RawOrigin::Signed(old_vault_id.account_id),
            replace_id,
            merkle_proof,
            transaction,
        );
    }

    #[benchmark]
    fn cancel_replace() {
        let ChainState {
            old_vault_id,
            new_vault_id,
            to_be_replaced,
            ..
        } = setup_chain::<T>();

        let (replace_id, _, _) = setup_replace::<T>(&old_vault_id, &new_vault_id, to_be_replaced);

        #[extrinsic_call]
        cancel_replace(RawOrigin::Signed(new_vault_id.account_id), replace_id);
    }

    #[benchmark]
    fn set_replace_period() {
        #[extrinsic_call]
        set_replace_period(RawOrigin::Root, 1u32.into());
    }

    impl_benchmark_test_suite! {
        Replace,
        crate::mock::ExtBuilder::build_with(Default::default()),
        crate::mock::Test
    }
}
