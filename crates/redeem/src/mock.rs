use crate as redeem;
use crate::{Config, Error};
use codec::{Decode, Encode};
use frame_support::{assert_ok, parameter_types, traits::GenesisBuild, PalletId};
use mocktopus::mocking::clear_mocks;
use orml_tokens::CurrencyAdapter;
use orml_traits::parameter_type_with_key;
use sp_arithmetic::{FixedI128, FixedPointNumber, FixedU128};
use sp_core::H256;
use sp_runtime::{
    testing::{Header, TestXt},
    traits::{BlakeTwo256, IdentityLookup, One, Zero},
};

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
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Call, Storage},

        // Tokens & Balances
        Tokens: orml_tokens::{Pallet, Storage, Config<T>, Event<T>},

        VaultRewards: reward::<Instance1>::{Pallet, Call, Storage, Event<T>},
        RelayerRewards: reward::<Instance2>::{Pallet, Call, Storage, Event<T>},

        // Operational
        BTCRelay: btc_relay::{Pallet, Call, Config<T>, Storage, Event<T>},
        Security: security::{Pallet, Call, Storage, Event<T>},
        VaultRegistry: vault_registry::{Pallet, Call, Config<T>, Storage, Event<T>},
        ExchangeRateOracle: exchange_rate_oracle::{Pallet, Call, Config<T>, Storage, Event<T>},
        Redeem: redeem::{Pallet, Call, Config<T>, Storage, Event<T>},
        Fee: fee::{Pallet, Call, Config<T>, Storage, Event<T>},
        Sla: sla::{Pallet, Call, Config<T>, Storage, Event<T>},
    }
);

pub type AccountId = u64;
pub type Balance = u128;
pub type Amount = i128;
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

#[derive(Encode, Decode, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum CurrencyId {
    DOT,
    INTERBTC,
}

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
    type Amount = Amount;
    type CurrencyId = CurrencyId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
    type MaxLocks = MaxLocks;
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
    type SignedInner = SignedInner;
    type Balance = Balance;
    type SignedFixedPoint = SignedFixedPoint;
    type UnsignedFixedPoint = UnsignedFixedPoint;
    type WeightInfo = ();
    type CollateralVaultRewards = reward::RewardsCurrencyAdapter<Test, reward::Vault, GetCollateralCurrencyId>;
    type WrappedVaultRewards = reward::RewardsCurrencyAdapter<Test, reward::Vault, GetWrappedCurrencyId>;
    type Collateral = CurrencyAdapter<Test, GetCollateralCurrencyId>;
    type Wrapped = CurrencyAdapter<Test, GetWrappedCurrencyId>;
}

impl reward::Config<reward::Vault> for Test {
    type Event = TestEvent;
    type SignedFixedPoint = SignedFixedPoint;
    type CurrencyId = CurrencyId;
}

impl reward::Config<reward::Relayer> for Test {
    type Event = TestEvent;
    type SignedFixedPoint = SignedFixedPoint;
    type CurrencyId = CurrencyId;
}

impl btc_relay::Config for Test {
    type Event = TestEvent;
    type WeightInfo = ();
}

