
//! Autogenerated weights for dex_stable
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-27, STEPS: `100`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `enterprise`, CPU: `Intel(R) Core(TM) i7-9700K CPU @ 3.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// dex-stable
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
// crates/dex-stable/src/default_weights.rs
// --template
// .deploy/default-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for dex_stable.
pub trait WeightInfo {
	fn create_base_pool(b: u32, s: u32, ) -> Weight;
	fn create_meta_pool(m: u32, s: u32, ) -> Weight;
	fn add_liquidity(b: u32, ) -> Weight;
	fn swap() -> Weight;
	fn remove_liquidity(b: u32, ) -> Weight;
	fn remove_liquidity_one_currency() -> Weight;
	fn remove_liquidity_imbalance(b: u32, ) -> Weight;
	fn add_pool_and_base_pool_liquidity(b: u32, m: u32, ) -> Weight;
	fn remove_pool_and_base_pool_liquidity(b: u32, m: u32, ) -> Weight;
	fn remove_pool_and_base_pool_liquidity_one_currency() -> Weight;
	fn swap_pool_from_base() -> Weight;
	fn swap_pool_to_base() -> Weight;
	fn swap_meta_pool_underlying() -> Weight;
	fn withdraw_admin_fee() -> Weight;
}

/// Weights for dex_stable using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: DexStable NextPoolId (r:1 w:1)
	/// Proof: DexStable NextPoolId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: DexStable LpCurrencies (r:1 w:1)
	/// Proof: DexStable LpCurrencies (max_values: None, max_size: Some(31), added: 2506, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	/// The range of component `s` is `[0, 50]`.
	fn create_base_pool(b: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1257`
		//  Estimated: `8899`
		// Minimum execution time: 46_508_000 picoseconds.
		Weight::from_parts(56_808_830, 8899)
			// Standard Error: 548_424
			.saturating_add(Weight::from_parts(8_832_018, 0).saturating_mul(b.into()))
			// Standard Error: 93_664
			.saturating_add(Weight::from_parts(378_381, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: DexStable LpCurrencies (r:2 w:1)
	/// Proof: DexStable LpCurrencies (max_values: None, max_size: Some(31), added: 2506, mode: MaxEncodedLen)
	/// Storage: DexStable NextPoolId (r:1 w:1)
	/// Proof: DexStable NextPoolId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DexStable Pools (r:2 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `s` is `[0, 50]`.
	fn create_meta_pool(m: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2535`
		//  Estimated: `17709`
		// Minimum execution time: 85_453_000 picoseconds.
		Weight::from_parts(119_901_479, 17709)
			// Standard Error: 270_051
			.saturating_add(Weight::from_parts(767_630, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:21 w:21)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	fn add_liquidity(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2261 + b * (156 ±0)`
		//  Estimated: `11497 + b * (5180 ±0)`
		// Minimum execution time: 138_770_000 picoseconds.
		Weight::from_parts(117_368_533, 11497)
			// Standard Error: 1_611_993
			.saturating_add(Weight::from_parts(41_756_999, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3545`
		//  Estimated: `16757`
		// Minimum execution time: 167_499_000 picoseconds.
		Weight::from_parts(168_870_000, 16757)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:21 w:21)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	fn remove_liquidity(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2268 + b * (254 ±0)`
		//  Estimated: `10994 + b * (5180 ±0)`
		// Minimum execution time: 115_293_000 picoseconds.
		Weight::from_parts(107_102_308, 10994)
			// Standard Error: 291_001
			.saturating_add(Weight::from_parts(17_781_961, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_liquidity_one_currency() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3478`
		//  Estimated: `16677`
		// Minimum execution time: 140_894_000 picoseconds.
		Weight::from_parts(142_052_000, 16677)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:21 w:21)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	fn remove_liquidity_imbalance(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2310 + b * (254 ±0)`
		//  Estimated: `11497 + b * (5180 ±0)`
		// Minimum execution time: 135_935_000 picoseconds.
		Weight::from_parts(180_491_462, 11497)
			// Standard Error: 891_745
			.saturating_add(Weight::from_parts(16_288_535, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:41 w:41)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	/// The range of component `m` is `[2, 10]`.
	fn add_pool_and_base_pool_liquidity(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2855 + b * (249 ±0) + m * (148 ±0)`
		//  Estimated: `17298 + b * (5180 ±0) + m * (5180 ±0)`
		// Minimum execution time: 456_171_000 picoseconds.
		Weight::from_parts(118_944_947, 17298)
			// Standard Error: 60_889
			.saturating_add(Weight::from_parts(27_809_259, 0).saturating_mul(b.into()))
			// Standard Error: 60_889
			.saturating_add(Weight::from_parts(32_816_221, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(m.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(m.into()))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:41 w:41)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	/// The range of component `m` is `[2, 10]`.
	fn remove_pool_and_base_pool_liquidity(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2996 + b * (249 ±0) + m * (246 ±0)`
		//  Estimated: `19398 + b * (5180 ±0) + m * (5180 ±0)`
		// Minimum execution time: 365_549_000 picoseconds.
		Weight::from_parts(139_954_949, 19398)
			// Standard Error: 49_475
			.saturating_add(Weight::from_parts(19_722_177, 0).saturating_mul(b.into()))
			// Standard Error: 49_475
			.saturating_add(Weight::from_parts(19_555_412, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(5_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(m.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(m.into()))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:5 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_pool_and_base_pool_liquidity_one_currency() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5442`
		//  Estimated: `30261`
		// Minimum execution time: 251_896_000 picoseconds.
		Weight::from_parts(253_213_000, 30261)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:15 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap_pool_from_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6122`
		//  Estimated: `51048`
		// Minimum execution time: 304_620_000 picoseconds.
		Weight::from_parts(306_614_000, 51048)
			.saturating_add(T::DbWeight::get().reads(20_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:6 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	fn swap_pool_to_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5638`
		//  Estimated: `30341`
		// Minimum execution time: 240_873_000 picoseconds.
		Weight::from_parts(242_718_000, 30341)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap_meta_pool_underlying() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3930`
		//  Estimated: `16757`
		// Minimum execution time: 130_616_000 picoseconds.
		Weight::from_parts(131_228_000, 16757)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:10 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	fn withdraw_admin_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3179`
		//  Estimated: `29191`
		// Minimum execution time: 126_289_000 picoseconds.
		Weight::from_parts(127_157_000, 29191)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: DexStable NextPoolId (r:1 w:1)
	/// Proof: DexStable NextPoolId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: DexStable LpCurrencies (r:1 w:1)
	/// Proof: DexStable LpCurrencies (max_values: None, max_size: Some(31), added: 2506, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	/// The range of component `s` is `[0, 50]`.
	fn create_base_pool(b: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1257`
		//  Estimated: `8899`
		// Minimum execution time: 46_508_000 picoseconds.
		Weight::from_parts(56_808_830, 8899)
			// Standard Error: 548_424
			.saturating_add(Weight::from_parts(8_832_018, 0).saturating_mul(b.into()))
			// Standard Error: 93_664
			.saturating_add(Weight::from_parts(378_381, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: DexStable LpCurrencies (r:2 w:1)
	/// Proof: DexStable LpCurrencies (max_values: None, max_size: Some(31), added: 2506, mode: MaxEncodedLen)
	/// Storage: DexStable NextPoolId (r:1 w:1)
	/// Proof: DexStable NextPoolId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DexStable Pools (r:2 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `s` is `[0, 50]`.
	fn create_meta_pool(m: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2535`
		//  Estimated: `17709`
		// Minimum execution time: 85_453_000 picoseconds.
		Weight::from_parts(119_901_479, 17709)
			// Standard Error: 270_051
			.saturating_add(Weight::from_parts(767_630, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:21 w:21)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	fn add_liquidity(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2261 + b * (156 ±0)`
		//  Estimated: `11497 + b * (5180 ±0)`
		// Minimum execution time: 138_770_000 picoseconds.
		Weight::from_parts(117_368_533, 11497)
			// Standard Error: 1_611_993
			.saturating_add(Weight::from_parts(41_756_999, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3545`
		//  Estimated: `16757`
		// Minimum execution time: 167_499_000 picoseconds.
		Weight::from_parts(168_870_000, 16757)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:21 w:21)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	fn remove_liquidity(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2268 + b * (254 ±0)`
		//  Estimated: `10994 + b * (5180 ±0)`
		// Minimum execution time: 115_293_000 picoseconds.
		Weight::from_parts(107_102_308, 10994)
			// Standard Error: 291_001
			.saturating_add(Weight::from_parts(17_781_961, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_liquidity_one_currency() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3478`
		//  Estimated: `16677`
		// Minimum execution time: 140_894_000 picoseconds.
		Weight::from_parts(142_052_000, 16677)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:21 w:21)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	fn remove_liquidity_imbalance(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2310 + b * (254 ±0)`
		//  Estimated: `11497 + b * (5180 ±0)`
		// Minimum execution time: 135_935_000 picoseconds.
		Weight::from_parts(180_491_462, 11497)
			// Standard Error: 891_745
			.saturating_add(Weight::from_parts(16_288_535, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:41 w:41)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	/// The range of component `m` is `[2, 10]`.
	fn add_pool_and_base_pool_liquidity(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2855 + b * (249 ±0) + m * (148 ±0)`
		//  Estimated: `17298 + b * (5180 ±0) + m * (5180 ±0)`
		// Minimum execution time: 456_171_000 picoseconds.
		Weight::from_parts(118_944_947, 17298)
			// Standard Error: 60_889
			.saturating_add(Weight::from_parts(27_809_259, 0).saturating_mul(b.into()))
			// Standard Error: 60_889
			.saturating_add(Weight::from_parts(32_816_221, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(m.into())))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(m.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(m.into()))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:41 w:41)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	/// The range of component `m` is `[2, 10]`.
	fn remove_pool_and_base_pool_liquidity(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2996 + b * (249 ±0) + m * (246 ±0)`
		//  Estimated: `19398 + b * (5180 ±0) + m * (5180 ±0)`
		// Minimum execution time: 365_549_000 picoseconds.
		Weight::from_parts(139_954_949, 19398)
			// Standard Error: 49_475
			.saturating_add(Weight::from_parts(19_722_177, 0).saturating_mul(b.into()))
			// Standard Error: 49_475
			.saturating_add(Weight::from_parts(19_555_412, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(m.into())))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(m.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(m.into()))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:5 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_pool_and_base_pool_liquidity_one_currency() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5442`
		//  Estimated: `30261`
		// Minimum execution time: 251_896_000 picoseconds.
		Weight::from_parts(253_213_000, 30261)
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:15 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap_pool_from_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6122`
		//  Estimated: `51048`
		// Minimum execution time: 304_620_000 picoseconds.
		Weight::from_parts(306_614_000, 51048)
			.saturating_add(RocksDbWeight::get().reads(20_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:6 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	fn swap_pool_to_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5638`
		//  Estimated: `30341`
		// Minimum execution time: 240_873_000 picoseconds.
		Weight::from_parts(242_718_000, 30341)
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap_meta_pool_underlying() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3930`
		//  Estimated: `16757`
		// Minimum execution time: 130_616_000 picoseconds.
		Weight::from_parts(131_228_000, 16757)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:10 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	fn withdraw_admin_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3179`
		//  Estimated: `29191`
		// Minimum execution time: 126_289_000 picoseconds.
		Weight::from_parts(127_157_000, 29191)
			.saturating_add(RocksDbWeight::get().reads(11_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
