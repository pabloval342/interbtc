
//! Autogenerated weights for orml_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-01, STEPS: `10`, REPEAT: `5`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `dev-benchmark`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// kintsugi-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 10
// --repeat
// 5
// --output
// ../tmp-weight/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for orml_vesting using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> orml_vesting::WeightInfo for WeightInfo<T> {

	/// Storage: Vesting VestingSchedules (r:1 w:1)
	/// Proof: Vesting VestingSchedules (max_values: None, max_size: Some(77), added: 2552, mode: MaxEncodedLen)
	/// Storage: Tokens Locks (r:1 w:1)
	/// Proof: Tokens Locks (max_values: None, max_size: Some(1268), added: 3743, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1]`.
	fn claim	(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315 + n * (306 ±0)`
		//  Estimated: `15448`
		// Minimum execution time: 85_979_000 picoseconds.
		Weight::from_parts(111_109_521, 15448)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Vesting VestingSchedules (r:1 w:1)
	/// Proof: Vesting VestingSchedules (max_values: None, max_size: Some(77), added: 2552, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Locks (r:1 w:1)
	/// Proof: Tokens Locks (max_values: None, max_size: Some(1268), added: 3743, mode: MaxEncodedLen)
	fn vested_transfer	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `395`
		//  Estimated: `18038`
		// Minimum execution time: 157_673_000 picoseconds.
		Weight::from_parts(167_432_000, 18038)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens Locks (r:1 w:1)
	/// Proof: Tokens Locks (max_values: None, max_size: Some(1268), added: 3743, mode: MaxEncodedLen)
	/// Storage: Vesting VestingSchedules (r:0 w:1)
	/// Proof: Vesting VestingSchedules (max_values: None, max_size: Some(77), added: 2552, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1]`.
	fn update_vesting_schedules	(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `560`
		//  Estimated: `11906`
		// Minimum execution time: 87_594_000 picoseconds.
		Weight::from_parts(94_841_478, 11906)
			// Standard Error: 2_921_497
			.saturating_add(Weight::from_parts(3_419_855, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}