use super::*;
use cumulus_primitives_core::ParaId;
use orml_asset_registry::{AssetRegistryTrader, FixedRateAssetRegistryTrader};
use orml_traits::FixedConversionRateProvider;
use orml_xcm_support::{DepositToAlternative, IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use primitives::{Balance, CurrencyId, CurrencyId::ForeignAsset};
use runtime_common::Transactless;
use xcm::latest::{
    prelude::{AccountId32, *},
    Weight,
};
use xcm_builder::{
    AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom, AllowTopLevelPaidExecutionFrom,
    EnsureXcmOrigin, FixedRateOfFungible, FixedWeightBounds, NativeAsset, ParentIsPreset, RelayChainAsNative,
    SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32,
    SovereignSignedViaLocation, TakeRevenue, TakeWeightCredit,
};
use xcm_executor::{traits::WithOriginFilter, XcmExecutor};

parameter_types! {
    pub const ParentLocation: MultiLocation = MultiLocation::parent();
    pub const ParentNetwork: NetworkId = NetworkId::Kusama;
    pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
    pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

/// Means for transacting assets on this chain.
type LocationToAccountId = (
    // The parent (Relay-chain) origin converts to the default `AccountId`.
    ParentIsPreset<AccountId>,
    // Sibling parachain origins convert to AccountId via the `ParaId::into`.
    SiblingParachainConvertsVia<Sibling, AccountId>,
    // Straight up local `AccountId32` origins just alias directly to `AccountId`.
    AccountId32Aliases<ParentNetwork, AccountId>,
);

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
    // Sovereign account converter; this attempts to derive an `AccountId` from the origin location
    // using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
    // foreign chains who want to have a local sovereign account on this chain which they control.
    SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
    // Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
    // recognised.
    RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
    // Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
    // recognised.
    SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
    // Native signed account converter; this just converts an `AccountId32` origin into a normal
    // `Origin::Signed` origin of the same 32-byte value.
    SignedAccountId32AsNative<ParentNetwork, RuntimeOrigin>,
    // Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
    XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
    // One XCM operation is 200_000_000 weight, cross-chain transfer ~= 2x of transfer.
    pub UnitWeightCost: Weight = Weight::from_ref_time(200_000_000);
}

pub type Barrier = Transactless<(
    TakeWeightCredit,
    AllowTopLevelPaidExecutionFrom<Everything>,
    AllowKnownQueryResponses<PolkadotXcm>,
    AllowSubscriptionsFrom<Everything>,
)>; // required for others to keep track of our xcm version

parameter_types! {
    pub const MaxInstructions: u32 = 100;
}

pub struct XcmConfig;

// the cost to to execute a no-op extrinsic
fn base_tx_in_xcm() -> Balance {
    PARENT_TOKEN_ID.one() / 50_000
}

pub fn xcm_per_second() -> u128 {
    let base_weight = Balance::from(ExtrinsicBaseWeight::get().ref_time());
    let base_tx_per_second = (WEIGHT_REF_TIME_PER_SECOND as u128) / base_weight;
    base_tx_per_second * base_tx_in_xcm()
}

parameter_types! {
    pub KsmPerSecond: (AssetId, u128, u128) = (
        MultiLocation::parent().into(),
        xcm_per_second(),
        0, // todo: determine how much to charge per mb of proof
    );
    pub NativePerSecond: (AssetId, u128, u128) = ( // can be removed once we no longer need to support polkadot < 0.9.16
        non_canonical_currency_location(NATIVE_CURRENCY_ID).into(),
        (xcm_per_second() * 4) / 3,
        0, // todo: determine how much to charge per mb of proof
    );
    pub WrappedPerSecond: (AssetId, u128, u128) = ( // can be removed once we no longer need to support polkadot < 0.9.16
        non_canonical_currency_location(WRAPPED_CURRENCY_ID).into(),
        xcm_per_second() / 1_500_000,
        0, // todo: determine how much to charge per mb of proof
    );
    pub CanonicalizedNativePerSecond: (AssetId, u128, u128) = (
        canonical_currency_location(NATIVE_CURRENCY_ID).into(),
        (xcm_per_second() * 4) / 3,
        0, // todo: determine how much to charge per mb of proof
    );
    pub CanonicalizedWrappedPerSecond: (AssetId, u128, u128) = (
        canonical_currency_location(WRAPPED_CURRENCY_ID).into(),
        xcm_per_second() / 1_500_000,
        0, // todo: determine how much to charge per mb of proof
    );
    pub const RelayNetwork: NetworkId = NetworkId::Polkadot;
    pub UniversalLocation: InteriorMultiLocation = X2(GlobalConsensus(RelayNetwork::get()), Parachain(ParachainInfo::parachain_id().into()));
}

pub struct ToTreasury;
impl TakeRevenue for ToTreasury {
    fn take_revenue(revenue: MultiAsset) {
        if let MultiAsset {
            id: Concrete(location),
            fun: Fungible(amount),
        } = revenue
        {
            if let Some(currency_id) = CurrencyIdConvert::convert(location) {
                // Note: we should ensure that treasury account has existential deposit for all of the cross-chain
                // asset. Ignore the result.
                let _ = Tokens::deposit(currency_id, &TreasuryAccount::get(), amount);
            }
        }
    }
}

pub type Trader = (
    FixedRateOfFungible<KsmPerSecond, ToTreasury>,
    FixedRateOfFungible<NativePerSecond, ToTreasury>,
    FixedRateOfFungible<WrappedPerSecond, ToTreasury>,
    FixedRateOfFungible<CanonicalizedNativePerSecond, ToTreasury>,
    FixedRateOfFungible<CanonicalizedWrappedPerSecond, ToTreasury>,
    AssetRegistryTrader<FixedRateAssetRegistryTrader<MyFixedConversionRateProvider>, ToTreasury>,
);

pub struct MyFixedConversionRateProvider;
impl FixedConversionRateProvider for MyFixedConversionRateProvider {
    fn get_fee_per_second(location: &MultiLocation) -> Option<u128> {
        let metadata = AssetRegistry::fetch_metadata_by_location(location)?;
        Some(metadata.additional.fee_per_second)
    }
}

// If all other trader items fail to apply, then execute for free. This is useful in in xcm
// testing: it allows the sibling to place the assets in UnknownTokens so that it can send
// them back. SHOULD NOT BE INCLUDED IN MAINNET
pub struct FreeTestExection;
impl WeightTrader for FreeTestExection {
    fn new() -> Self {
        Self
    }

    fn buy_weight(&mut self, _weight: Weight, payment: Assets) -> Result<Assets, XcmError> {
        log::warn!("xcm trader fallthrough: allowing free execution (TESTING ONLY)");
        Ok(payment)
    }

    fn refund_weight(&mut self, _weight: Weight) -> Option<MultiAsset> {
        log::warn!("xcm trader refund fallthrough: giving no refund (TESTING ONLY)");
        None
    }
}

/// A call filter for the XCM Transact instruction. This is a temporary measure until we properly
/// account for proof size weights.
///
/// Calls that are allowed through this filter must:
/// 1. Have a fixed weight;
/// 2. Cannot lead to another call being made;
/// 3. Have a defined proof size weight, e.g. no unbounded vecs in call parameters.
pub struct SafeCallFilter;
impl Contains<RuntimeCall> for SafeCallFilter {
    fn contains(call: &RuntimeCall) -> bool {
        // we need to filter all calls that can recurse. We're being a bit overly conservative here
        // by completly blocking the pallets below rather than filter per specific call.
        match call {
            RuntimeCall::Sudo(..) | RuntimeCall::Proxy(..) | RuntimeCall::Multisig(..) | RuntimeCall::Utility(..) => {
                // these calls can recurse - disallow
                false
            }
            RuntimeCall::Issue(..) | RuntimeCall::Replace(..) | RuntimeCall::Redeem(..) | RuntimeCall::BTCRelay(..) => {
                // disallow anything to do with btc transactions since btc tx may be unbounded
                false
            }
            _ => true,
        }
    }
}

impl xcm_executor::Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    // How to withdraw and deposit an asset.
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = XcmOriginToTransactDispatchOrigin;
    type IsReserve = MultiNativeAsset<AbsoluteReserveProvider>;
    type IsTeleporter = NativeAsset; // <- should be enough to allow teleportation
    type Barrier = Barrier;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type Trader = Trader;
    type ResponseHandler = PolkadotXcm;
    type SubscriptionService = PolkadotXcm;
    type AssetTrap = PolkadotXcm;
    type AssetClaims = PolkadotXcm;
    type PalletInstancesInfo = AllPalletsWithSystem;
    type MaxAssetsIntoHolding = ConstU32<8>;
    type AssetLocker = ();
    type AssetExchanger = ();
    type FeeManager = ();
    type MessageExporter = ();
    type UniversalAliases = Nothing;
    type SafeCallFilter = SafeCallFilter;
    type CallDispatcher = WithOriginFilter<SafeCallFilter>;
    type UniversalLocation = UniversalLocation;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = (SignedToAccountId32<RuntimeOrigin, AccountId, ParentNetwork>,);

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
    // Two routers - use UMP to communicate with the relay chain:
    cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, ()>, /* note: sets PriceForParentDelivery
                                                                                * to 0 */
    // ..and XCMP to communicate with the sibling chains.
    XcmpQueue,
);

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
    pub const ReachableDest: MultiLocation = MultiLocation::parent();
}

