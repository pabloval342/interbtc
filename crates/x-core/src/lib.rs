#![cfg_attr(not(feature = "std"), no_std)]
#![deny(warnings)]

use codec::alloc::string::{String, ToString};
use frame_support::dispatch::DispatchError;
use sp_std::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    // ----------
    // BTC Errors
    // ----------
    AlreadyInitialized,
    MissingBlockHeight, //not in spec
    InvalidHeaderSize,
    DuplicateBlock,
    PrevBlock, // TODO: rename to self-explanatory
    InvalidChainID,
    LowDiff,
    DiffTargetHeader, // TODO: rename to self-explanatory
    MalformedTxid,
    Confirmations,                   // TODO: rename to self-explanatory
    InsufficientStableConfirmations, //not in spec
    OngoingFork,                     //not in spec
    /// Format of the proof is not correct
    MalformedMerkleProof, // not in the spec
    /// Format of the proof is correct but does not yield the correct merkle root
    InvalidMerkleProof,
    NoData,
    Invalid,
    Shutdown,
    InvalidTxid,
    InsufficientValue,
    MalformedTransaction, // rename ERR_TX_FORMAT
    WrongRecipient,
    InvalidOutputFormat, // not in spec
    InvalidOpreturn,
    InvalidTxVersion,
    NotOpReturn,
    UnknownErrorcode,     // not in spec
    ForkIdNotFound,       // not in spec
    BlockNotFound,        // not in spec
    AlreadyReported,      // not in spec
    UnauthorizedRelayer,  // not in spec
    ChainCounterOverflow, // not in spec
    BlockHeightOverflow,  // not in spec
    ChainsUnderflow,      // not in spec
    /// Reached EOS without finishing to parse bytes
    EOS,
    /// Malformed header
    MalformedHeader,
    /// Format of the BIP141 witness transaction output is invalid
    MalformedWitnessOutput,
    // Format of the P2PKH transaction output is invalid
    MalformedP2PKHOutput,
    // Format of the P2SH transaction output is invalid
    MalformedP2SHOutput,
    /// Format of the OP_RETURN transaction output is invalid
    MalformedOpReturnOutput,
    // Output does not match format of supported output types (Witness, P2PKH, P2SH)
    UnsupportedOutputFormat,
    // Input does not match format of supported input types (Witness, P2PKH, P2SH)
    UnsupportedInputFormat,
    /// There are no NO_DATA blocks in this BlockChain
    NoDataEmpty, // not in spec
    // -------------
    // XClaim Errors
    // -------------
    CancelAcceptedRequest,
    InvalidReplaceID,
    ReplacePeriodExpired,
    UnauthorizedVault,
    ReplacePeriodNotExpired,
    InsufficientTokensComitted,
    InvalidVaultID,
    InvalidAmount,
    InvalidTimeout,
    MissingExchangeRate,
    InvalidOracleSource,
    InsufficientFunds,
    InsufficientLockedFunds,
    InsufficientCollateralAvailable,
    VaultNotFound,
    VaultBanned,
    VaultOverAuctionThreshold,
    CollateralBelowSecureThreshold,
    /// Returned if the collateral amount to register a vault was too low
    InsuficientVaultCollateralAmount,
    // FIXME: ERR_MIN_AMOUNT in spec
    /// Returned if a vault tries to register while already being registered
    VaultAlreadyRegistered,
    InsufficientCollateral,
    ExceedingVaultLimit,
    IssueIdNotFound,
    CommitPeriodExpired,
    UnauthorizedUser,
    TimeNotExpired,
    IssueCompleted,
    InsufficientTokensCommitted,
    AmountExceedsUserBalance,
    AmountExceedsVaultBalance,
    RedeemIdNotFound,
    RedeemPeriodExpired,
    RedeemPeriodNotExpired,

    /// Parachain Status Errors (Security module)
    ParachainNotRunning,
    ParachainShutdown,
    ParachainNotRunningOrLiquidation,
    ParachainOracleOfflineError,
    ParachainLiquidationError,
    /// use only for errors which means something
    /// going very wrong and which do not match any other error
    RuntimeError,
}

