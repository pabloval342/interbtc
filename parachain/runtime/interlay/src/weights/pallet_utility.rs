
//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-07, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `interlay-rust-runner-2mz2v-jrrg4`, CPU: `AMD EPYC 7502P 32-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("interlay-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// interlay-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 10
// --output
// parachain/runtime/interlay/src/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_utility using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {

	/// The range of component `c` is `[0, 1000]`.
	fn batch	(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_637_000 picoseconds.
		Weight::from_parts(14_143_347, 0)
			// Standard Error: 7_460
			.saturating_add(Weight::from_parts(10_531_820, 0).saturating_mul(c.into()))
	}
	fn as_derivative	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_812_000 picoseconds.
		Weight::from_parts(11_162_000, 0)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all	(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_567_000 picoseconds.
		Weight::from_parts(92_452_234, 0)
			// Standard Error: 81_966
			.saturating_add(Weight::from_parts(11_112_221, 0).saturating_mul(c.into()))
	}
	fn dispatch_as	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_156_000 picoseconds.
		Weight::from_parts(18_417_000, 0)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch	(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_497_000 picoseconds.
		Weight::from_parts(13_667_000, 0)
			// Standard Error: 70_076
			.saturating_add(Weight::from_parts(10_839_741, 0).saturating_mul(c.into()))
	}
}