use crate::Config;
use codec::{Decode, Encode};
use sp_arithmetic::FixedPointNumber;

pub(crate) type Collateral<T> = UnsignedInner<T>;

pub(crate) type Wrapped<T> = UnsignedInner<T>;

pub(crate) type BalanceOf<T> = <T as currency::Config>::Balance;

pub(crate) type SignedFixedPoint<T> = <T as Config>::SignedFixedPoint;

pub(crate) type UnsignedFixedPoint<T> = <T as Config>::UnsignedFixedPoint;

pub(crate) type UnsignedInner<T> = <<T as Config>::UnsignedFixedPoint as FixedPointNumber>::Inner;

/// Storage version.
#[derive(Encode, Decode, Eq, PartialEq)]
pub enum Version {
    /// Initial version.
    V0,
    /// Use sovereign account ID.
    V1,
}
