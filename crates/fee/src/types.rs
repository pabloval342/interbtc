use crate::Config;
use codec::{Decode, Encode};
use frame_support::traits::Currency;
use sp_arithmetic::FixedPointNumber;

pub(crate) type Backing<T> =
    <<T as currency::Config<currency::Backing>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub(crate) type Issuing<T> =
    <<T as currency::Config<currency::Issuing>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub(crate) type UnsignedFixedPoint<T> = <T as Config>::UnsignedFixedPoint;

// TODO: concrete type is the same, circumvent this conversion
pub(crate) type Inner<T> = <<T as Config>::UnsignedFixedPoint as FixedPointNumber>::Inner;

/// Storage version.
#[derive(Encode, Decode, Eq, PartialEq)]
pub enum Version {
    /// Initial version.
    V0,
    /// Use sovereign account ID.
    V1,
}
