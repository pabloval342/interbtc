
//! Autogenerated weights for dex_general
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-14, STEPS: `100`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `enterprise`, CPU: `Intel(R) Core(TM) i7-9700K CPU @ 3.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// dex-general
// --extrinsic
// *
// --chain
// kintsugi-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 100
// --repeat
// 10
// --output
// parachain/runtime/kintsugi/src/weights
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for dex_general using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> dex_general::WeightInfo for WeightInfo<T> {
	/// Storage: DexGeneral FeeMeta (r:1 w:1)
	/// Proof Skipped: DexGeneral FeeMeta (max_values: Some(1), max_size: None, mode: Measured)
	fn set_fee_receiver() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `787`
		//  Estimated: `1282`
		// Minimum execution time: 10_253_000 picoseconds.
		Weight::from_parts(10_563_000, 1282)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DexGeneral FeeMeta (r:1 w:1)
	/// Proof Skipped: DexGeneral FeeMeta (max_values: Some(1), max_size: None, mode: Measured)
	fn set_fee_point() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `787`
		//  Estimated: `1282`
		// Minimum execution time: 10_005_000 picoseconds.
		Weight::from_parts(10_492_000, 1282)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof Skipped: DexGeneral PairStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral LiquidityPairs (r:0 w:1)
	/// Proof Skipped: DexGeneral LiquidityPairs (max_values: None, max_size: None, mode: Measured)
	fn create_pair() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1024`
		//  Estimated: `4523`
		// Minimum execution time: 26_745_000 picoseconds.
		Weight::from_parts(27_856_000, 4523)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof Skipped: DexGeneral PairStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral BootstrapLimits (r:0 w:1)
	/// Proof Skipped: DexGeneral BootstrapLimits (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral BootstrapRewards (r:0 w:1)
	/// Proof Skipped: DexGeneral BootstrapRewards (max_values: None, max_size: None, mode: Measured)
	/// The range of component `r` is `[1, 10]`.
	/// The range of component `l` is `[1, 10]`.
	fn bootstrap_create(_r: u32, _l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1024`
		//  Estimated: `5547`
		// Minimum execution time: 28_589_000 picoseconds.
		Weight::from_parts(29_361_000, 5547)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: DexGeneral BootstrapLimits (r:1 w:0)
	/// Proof Skipped: DexGeneral BootstrapLimits (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof Skipped: DexGeneral PairStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral BootstrapPersonalSupply (r:1 w:1)
	/// Proof Skipped: DexGeneral BootstrapPersonalSupply (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn bootstrap_contribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1975`
		//  Estimated: `26313`
		// Minimum execution time: 98_978_000 picoseconds.
		Weight::from_parts(102_105_000, 26313)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:0)
	/// Proof Skipped: DexGeneral PairStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral BootstrapPersonalSupply (r:1 w:1)
	/// Proof Skipped: DexGeneral BootstrapPersonalSupply (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral BootstrapEndStatus (r:1 w:0)
	/// Proof Skipped: DexGeneral BootstrapEndStatus (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral LiquidityPairs (r:1 w:0)
	/// Proof Skipped: DexGeneral LiquidityPairs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DexGeneral BootstrapRewards (r:1 w:0)
	/// Proof Skipped: DexGeneral BootstrapRewards (max_values: None, max_size: None, mode: Measured)
	fn bootstrap_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3218`
		//  Estimated: `36248`
		// Minimum execution time: 89_325_000 picoseconds.
		Weight::from_parts(95_558_000, 36248)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof Skipped: DexGeneral PairStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:5 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: DexGeneral LiquidityPairs (r:0 w:1)
	/// Proof Skipped: DexGeneral LiquidityPairs (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral BootstrapEndStatus (r:0 w:1)
	/// Proof Skipped: DexGeneral BootstrapEndStatus (max_values: None, max_size: None, mode: Measured)
	fn bootstrap_end() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2128`
		//  Estimated: `29525`
		// Minimum execution time: 116_357_000 picoseconds.
		Weight::from_parts(123_142_000, 29525)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(10_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof Skipped: DexGeneral PairStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral BootstrapRewards (r:1 w:1)
	/// Proof Skipped: DexGeneral BootstrapRewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral BootstrapLimits (r:0 w:1)
	/// Proof Skipped: DexGeneral BootstrapLimits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `r` is `[1, 10]`.
	/// The range of component `l` is `[1, 10]`.
	fn bootstrap_update(r: u32, l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1330`
		//  Estimated: `8940`
		// Minimum execution time: 34_083_000 picoseconds.
		Weight::from_parts(34_855_000, 8940)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 69).saturating_mul(r.into()))
			.saturating_add(Weight::from_parts(0, 3).saturating_mul(l.into()))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof Skipped: DexGeneral PairStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral BootstrapPersonalSupply (r:1 w:1)
	/// Proof Skipped: DexGeneral BootstrapPersonalSupply (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn bootstrap_refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2342`
		//  Estimated: `22597`
		// Minimum execution time: 90_964_000 picoseconds.
		Weight::from_parts(92_132_000, 22597)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof Skipped: DexGeneral PairStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:5 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: DexGeneral LiquidityPairs (r:1 w:0)
	/// Proof Skipped: DexGeneral LiquidityPairs (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral KLast (r:1 w:1)
	/// Proof Skipped: DexGeneral KLast (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral FeeMeta (r:1 w:0)
	/// Proof Skipped: DexGeneral FeeMeta (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2040`
		//  Estimated: `34143`
		// Minimum execution time: 127_878_000 picoseconds.
		Weight::from_parts(131_913_000, 34143)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof Skipped: DexGeneral PairStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:5 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: DexGeneral LiquidityPairs (r:1 w:0)
	/// Proof Skipped: DexGeneral LiquidityPairs (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral KLast (r:1 w:1)
	/// Proof Skipped: DexGeneral KLast (max_values: None, max_size: None, mode: Measured)
	/// Storage: DexGeneral FeeMeta (r:1 w:0)
	/// Proof Skipped: DexGeneral FeeMeta (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2548`
		//  Estimated: `36175`
		// Minimum execution time: 117_025_000 picoseconds.
		Weight::from_parts(119_535_000, 36175)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Tokens Accounts (r:20 w:20)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: DexGeneral PairStatuses (r:9 w:0)
	/// Proof Skipped: DexGeneral PairStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:9 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[2, 10]`.
	fn swap_exact_assets_for_assets(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1119 + a * (590 ±0)`
		//  Estimated: `7379 + a * (9103 ±17)`
		// Minimum execution time: 99_982_000 picoseconds.
		Weight::from_parts(27_642_058, 7379)
			// Standard Error: 55_690
			.saturating_add(Weight::from_parts(38_321_239, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 9103).saturating_mul(a.into()))
	}
	/// Storage: Tokens Accounts (r:20 w:20)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: DexGeneral PairStatuses (r:9 w:0)
	/// Proof Skipped: DexGeneral PairStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:9 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[2, 10]`.
	fn swap_assets_for_exact_assets(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1119 + a * (590 ±0)`
		//  Estimated: `7379 + a * (9103 ±17)`
		// Minimum execution time: 99_732_000 picoseconds.
		Weight::from_parts(27_139_839, 7379)
			// Standard Error: 45_498
			.saturating_add(Weight::from_parts(37_953_281, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 9103).saturating_mul(a.into()))
	}
	/// Storage: DexGeneral BootstrapRewards (r:1 w:1)
	/// Proof Skipped: DexGeneral BootstrapRewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:20 w:20)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 10]`.
	fn bootstrap_charge_reward(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1618 + r * (119 ±0)`
		//  Estimated: `6680 + r * (5302 ±0)`
		// Minimum execution time: 72_229_000 picoseconds.
		Weight::from_parts(114_964_818, 6680)
			// Standard Error: 789_997
			.saturating_add(Weight::from_parts(24_835_182, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 5302).saturating_mul(r.into()))
	}
	/// Storage: DexGeneral BootstrapRewards (r:1 w:1)
	/// Proof Skipped: DexGeneral BootstrapRewards (max_values: None, max_size: None, mode: Measured)
	fn bootstrap_withdraw_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1156`
		//  Estimated: `3631`
		// Minimum execution time: 45_610_000 picoseconds.
		Weight::from_parts(45_990_000, 3631)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}