impl pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type RuntimeOrigin = RuntimeOrigin;
    type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmRouter = XcmRouter;
    type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmExecuteFilter = Nothing;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type XcmTeleportFilter = Everything;
    type XcmReserveTransferFilter = Everything;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
    type Currency = NativeCurrency; // note: not used due to the empty CurrencyMatcher
    type CurrencyMatcher = ();
    type TrustedLockers = ();
    type SovereignAccountOf = LocationToAccountId;
    type MaxLockers = ConstU32<8>;
    type UniversalLocation = UniversalLocation;
    type WeightInfo = pallet_xcm::TestWeightInfo; // todo: use actual weight
    #[cfg(feature = "runtime-benchmarks")]
    type ReachableDest = ReachableDest;
}

impl cumulus_pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type ChannelInfo = ParachainSystem;
    type VersionWrapper = PolkadotXcm;
    type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
    type ControllerOrigin = EnsureRoot<AccountId>;
    type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
    type PriceForSiblingDelivery = ();
    type WeightInfo = ();
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type ExecuteOverweightOrigin = frame_system::EnsureRoot<AccountId>;
}

pub type LocalAssetTransactor = MultiCurrencyAdapter<
    Tokens,
    UnknownTokens,
    IsNativeConcrete<CurrencyId, CurrencyIdConvert>,
    AccountId,
    LocationToAccountId,
    CurrencyId,
    CurrencyIdConvert,
    DepositToAlternative<TreasuryAccount, Tokens, CurrencyId, AccountId, Balance>,
