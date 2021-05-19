use btc_relay::{BtcAddress, BtcPublicKey};
use codec::{Decode, Encode};
use frame_support::traits::Currency;
#[cfg(feature = "std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sp_core::H256;

/// Storage version.
#[derive(Encode, Decode, Eq, PartialEq)]
pub enum Version {
    /// Initial version.
    V0,
    /// BtcAddress type with script format.
    V1,
    /// IssueRequestStatus
    V2,
    /// ActiveBlockNumber, btc_height
    V3,
}

pub(crate) type Backing<T> =
    <<T as currency::Config<currency::Backing>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub(crate) type Issuing<T> =
    <<T as currency::Config<currency::Issuing>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum IssueRequestStatus {
    Pending,
    /// optional refund ID
    Completed(Option<H256>),
    Cancelled,
}

impl Default for IssueRequestStatus {
    fn default() -> Self {
        IssueRequestStatus::Pending
    }
}

// Due to a known bug in serde we need to specify how u128 is (de)serialized.
// See https://github.com/paritytech/substrate/issues/4641
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct IssueRequest<AccountId, BlockNumber, Issuing, Backing> {
    pub vault: AccountId,
    pub opentime: BlockNumber,
    pub period: BlockNumber,
    #[cfg_attr(feature = "std", serde(bound(deserialize = "Backing: std::str::FromStr")))]
    #[cfg_attr(feature = "std", serde(deserialize_with = "deserialize_from_string"))]
    #[cfg_attr(feature = "std", serde(bound(serialize = "Backing: std::fmt::Display")))]
    #[cfg_attr(feature = "std", serde(serialize_with = "serialize_as_string"))]
    pub griefing_collateral: Backing,
    #[cfg_attr(feature = "std", serde(bound(deserialize = "Issuing: std::str::FromStr")))]
    #[cfg_attr(feature = "std", serde(deserialize_with = "deserialize_from_string"))]
    #[cfg_attr(feature = "std", serde(bound(serialize = "Issuing: std::fmt::Display")))]
    #[cfg_attr(feature = "std", serde(serialize_with = "serialize_as_string"))]
    /// the number of tokens that will be transfered to the user (as such, this does not include the fee)
    pub amount: Issuing,
    #[cfg_attr(feature = "std", serde(bound(deserialize = "Issuing: std::str::FromStr")))]
    #[cfg_attr(feature = "std", serde(deserialize_with = "deserialize_from_string"))]
    #[cfg_attr(feature = "std", serde(bound(serialize = "Issuing: std::fmt::Display")))]
    #[cfg_attr(feature = "std", serde(serialize_with = "serialize_as_string"))]
    /// the number of tokens that will be tranferred to the fee pool
    pub fee: Issuing,
    pub requester: AccountId,
    pub btc_address: BtcAddress,
    pub btc_public_key: BtcPublicKey,
    pub btc_height: u32,
    pub status: IssueRequestStatus,
}

#[cfg(feature = "std")]
fn serialize_as_string<S: Serializer, T: std::fmt::Display>(t: &T, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&t.to_string())
}

#[cfg(feature = "std")]
fn deserialize_from_string<'de, D: Deserializer<'de>, T: std::str::FromStr>(deserializer: D) -> Result<T, D::Error> {
    let s = String::deserialize(deserializer)?;
    s.parse::<T>()
        .map_err(|_| serde::de::Error::custom("Parse from string failed"))
}
