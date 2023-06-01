
//! Autogenerated weights for tx_pause
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

/// Weights for tx_pause using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> tx_pause::WeightInfo for WeightInfo<T> {

	/// Storage: TxPause PausedCalls (r:2 w:1)
	/// Proof: TxPause PausedCalls (max_values: None, max_size: Some(277), added: 2752, mode: MaxEncodedLen)
	fn pause	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `6494`
		// Minimum execution time: 42_129_000 picoseconds.
		Weight::from_parts(43_235_000, 6494)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: TxPause PausedCalls (r:2 w:1)
	/// Proof: TxPause PausedCalls (max_values: None, max_size: Some(277), added: 2752, mode: MaxEncodedLen)
	fn unpause	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `308`
		//  Estimated: `6494`
		// Minimum execution time: 48_153_000 picoseconds.
		Weight::from_parts(49_274_000, 6494)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}