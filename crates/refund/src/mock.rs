use crate as refund;
use crate::{Config, Error};
use currency::Amount;
use frame_support::{assert_ok, parameter_types, PalletId};
use mocktopus::{macros::mockable, mocking::clear_mocks};
use orml_traits::parameter_type_with_key;
pub use primitives::CurrencyId;
use sp_core::H256;
use sp_runtime::{
    testing::{Header, TestXt},
    traits::{BlakeTwo256, IdentityLookup, One, Zero},
    FixedI128, FixedU128,
};

pub const VAULT: AccountId = 1;
pub const USER: AccountId = 2;

type TestExtrinsic = TestXt<Call, ()>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},

        // Tokens & Balances
        Tokens: orml_tokens::{Pallet, Storage, Config<T>, Event<T>},

        Rewards: reward::{Pallet, Call, Storage, Event<T>},

        // Operational
        BTCRelay: btc_relay::{Pallet, Call, Config<T>, Storage, Event<T>},
        Security: security::{Pallet, Call, Storage, Event<T>},
        VaultRegistry: vault_registry::{Pallet, Call, Config<T>, Storage, Event<T>},
        Oracle: oracle::{Pallet, Call, Config<T>, Storage, Event<T>},
        Fee: fee::{Pallet, Call, Config<T>, Storage},
        Refund: refund::{Pallet, Call, Config<T>, Storage, Event<T>},
        Staking: staking::{Pallet, Storage, Event<T>},
        Currency: currency::{Pallet},
    }
);

pub type AccountId = u64;
pub type Balance = u128;
pub type RawAmount = i128;
pub type BlockNumber = u64;
pub type Moment = u64;
pub type Index = u64;
pub type SignedFixedPoint = FixedI128;
pub type SignedInner = i128;
pub type UnsignedFixedPoint = FixedU128;
pub type UnsignedInner = u128;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = TestEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
}

impl pallet_randomness_collective_flip::Config for Test {}

pub const DEFAULT_TESTING_CURRENCY: CurrencyId = CurrencyId::DOT;
pub const DOT: CurrencyId = CurrencyId::DOT;
pub const INTERBTC: CurrencyId = CurrencyId::INTERBTC;

parameter_types! {
    pub const GetCollateralCurrencyId: CurrencyId = DOT;
    pub const GetWrappedCurrencyId: CurrencyId = INTERBTC;
    pub const MaxLocks: u32 = 50;
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
        Zero::zero()
    };
}

impl orml_tokens::Config for Test {
    type Event = Event;
    type Balance = Balance;
    type Amount = RawAmount;
    type CurrencyId = CurrencyId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
    type MaxLocks = MaxLocks;
    type DustRemovalWhitelist = ();
}

impl reward::Config for Test {
    type Event = TestEvent;
    type SignedFixedPoint = SignedFixedPoint;
    type CurrencyId = CurrencyId;
}

impl Config for Test {
    type Event = TestEvent;
    type WeightInfo = ();
}

parameter_types! {
    pub const FeePalletId: PalletId = PalletId(*b"mod/fees");
}

impl fee::Config for Test {
    type FeePalletId = FeePalletId;
    type WeightInfo = ();
    type SignedFixedPoint = SignedFixedPoint;
    type SignedInner = SignedInner;
    type UnsignedFixedPoint = UnsignedFixedPoint;
    type UnsignedInner = UnsignedInner;
    type VaultRewards = reward::RewardsCurrencyAdapter<Test, (), GetWrappedCurrencyId>;
    type VaultStaking = staking::StakingCurrencyAdapter<Test, GetWrappedCurrencyId>;
    type OnSweep = ();
}

parameter_types! {
    pub const ParachainBlocksPerBitcoinBlock: BlockNumber = 100;
}

impl btc_relay::Config for Test {
    type Event = TestEvent;
    type ParachainBlocksPerBitcoinBlock = ParachainBlocksPerBitcoinBlock;
    type WeightInfo = ();
}

impl security::Config for Test {
    type Event = TestEvent;
}

parameter_types! {
    pub const VaultPalletId: PalletId = PalletId(*b"mod/vreg");
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Test
where
    Call: From<C>,
{
    type OverarchingCall = Call;
    type Extrinsic = TestExtrinsic;
}

impl vault_registry::Config for Test {
    type PalletId = VaultPalletId;
    type Event = TestEvent;
    type RandomnessSource = pallet_randomness_collective_flip::Pallet<Test>;
    type Balance = Balance;
    type WeightInfo = ();
    type GetGriefingCollateralCurrencyId = GetCollateralCurrencyId;
}

pub struct CurrencyConvert;
impl currency::CurrencyConversion<currency::Amount<Test>, CurrencyId> for CurrencyConvert {
    fn convert(
        amount: &currency::Amount<Test>,
        to: CurrencyId,
    ) -> Result<currency::Amount<Test>, sp_runtime::DispatchError> {
        let amount = convert_to(to, amount.amount())?;
        Ok(Amount::new(amount, to))
    }
}

#[cfg_attr(test, mockable)]
pub fn convert_to(to: CurrencyId, amount: Balance) -> Result<Balance, sp_runtime::DispatchError> {
    Ok(amount) // default conversion 1:1 - overwritable with mocktopus
}

impl currency::Config for Test {
    type SignedInner = SignedInner;
    type SignedFixedPoint = SignedFixedPoint;
    type UnsignedFixedPoint = UnsignedFixedPoint;
    type Balance = Balance;
    type GetWrappedCurrencyId = GetWrappedCurrencyId;
    type CurrencyConversion = CurrencyConvert;
}

impl staking::Config for Test {
    type Event = TestEvent;
    type SignedFixedPoint = SignedFixedPoint;
    type SignedInner = SignedInner;
    type CurrencyId = CurrencyId;
}

impl oracle::Config for Test {
    type Event = TestEvent;
    type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: Moment = 5;
}

impl pallet_timestamp::Config for Test {
    type Moment = Moment;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

pub type TestEvent = Event;
pub type TestError = Error<Test>;

pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build() -> sp_io::TestExternalities {
        let storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

        sp_io::TestExternalities::from(storage)
    }
}

pub fn run_test<T>(test: T)
where
    T: FnOnce(),
{
    clear_mocks();
    ExtBuilder::build().execute_with(|| {
        assert_ok!(<oracle::Pallet<Test>>::_set_exchange_rate(
            DEFAULT_TESTING_CURRENCY,
            UnsignedFixedPoint::one()
        ));
        System::set_block_number(1);
        Security::set_active_block_number(1);
        test();
    });
}
