use crate::{Error, ACCEPTED_MAX_OPRETURN_TRANSACTION_OUTPUTS};
pub use bitcoin::Address as BtcAddress;
use bitcoin::{
    parser::FromLeBytes,
    types::{BlockHeader, H256Le, RawBlockHeader, Transaction},
};
use codec::{Decode, Encode};
use frame_support::{dispatch::DispatchError, ensure};
use sp_core::H256;
use sp_std::vec::Vec;

/// Bitcoin Enriched Block Headers
#[derive(Encode, Decode, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct RichBlockHeader<AccountId, BlockNumber> {
    pub block_hash: H256Le,
    pub block_header: BlockHeader,
    pub block_height: u32,
    pub chain_ref: u32,
    // required for fault attribution
    pub account_id: AccountId,
    pub para_height: BlockNumber,
}

impl<AccountId, BlockNumber> RichBlockHeader<AccountId, BlockNumber> {
    /// Creates a new RichBlockHeader
    ///
    /// # Arguments
    ///
    /// * `raw_block_header` - 80 byte raw Bitcoin block header
    /// * `chain_ref` - chain reference
    /// * `block_height` - chain height
    /// * `account_id` - submitter
    /// * `para_height` - height of the parachain at submission
    #[allow(dead_code)]
    pub fn new(
        raw_block_header: RawBlockHeader,
        chain_ref: u32,
        block_height: u32,
        account_id: AccountId,
        para_height: BlockNumber,
    ) -> Result<Self, bitcoin::Error> {
        Ok(RichBlockHeader {
            block_hash: raw_block_header.hash(),
            block_header: BlockHeader::from_le_bytes(raw_block_header.as_bytes())?,
            block_height,
            chain_ref,
            account_id,
            para_height,
        })
    }
}

#[cfg_attr(feature = "std", derive(Debug, PartialEq))]
pub struct OpReturnPaymentData<T: frame_system::Config> {
    pub op_return: H256,
    // vec of (amount, address)
    payments: Vec<(i64, BtcAddress)>,
    _marker: sp_std::marker::PhantomData<T>,
}

impl<T: crate::Config> OpReturnPaymentData<T> {
    pub fn try_from_transaction(transaction: Transaction) -> Result<Self, DispatchError> {
        // check the number of outputs - this check is redundant due to the checks below, but
        // this serves to put an upperbound to the number of iterations
        ensure!(
            transaction.outputs.len() <= ACCEPTED_MAX_OPRETURN_TRANSACTION_OUTPUTS as usize,
            Error::<T>::InvalidOpReturnTransaction
        );

        let mut payments = Vec::new();
        let mut op_returns = Vec::new();
        for tx in transaction.outputs {
            if let Ok(address) = tx.extract_address() {
                payments.push((tx.value, address));
            } else if let Ok(data) = tx.script.extract_op_return_data() {
                // make sure the amount is zero
                ensure!(tx.value == 0, Error::<T>::InvalidOpReturnTransaction);
                // make sure that the op_return is exactly 32 bytes
                ensure!(data.len() == 32, Error::<T>::InvalidOpReturnTransaction);
                op_returns.push(H256::from_slice(&data));
            } else {
                return Err(Error::<T>::InvalidOpReturnTransaction.into());
            }
        }

        // check we have exactly 1 op-return
        ensure!(op_returns.len() == 1, Error::<T>::InvalidOpReturnTransaction);

        // Check that we have either 1 payment, or 2 payments to different addresses. Enforcing the
        // payments to be unique helps to prevent the vault from paying more than is allowed
        match payments.len() {
            1 => (),
            2 => {
                // ensure that the addresses are not identical
                ensure!(payments[0].1 != payments[1].1, Error::<T>::InvalidOpReturnTransaction);
            }
            _ => return Err(Error::<T>::InvalidOpReturnTransaction.into()),
        }

        Ok(Self {
            op_return: op_returns.remove(0),
            payments,
            _marker: Default::default(),
        })
    }

    // ensures this is a valid payment. If it is, it returns the return-to-self address
    pub fn ensure_valid_payment_to(
        &self,
        expected_amount: i64,
        recipient: BtcAddress,
        op_return: Option<H256>,
    ) -> Result<Option<BtcAddress>, DispatchError> {
        // make sure the op_return matches
        if let Some(op_return) = op_return {
            ensure!(op_return == self.op_return, Error::<T>::InvalidPayment);
        }

        // ensure we have a correct payment to the recipient
        let paid_amount = self
            .payments
            .iter()
            .find_map(|&(amount, address)| if address == recipient { Some(amount) } else { None })
            .ok_or(Error::<T>::InvalidPayment)?;

        ensure!(paid_amount == expected_amount, Error::<T>::InvalidPaymentAmount);

        // return the return-to-self if it exists, otherwise None
        Ok(self
            .payments
            .iter()
            .find_map(|&(_, address)| if address != recipient { Some(address) } else { None }))
    }
}