impl security::Config for Test {
    type Event = TestEvent;
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

impl exchange_rate_oracle::Config for Test {
    type Event = TestEvent;
    type Balance = Balance;
    type UnsignedFixedPoint = UnsignedFixedPoint;
    type WeightInfo = ();
}

parameter_types! {
    pub const FeePalletId: PalletId = PalletId(*b"mod/fees");
}

impl fee::Config for Test {
    type PalletId = FeePalletId;
    type Event = TestEvent;
    type WeightInfo = ();
    type SignedFixedPoint = SignedFixedPoint;
    type SignedInner = SignedInner;
    type UnsignedFixedPoint = UnsignedFixedPoint;
    type UnsignedInner = UnsignedInner;
    type CollateralVaultRewards = reward::RewardsCurrencyAdapter<Test, reward::Vault, GetCollateralCurrencyId>;
    type WrappedVaultRewards = reward::RewardsCurrencyAdapter<Test, reward::Vault, GetWrappedCurrencyId>;
    type CollateralRelayerRewards = reward::RewardsCurrencyAdapter<Test, reward::Relayer, GetCollateralCurrencyId>;
    type WrappedRelayerRewards = reward::RewardsCurrencyAdapter<Test, reward::Relayer, GetWrappedCurrencyId>;
    type Collateral = CurrencyAdapter<Test, GetCollateralCurrencyId>;
    type Wrapped = CurrencyAdapter<Test, GetWrappedCurrencyId>;
}

impl sla::Config for Test {
    type Event = TestEvent;
    type SignedFixedPoint = SignedFixedPoint;
    type SignedInner = SignedInner;
    type Balance = Balance;
    type CollateralVaultRewards = reward::RewardsCurrencyAdapter<Test, reward::Vault, GetCollateralCurrencyId>;
    type WrappedVaultRewards = reward::RewardsCurrencyAdapter<Test, reward::Vault, GetWrappedCurrencyId>;
    type CollateralRelayerRewards = reward::RewardsCurrencyAdapter<Test, reward::Relayer, GetCollateralCurrencyId>;
    type WrappedRelayerRewards = reward::RewardsCurrencyAdapter<Test, reward::Relayer, GetWrappedCurrencyId>;
}

impl Config for Test {
    type Event = TestEvent;
    type WeightInfo = ();
}

pub type TestEvent = Event;
pub type TestError = Error<Test>;
pub type VaultRegistryError = vault_registry::Error<Test>;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CAROL: AccountId = 3;

pub const ALICE_BALANCE: u128 = 1_005_000;
pub const BOB_BALANCE: u128 = 1_005_000;
pub const CAROL_BALANCE: u128 = 1_005_000;

pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build_with(balances: orml_tokens::GenesisConfig<Test>) -> sp_io::TestExternalities {
        let mut storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

        balances.assimilate_storage(&mut storage).unwrap();

        fee::GenesisConfig::<Test> {
            issue_fee: UnsignedFixedPoint::checked_from_rational(5, 1000).unwrap(), // 0.5%
            issue_griefing_collateral: UnsignedFixedPoint::checked_from_rational(5, 100000).unwrap(), // 0.005%
            refund_fee: UnsignedFixedPoint::checked_from_rational(5, 1000).unwrap(), // 0.5%
            redeem_fee: UnsignedFixedPoint::checked_from_rational(5, 1000).unwrap(), // 0.5%
            premium_redeem_fee: UnsignedFixedPoint::checked_from_rational(5, 100).unwrap(), // 5%
            punishment_fee: UnsignedFixedPoint::checked_from_rational(1, 10).unwrap(), // 10%
            replace_griefing_collateral: UnsignedFixedPoint::checked_from_rational(1, 10).unwrap(), // 10%
            maintainer_account_id: 1,
            vault_rewards: UnsignedFixedPoint::checked_from_rational(77, 100).unwrap(),
            relayer_rewards: UnsignedFixedPoint::checked_from_rational(3, 100).unwrap(),
            maintainer_rewards: UnsignedFixedPoint::checked_from_rational(20, 100).unwrap(),
            nomination_rewards: UnsignedFixedPoint::checked_from_rational(0, 100).unwrap(),
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        vault_registry::GenesisConfig::<Test> {
            minimum_collateral_vault: 0,
            punishment_delay: 8,
            secure_collateral_threshold: UnsignedFixedPoint::checked_from_rational(200, 100).unwrap(),
            premium_redeem_threshold: UnsignedFixedPoint::checked_from_rational(120, 100).unwrap(),
            liquidation_collateral_threshold: UnsignedFixedPoint::checked_from_rational(110, 100).unwrap(),
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        sla::GenesisConfig::<Test> {
            vault_target_sla: SignedFixedPoint::from(100),
            vault_redeem_failure_sla_change: SignedFixedPoint::from(-10),
            vault_execute_issue_max_sla_change: SignedFixedPoint::from(4),
            vault_deposit_max_sla_change: SignedFixedPoint::from(4),
            vault_withdraw_max_sla_change: SignedFixedPoint::from(-4),
            vault_submit_issue_proof: SignedFixedPoint::from(0),
            vault_refund: SignedFixedPoint::from(1),
            relayer_target_sla: SignedFixedPoint::from(100),
            relayer_store_block: SignedFixedPoint::from(1),
            relayer_theft_report: SignedFixedPoint::from(1),
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        redeem::GenesisConfig::<Test> {
            redeem_transaction_size: 400,
            redeem_period: 10,
            redeem_btc_dust_value: 2,
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        storage.into()
    }

    pub fn build() -> sp_io::TestExternalities {
        ExtBuilder::build_with(orml_tokens::GenesisConfig::<Test> {
            balances: vec![
                (ALICE, DOT, ALICE_BALANCE),
                (BOB, DOT, BOB_BALANCE),
                (CAROL, DOT, CAROL_BALANCE),
                (ALICE, INTERBTC, ALICE_BALANCE),
                (BOB, INTERBTC, BOB_BALANCE),
                (CAROL, INTERBTC, CAROL_BALANCE),
            ],
        })
    }
}

pub fn run_test<T>(test: T)
where
    T: FnOnce(),
{
    clear_mocks();
    ExtBuilder::build().execute_with(|| {
        assert_ok!(<exchange_rate_oracle::Pallet<Test>>::_set_exchange_rate(
            UnsignedFixedPoint::one()
        ));
        Security::set_active_block_number(1);
        System::set_block_number(1);
        test();
    });
}
