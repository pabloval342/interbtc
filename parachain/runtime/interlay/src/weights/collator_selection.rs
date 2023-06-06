
//! Autogenerated weights for collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `dev-benchmark`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
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
// ../tmp-weight/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for collator_selection using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> collator_selection::WeightInfo for WeightInfo<T> {

	/// Storage: Session NextKeys (r:100 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorSelection Invulnerables (r:0 w:1)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 100]`.
	fn set_invulnerables	(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `217 + b * (78 ±0)`
		//  Estimated: `1207 + b * (2554 ±0)`
		// Minimum execution time: 33_665_000 picoseconds.
		Weight::from_parts(36_494_050, 1207)
			// Standard Error: 24_862
			.saturating_add(Weight::from_parts(6_432_343, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2554).saturating_mul(b.into()))
	}
	/// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	/// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_desired_candidates	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 17_101_000 picoseconds.
		Weight::from_parts(18_058_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	/// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_candidacy_bond	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_328_000 picoseconds.
		Weight::from_parts(18_909_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:1)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(48002), added: 48497, mode: MaxEncodedLen)
	/// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	/// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: CollatorSelection Invulnerables (r:1 w:0)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	/// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Escrow UserPointEpoch (r:1 w:0)
	/// Proof: Escrow UserPointEpoch (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Escrow UserPointHistory (r:1 w:0)
	/// Proof: Escrow UserPointHistory (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Escrow Reserved (r:1 w:1)
	/// Proof: Escrow Reserved (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 999]`.
	fn register_as_candidate	(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2801 + c * (49 ±0)`
		//  Estimated: `73766 + c * (50 ±0)`
		// Minimum execution time: 96_633_000 picoseconds.
		Weight::from_parts(128_853_656, 73766)
			// Standard Error: 3_768
			.saturating_add(Weight::from_parts(280_459, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 50).saturating_mul(c.into()))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:1)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(48002), added: 48497, mode: MaxEncodedLen)
	/// Storage: Escrow Reserved (r:1 w:1)
	/// Proof: Escrow Reserved (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[6, 1000]`.
	fn leave_intent	(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `983 + c * (48 ±0)`
		//  Estimated: `53016`
		// Minimum execution time: 54_720_000 picoseconds.
		Weight::from_parts(71_237_061, 53016)
			// Standard Error: 3_387
			.saturating_add(Weight::from_parts(270_577, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn note_author	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `319`
		//  Estimated: `9763`
		// Minimum execution time: 80_950_000 picoseconds.
		Weight::from_parts(84_539_000, 9763)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:0)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(48002), added: 48497, mode: MaxEncodedLen)
	/// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	/// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:999 w:0)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: Escrow UserPointEpoch (r:999 w:0)
	/// Proof: Escrow UserPointEpoch (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Escrow UserPointHistory (r:999 w:0)
	/// Proof: Escrow UserPointHistory (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: CollatorSelection Invulnerables (r:1 w:0)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Escrow Reserved (r:995 w:995)
	/// Proof: Escrow Reserved (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 1000]`.
	/// The range of component `c` is `[1, 1000]`.
	fn new_session	(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `23641 + r * (50 ±0) + c * (193 ±0)`
		//  Estimated: `58645 + c * (7610 ±0) + r * (2538 ±0)`
		// Minimum execution time: 66_169_000 picoseconds.
		Weight::from_parts(66_765_000, 58645)
			// Standard Error: 1_569_335
			.saturating_add(Weight::from_parts(87_957_359, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 7610).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 2538).saturating_mul(r.into()))
	}
}