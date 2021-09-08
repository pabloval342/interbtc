use crate as vault_registry;
use crate::{Config, Error};
use frame_support::{parameter_types, traits::GenesisBuild, PalletId};
use mocktopus::{macros::mockable, mocking::clear_mocks};
use orml_tokens::CurrencyAdapter;
use orml_traits::parameter_type_with_key;
pub use primitives::CurrencyId;
use sp_arithmetic::{FixedI128, FixedPointNumber, FixedU128};
use sp_core::H256;
use sp_runtime::{
    testing::{Header, TestXt},
    traits::{BlakeTwo256, IdentityLookup, One, Zero},
};

pub(crate) type Extrinsic = TestXt<Call, ()>;
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
        Security: security::{Pallet, Call, Storage, Event<T>},
        VaultRegistry: vault_registry::{Pallet, Call, Config<T>, Storage, Event<T>, ValidateUnsigned},
        Oracle: oracle::{Pallet, Call, Config<T>, Storage, Event<T>},
        Staking: staking::{Pallet, Storage, Event<T>},
        Fee: fee::{Pallet, Call, Config<T>, Storage},
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
pub const GRIEFING_CURRENCY: CurrencyId = CurrencyId::DOT;
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

parameter_types! {
    pub const MinimumPeriod: Moment = 5;
}

