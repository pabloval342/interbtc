
//! Autogenerated weights for supply
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-26, STEPS: `100`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `enterprise`, CPU: `Intel(R) Core(TM) i7-9700K CPU @ 3.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// supply
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
// crates/supply/src/default_weights.rs
// --template
// .deploy/default-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for supply.
pub trait WeightInfo {
	fn on_initialize() -> Weight;
	fn set_start_height_and_inflation() -> Weight;
}

/// Weights for supply using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Supply StartHeight (r:1 w:1)
	/// Proof: Supply StartHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Supply Inflation (r:1 w:0)
	/// Proof: Supply Inflation (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VaultAnnuity RewardPerBlock (r:0 w:1)
	/// Proof: VaultAnnuity RewardPerBlock (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Supply LastEmission (r:0 w:1)
	/// Proof: Supply LastEmission (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: EscrowAnnuity RewardPerBlock (r:0 w:1)
	/// Proof: EscrowAnnuity RewardPerBlock (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1340`
		//  Estimated: `19086`
		// Minimum execution time: 129_875_000 picoseconds.
		Weight::from_parts(130_869_000, 19086)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(11_u64))
	}
	/// Storage: Supply StartHeight (r:0 w:1)
	/// Proof: Supply StartHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Supply Inflation (r:0 w:1)
	/// Proof: Supply Inflation (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_start_height_and_inflation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `711`
		//  Estimated: `0`
		// Minimum execution time: 9_566_000 picoseconds.
		Weight::from_parts(9_916_000, 0)
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Supply StartHeight (r:1 w:1)
	/// Proof: Supply StartHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Supply Inflation (r:1 w:0)
	/// Proof: Supply Inflation (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VaultAnnuity RewardPerBlock (r:0 w:1)
	/// Proof: VaultAnnuity RewardPerBlock (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Supply LastEmission (r:0 w:1)
	/// Proof: Supply LastEmission (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: EscrowAnnuity RewardPerBlock (r:0 w:1)
	/// Proof: EscrowAnnuity RewardPerBlock (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1340`
		//  Estimated: `19086`
		// Minimum execution time: 129_875_000 picoseconds.
		Weight::from_parts(130_869_000, 19086)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(11_u64))
	}
	/// Storage: Supply StartHeight (r:0 w:1)
	/// Proof: Supply StartHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Supply Inflation (r:0 w:1)
	/// Proof: Supply Inflation (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_start_height_and_inflation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `711`
		//  Estimated: `0`
		// Minimum execution time: 9_566_000 picoseconds.
		Weight::from_parts(9_916_000, 0)
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
