pub use codec::Encode;
use frame_support::traits::GenesisBuild;
pub use frame_support::{assert_noop, assert_ok, traits::Currency};
pub use frame_system::RawOrigin;
pub use orml_traits::{location::RelativeLocations, Change, GetByKey, MultiCurrency};
pub use sp_core::H160;
pub use sp_runtime::{
    traits::{AccountIdConversion, BadOrigin, BlakeTwo256, Convert, Hash, Zero},
    DispatchError, DispatchResult, FixedPointNumber, MultiAddress, Perbill, Permill,
};
pub use xcm::latest::prelude::*;
pub use xcm_emulator::XcmExecutor;

pub use kintsugi_imports::*;
mod kintsugi_imports {
    pub use frame_support::{parameter_types, weights::Weight};
    pub use kintsugi_runtime_parachain::*;
    pub use sp_runtime::{traits::AccountIdConversion, FixedPointNumber};

    parameter_types! {
        pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account();
    }

    pub use primitives::{CurrencyId::Token, KINT, KSM};
}

pub const KINTSUGI_PARA_ID: u32 = 2092;
pub const SIBLING_PARA_ID: u32 = 2001;

#[allow(dead_code)]
pub const DEFAULT: [u8; 32] = [0u8; 32];

#[allow(dead_code)]
pub const ALICE: [u8; 32] = [4u8; 32];
#[allow(dead_code)]
pub const BOB: [u8; 32] = [5u8; 32];

pub struct ExtBuilder {
    balances: Vec<(AccountId, CurrencyId, Balance)>,
    parachain_id: u32,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            balances: vec![],
            parachain_id: 2000,
        }
    }
}

impl ExtBuilder {
    pub fn balances(mut self, balances: Vec<(AccountId, CurrencyId, Balance)>) -> Self {
        self.balances = balances;
        self
    }

    #[allow(dead_code)]
    pub fn parachain_id(mut self, parachain_id: u32) -> Self {
        self.parachain_id = parachain_id;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        let native_currency_id = GetNativeCurrencyId::get();

        orml_tokens::GenesisConfig::<Runtime> {
            balances: self
                .balances
                .into_iter()
                .filter(|(_, currency_id, _)| *currency_id != native_currency_id)
                .collect::<Vec<_>>(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        <parachain_info::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
            &parachain_info::GenesisConfig {
                parachain_id: self.parachain_id.into(),
            },
            &mut t,
        )
        .unwrap();

        <pallet_xcm::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
            &pallet_xcm::GenesisConfig {
                safe_xcm_version: Some(2),
            },
            &mut t,
        )
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