impl pallet_timestamp::Config for Test {
    type Moment = Moment;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl oracle::Config for Test {
    type Event = TestEvent;
    type WeightInfo = ();
}

pub struct CurrencyConvert;
impl currency::CurrencyConversion<currency::Amount<Test>, CurrencyId> for CurrencyConvert {
    fn convert(
        amount: &currency::Amount<Test>,
        to: CurrencyId,
    ) -> Result<currency::Amount<Test>, sp_runtime::DispatchError> {
        convert_to(to, amount.clone())
    }
}

#[cfg_attr(test, mockable)]
pub fn convert_to(
    to: CurrencyId,
    amount: currency::Amount<Test>,
) -> Result<currency::Amount<Test>, sp_runtime::DispatchError> {
    <oracle::Pallet<Test>>::convert(&amount, to)
}

impl currency::Config for Test {
    type SignedInner = SignedInner;
    type SignedFixedPoint = SignedFixedPoint;
    type UnsignedFixedPoint = UnsignedFixedPoint;
    type Balance = Balance;
    type GetWrappedCurrencyId = GetWrappedCurrencyId;
    type CurrencyConversion = CurrencyConvert;
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
    type UnsignedInner = Balance;
    type VaultRewards = reward::RewardsCurrencyAdapter<Test, (), GetWrappedCurrencyId>;
    type VaultStaking = staking::StakingCurrencyAdapter<Test, GetWrappedCurrencyId>;
    type OnSweep = ();
}

parameter_types! {
    pub const VaultPalletId: PalletId = PalletId(*b"mod/vreg");
}

impl Config for Test {
    type PalletId = VaultPalletId;
    type Event = TestEvent;
    type RandomnessSource = pallet_randomness_collective_flip::Pallet<Test>;
    type Balance = Balance;
    type WeightInfo = ();
    type GetGriefingCollateralCurrencyId = GetCollateralCurrencyId;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Test
where
    Call: From<C>,
{
    type OverarchingCall = Call;
    type Extrinsic = Extrinsic;
}

impl security::Config for Test {
    type Event = TestEvent;
}

impl staking::Config for Test {
    type Event = TestEvent;
    type SignedFixedPoint = SignedFixedPoint;
    type SignedInner = SignedInner;
    type CurrencyId = CurrencyId;
}

pub type TestEvent = Event;
pub type TestError = Error<Test>;
pub type SecurityError = security::Error<Test>;
pub type CurrencyError = currency::Error<Test>;
pub type TokensError = orml_tokens::Error<Test>;

pub struct ExtBuilder;

pub const DEFAULT_ID: u64 = 3;
pub const OTHER_ID: u64 = 4;
pub const RICH_ID: u64 = 5;
pub const DEFAULT_COLLATERAL: u128 = 100000;
pub const RICH_COLLATERAL: u128 = DEFAULT_COLLATERAL + 100000;
pub const MULTI_VAULT_TEST_IDS: [u64; 4] = [100, 101, 102, 103];
pub const MULTI_VAULT_TEST_COLLATERAL: u128 = 100000;

impl ExtBuilder {
    pub fn build_with(conf: orml_tokens::GenesisConfig<Test>) -> sp_io::TestExternalities {
        let mut storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

        conf.assimilate_storage(&mut storage).unwrap();

        // Parameters to be set in tests
        vault_registry::GenesisConfig::<Test> {
            minimum_collateral_vault: vec![(DEFAULT_TESTING_CURRENCY, 0)],
            punishment_delay: 0,
            system_collateral_ceiling: vec![(DEFAULT_TESTING_CURRENCY, 1_000_000_000_000)],
            secure_collateral_threshold: vec![(DEFAULT_TESTING_CURRENCY, UnsignedFixedPoint::one())],
            premium_redeem_threshold: vec![(DEFAULT_TESTING_CURRENCY, UnsignedFixedPoint::one())],
            liquidation_collateral_threshold: vec![(DEFAULT_TESTING_CURRENCY, UnsignedFixedPoint::one())],
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        sp_io::TestExternalities::from(storage)
    }
    pub fn build() -> sp_io::TestExternalities {
        ExtBuilder::build_with(orml_tokens::GenesisConfig::<Test> {
            balances: vec![
                (DEFAULT_ID, DEFAULT_TESTING_CURRENCY, DEFAULT_COLLATERAL),
                (OTHER_ID, DEFAULT_TESTING_CURRENCY, DEFAULT_COLLATERAL),
                (RICH_ID, DEFAULT_TESTING_CURRENCY, RICH_COLLATERAL),
                (
                    MULTI_VAULT_TEST_IDS[0],
                    DEFAULT_TESTING_CURRENCY,
                    MULTI_VAULT_TEST_COLLATERAL,
                ),
                (
                    MULTI_VAULT_TEST_IDS[1],
                    DEFAULT_TESTING_CURRENCY,
                    MULTI_VAULT_TEST_COLLATERAL,
                ),
                (
                    MULTI_VAULT_TEST_IDS[2],
                    DEFAULT_TESTING_CURRENCY,
                    MULTI_VAULT_TEST_COLLATERAL,
                ),
                (
                    MULTI_VAULT_TEST_IDS[3],
                    DEFAULT_TESTING_CURRENCY,
                    MULTI_VAULT_TEST_COLLATERAL,
                ),
            ],
        })
    }
}

pub(crate) fn set_default_thresholds() {
    let secure = UnsignedFixedPoint::checked_from_rational(200, 100).unwrap(); // 200%
    let premium = UnsignedFixedPoint::checked_from_rational(120, 100).unwrap(); // 120%
    let liquidation = UnsignedFixedPoint::checked_from_rational(110, 100).unwrap(); // 110%

    VaultRegistry::set_secure_collateral_threshold(DEFAULT_TESTING_CURRENCY, secure);
    VaultRegistry::set_premium_redeem_threshold(DEFAULT_TESTING_CURRENCY, premium);
    VaultRegistry::set_liquidation_collateral_threshold(DEFAULT_TESTING_CURRENCY, liquidation);
}

pub fn run_test<T>(test: T)
where
    T: FnOnce(),
{
    clear_mocks();
    ExtBuilder::build().execute_with(|| {
        System::set_block_number(1);
        Security::set_active_block_number(1);
        set_default_thresholds();
        <oracle::Pallet<Test>>::_set_exchange_rate(DEFAULT_TESTING_CURRENCY, UnsignedFixedPoint::one()).unwrap();
        test()
    })
}