>;

fn general_key_of(id: CurrencyId) -> Junction {
    let encoded = id.encode();
    let mut data = [0u8; 32];
    if encoded.len() > 32 {
        // we are not returning result, so panic is inevitable. Let's make it explicit.
        panic!("Currency ID was too long to be encoded");
    }
    data[..encoded.len()].copy_from_slice(&encoded[..]);
    GeneralKey {
        length: encoded.len() as u8,
        data,
    }
}
pub fn canonical_currency_location(id: CurrencyId) -> MultiLocation {
    MultiLocation::new(0, X1(general_key_of(id)))
}

pub fn non_canonical_currency_location(id: CurrencyId) -> MultiLocation {
    MultiLocation::new(1, X2(Parachain(ParachainInfo::get().into()), general_key_of(id)))
}

pub use currency_id_convert::CurrencyIdConvert;

mod currency_id_convert {
    use super::*;
    use codec::Decode;

    pub struct CurrencyIdConvert;

    impl Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
        fn convert(id: CurrencyId) -> Option<MultiLocation> {
            match id {
                PARENT_CURRENCY_ID => Some(MultiLocation::parent()),
                WRAPPED_CURRENCY_ID => Some(non_canonical_currency_location(id)),
                NATIVE_CURRENCY_ID => Some(non_canonical_currency_location(id)),
                ForeignAsset(id) => AssetRegistry::multilocation(&id).unwrap_or_default(),
                _ => None,
            }
        }
    }

    impl Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
        fn convert(location: MultiLocation) -> Option<CurrencyId> {
            fn decode_currency_id(length: u8, data: [u8; 32]) -> Option<CurrencyId> {
                let length = length as usize;
                if length > data.len() {
                    return None;
                }
                // decode the general key
                if let Ok(currency_id) = CurrencyId::decode(&mut &data[..length]) {
                    // check `currency_id` is cross-chain asset
                    match currency_id {
                        WRAPPED_CURRENCY_ID => Some(currency_id),
                        NATIVE_CURRENCY_ID => Some(currency_id),
                        _ => None,
                    }
                } else {
                    None
                }
            }

            match location.clone() {
                x if x == MultiLocation::parent() => Some(PARENT_CURRENCY_ID),
                MultiLocation {
                    parents: 1,
                    interior: X2(Parachain(id), GeneralKey { length, data }),
                } if ParaId::from(id) == ParachainInfo::get() => decode_currency_id(length, data),
                MultiLocation {
                    // adapt for reanchor canonical location: https://github.com/paritytech/polkadot/pull/4470
                    parents: 0,
                    interior: X1(GeneralKey { length, data }),
                } => decode_currency_id(length, data),
                _ => None,
            }
            .or_else(|| AssetRegistry::location_to_asset_id(&location).map(|id| CurrencyId::ForeignAsset(id)))
        }
    }

    impl Convert<MultiAsset, Option<CurrencyId>> for CurrencyIdConvert {
        fn convert(asset: MultiAsset) -> Option<CurrencyId> {
            if let MultiAsset {
                id: Concrete(location), ..
            } = asset
            {
                Self::convert(location)
            } else {
                None
            }
        }
    }
}