impl Error {
    pub fn message(self) -> &'static str {
        match self {
            Error::AlreadyInitialized => "Already initialized",
            Error::MissingBlockHeight => "Missing the block at this height",
            Error::InvalidHeaderSize => "Invalid block header size",
            Error::DuplicateBlock => "Block already stored",
            Error::PrevBlock => "Previous block hash not found",
            Error::InvalidChainID => "Invalid chain ID",
            Error::LowDiff => "PoW hash does not meet difficulty target of header",
            Error::DiffTargetHeader => "Incorrect difficulty target specified in block header",
            Error::MalformedTxid => "Malformed transaction identifier",
            Error::Confirmations => "Transaction has less confirmations than requested",
            Error::InsufficientStableConfirmations => "Transaction has less confirmations than the global STABLE_TRANSACTION_CONFIRMATIONS parameter",
            Error::OngoingFork => "Current fork ongoing",
            Error::MalformedMerkleProof => "Merkle proof is malformed",
            Error::InvalidMerkleProof => "Invalid Merkle Proof",
            Error::NoData => "Feature disabled. Reason: a main chain block with a lower height is flagged with NO_DATA.",
            Error::Invalid => "Feature disabled. Reason: a main chain block is flagged as INVALID.",
            Error::Shutdown => "BTC Parachain has shut down",
            Error::InvalidTxid => "Transaction hash does not match given txid",
            Error::InsufficientValue => "Value of payment below requested amount",
            Error::MalformedTransaction => "Transaction has incorrect format",
            Error::WrongRecipient => "Incorrect recipient Bitcoin address",
            Error::InvalidOutputFormat => "Incorrect transaction output format",
            Error::InvalidOpreturn => "Incorrect identifier in OP_RETURN field",
            Error::InvalidTxVersion => "Invalid transaction version",
            Error::NotOpReturn => "Expecting OP_RETURN output, but got another type",
            Error::UnknownErrorcode => "Error code not applicable to blocks",
            Error::ForkIdNotFound => "Blockchain with requested ID not found",
            Error::BlockNotFound => "Block header not found for given hash",
            Error::AlreadyReported => "Error code already reported",
            Error::UnauthorizedRelayer => "Unauthorized staked relayer",
            Error::ChainCounterOverflow => "Overflow of chain counter",
            Error::BlockHeightOverflow => "Overflow of block height",
            Error::ChainsUnderflow => "Underflow of stored blockchains counter",
            Error::EOS => "EOS reached while parsing",
            Error::MalformedHeader => "Format of the header is invalid",
            Error::MalformedWitnessOutput => "Format of the witness output is invalid",
            Error::MalformedP2PKHOutput => "Format of the P2PKH output is invalid",
            Error::MalformedP2SHOutput => "Format of the P2SH output is invalid",
            Error::MalformedOpReturnOutput => "Format of the OP_RETURN transaction output is invalid",
            Error::UnsupportedOutputFormat => "Unsupported output format. Currently supported: Witness, P2PKH, P2SH,",
            Error::UnsupportedInputFormat => "Unsupported input format. Currently supported: Witness, P2PKH, P2SH",
            Error::NoDataEmpty => "There are no NO_DATA blocks in this BlockChain.",

            Error::ReplacePeriodExpired => "Replace period expired",
            Error::ReplacePeriodNotExpired => "Replace period not expired",
            Error::InsufficientTokensComitted => "Insufficient tokens comitted",
            Error::InvalidVaultID => "Invalid vault ID",
            Error::InvalidAmount => "Invalid amount",
            Error::InvalidTimeout => "Invalid timeout",
            Error::InvalidReplaceID => "Invalid request ID",
            Error::CancelAcceptedRequest => "Cannot cancel an already accepted request",
            Error::MissingExchangeRate => "Exchange rate not set",
            Error::InvalidOracleSource => "Invalid oracle account",
            Error::InsufficientFunds => {
                "The balance of this account is insufficient to complete the transaction."
            }
            Error::InsufficientLockedFunds => {
                "The locked token balance of this account is insufficient to burn the tokens."
            }
            Error::InsufficientCollateralAvailable => {
                "The sender’s collateral balance is below the requested amount."
            }
            Error::VaultNotFound => "There exists no Vault with the given account id",
            Error::VaultBanned => "The selected Vault has been temporarily banned",
            Error::VaultOverAuctionThreshold => "Vault over auction threshold",
            Error::CollateralBelowSecureThreshold => "Collateral below secure threshold",
            Error::InsuficientVaultCollateralAmount => "The provided collateral was insufficient",
            Error::VaultAlreadyRegistered => "This vault is already registered",
            Error::InsufficientCollateral => "User provided collateral below limit",
            Error::ExceedingVaultLimit => "The requested Vault has not locked enough collateral",
            Error::IssueIdNotFound => "Requested issue id not found",
            Error::CommitPeriodExpired => "Time to issue PolkaBTC expired",
            Error::UnauthorizedUser => "Unauthorized: Caller must be associated user",
            Error::TimeNotExpired => "Time to issue PolkaBTC not yet expired",
            Error::IssueCompleted => "Issue completed and cannot be cancelled",
            Error::InsufficientTokensCommitted => "The requested amount of tokens exceeds the toBeIssuedTokens by this vault.",
            Error::AmountExceedsUserBalance => "The requested amount exceeds the user’s balance.",
            Error::AmountExceedsVaultBalance => "The requested amount exceeds the vault’s balance.",
            Error::RedeemIdNotFound => "The redeemId cannot be found.",
            Error::RedeemPeriodExpired => "The redeem period expired.",
            Error::UnauthorizedVault => "Unauthorized: Caller must be associated vault.",
            Error::RedeemPeriodNotExpired => "The period to complete the redeem request is not yet expired.",


            Error::ParachainNotRunning => "Function disabled. Reason: the Parachain status is not 'RUNNING'.",
            Error::ParachainShutdown => "Function disabled. Reason: the Parachain is shutdown",
            Error::ParachainNotRunningOrLiquidation => "Function disabled. Reason: Parachain must be in RUNNING or ERROR/LIQUIDATION state.",
            Error::ParachainOracleOfflineError => "Function disabled. Reason: Parachain is in ERROR state - exchange rate oracle is offline.",
            Error::ParachainLiquidationError => "Function disabled. Reason Parachain is in ERROR state - at least one vault is being liquidated.",
            Error::RuntimeError => "Runtime error",
        }
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        String::from(self.message())
    }
}

impl From<Error> for DispatchError {
    fn from(error: Error) -> Self {
        DispatchError::Module {
            // FIXME: this should be set to the module returning the error
            // It should be super easy to do if Substrate has an "after request"
            // kind of middleware but not sure if it does
            index: 0,
            error: error as u8,
            message: Some(error.message()),
        }
    }
}

pub type Result<T> = sp_std::result::Result<T, Error>;
pub type UnitResult = Result<()>;
