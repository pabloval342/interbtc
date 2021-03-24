use super::*;
use crate::Module as BtcRelay;
use bitcoin::{
    formatter::{Formattable, TryFormattable},
    types::{
        Block, BlockBuilder, RawBlockHeader, Transaction, TransactionBuilder, TransactionInputBuilder,
        TransactionOutput,
    },
};
use frame_benchmarking::{account, benchmarks};
use frame_system::{Module as System, RawOrigin};
use sp_core::{H256, U256};
use sp_std::prelude::*;

fn mine_genesis<T: Config>(account_id: T::AccountId, address: &BtcAddress, height: u32) -> Block {
    let block = BlockBuilder::new()
        .with_version(2)
        .with_coinbase(address, 50, 3)
        .with_timestamp(1588813835)
        .mine(U256::from(2).pow(254.into()))
        .unwrap();

    let block_header = RawBlockHeader::from_bytes(&block.header.try_format().unwrap()).unwrap();
    BtcRelay::<T>::initialize(account_id, block_header, height).unwrap();

    block
}

fn mine_block_with_one_tx<T: Config>(
    account_id: T::AccountId,
    prev: Block,
    address: &BtcAddress,
    value: i32,
    op_return: &[u8],
) -> (Block, Transaction) {
    let prev_block_hash = prev.header.hash().unwrap();

    let transaction = TransactionBuilder::new()
        .with_version(2)
        .add_input(
            TransactionInputBuilder::new()
                .with_coinbase(false)
                .with_previous_hash(prev.transactions[0].hash())
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
        .add_output(TransactionOutput::payment(value.into(), address))
        .add_output(TransactionOutput::op_return(0, op_return))
        .build();

    let block = BlockBuilder::new()
        .with_previous_hash(prev_block_hash)
        .with_version(2)
        .with_coinbase(address, 50, 3)
        .with_timestamp(1588813835)
        .add_transaction(transaction.clone())
        .mine(U256::from(2).pow(254.into()))
        .unwrap();

    let block_header = RawBlockHeader::from_bytes(&block.header.try_format().unwrap()).unwrap();
    BtcRelay::<T>::_store_block_header(&account_id, block_header).unwrap();

    (block, transaction)
}

benchmarks! {
    verify_and_validate_transaction {
        let origin: T::AccountId = account("Origin", 0, 0);

        let address = BtcAddress::P2PKH(H160::from([0; 20]));

        let height = 0;
        let block = mine_genesis::<T>(origin.clone(), &address, height);

        let value = 0;
        let op_return = H256::zero().as_bytes().to_vec();
        let (block, transaction) = mine_block_with_one_tx::<T>(origin.clone(), block, &address, value, &op_return);

        let tx_id = transaction.tx_id();
        let proof = block.merkle_proof(&vec![tx_id]).unwrap().try_format().unwrap();
        let raw_tx = transaction.format_with(true);

        System::<T>::set_block_number(100u32.into());

    }: _(RawOrigin::Signed(origin), tx_id, proof, Some(0), raw_tx, value.into(), address, Some(op_return))

    verify_transaction_inclusion {
        let origin: T::AccountId = account("Origin", 0, 0);

        let address = BtcAddress::P2PKH(H160::from([0; 20]));

        let height = 0;
        let block = mine_genesis::<T>(origin.clone(), &address, height);

        let value = 0;
        let op_return = H256::zero().as_bytes().to_vec();
        let (block, transaction) = mine_block_with_one_tx::<T>(origin.clone(), block, &address, value, &op_return);

        let tx_id = transaction.tx_id();
        let tx_block_height = height;
        let proof = block.merkle_proof(&vec![tx_id]).unwrap().try_format().unwrap();

        System::<T>::set_block_number(100u32.into());

    }: _(RawOrigin::Signed(origin), tx_id, proof, Some(0))

    validate_transaction {
        let origin: T::AccountId = account("Origin", 0, 0);

        let address = BtcAddress::P2PKH(H160::from([0; 20]));
        let value = 0;
        let op_return = H256::zero().as_bytes().to_vec();

        let block = mine_genesis::<T>(origin.clone(), &address, 0);
        let (_, transaction) = mine_block_with_one_tx::<T>(origin.clone(), block, &address, value, &op_return);

        let raw_tx = transaction.format_with(true);

    }: _(RawOrigin::Signed(origin), raw_tx, value.into(), address, Some(op_return))

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{ExtBuilder, Test};
    use frame_support::assert_ok;

    #[test]
    fn test_benchmarks() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_verify_and_validate_transaction::<Test>());
            assert_ok!(test_benchmark_verify_transaction_inclusion::<Test>());
            assert_ok!(test_benchmark_validate_transaction::<Test>());
        });
    }
}