parameter_types! {
    pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::get().into())));
    pub const MaxAssetsForTransfer: usize = 2; // potentially useful to send both kint and kbtc at once
}

parameter_type_with_key! {
    // Only used for transferring parachain tokens to other parachains using DOT/KSM as fee currency. Currently we do not support this, hence return MAX.
    // See: https://github.com/open-web3-stack/open-runtime-module-library/blob/cadcc9fb10b8212f92668138fc8f83dc0c53acf5/xtokens/README.md#transfer-multiple-currencies
    pub ParachainMinFee: |location: MultiLocation| -> Option<u128> {
        #[allow(clippy::match_ref_pats)] // false positive
        match (location.parents, location.first_interior()) {
            _ => None,
        }
    };
}

pub struct AccountIdToMultiLocation;

impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
    fn convert(account: AccountId) -> MultiLocation {
        X1(AccountId32 {
            network: None,
            id: account.into(),
        })
        .into()
    }
}

impl orml_xtokens::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type CurrencyId = CurrencyId;
    type CurrencyIdConvert = CurrencyIdConvert;
    type AccountIdToMultiLocation = AccountIdToMultiLocation;
    type SelfLocation = SelfLocation;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type BaseXcmWeight = UnitWeightCost;
    type MaxAssetsForTransfer = MaxAssetsForTransfer;
    type MinXcmFee = ParachainMinFee;
    type MultiLocationsFilter = Everything;
    type ReserveProvider = AbsoluteReserveProvider;
    type UniversalLocation = UniversalLocation;
}
