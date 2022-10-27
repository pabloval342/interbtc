//! Autogenerated weights for annuity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-07, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-standalone
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// annuity
// --extrinsic
// *
// --steps
// 100
// --repeat
// 10
// --output
// ./crates/annuity/src/default_weights.rs
// --template
// .deploy/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for annuity.
pub trait WeightInfo {
	fn withdraw_rewards() -> Weight;
	fn update_rewards() -> Weight;
	fn set_reward_per_wrapped() -> Weight;
}

/// Weights for annuity using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: EscrowRewards Stake (r:1 w:0)
	// Storage: EscrowRewards RewardPerToken (r:1 w:0)
	// Storage: EscrowRewards RewardTally (r:1 w:1)
	// Storage: EscrowRewards TotalRewards (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:0)
	fn withdraw_rewards() -> Weight {
		Weight::from_ref_time(42_255_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Tokens Accounts (r:1 w:0)
	// Storage: EscrowAnnuity RewardPerBlock (r:0 w:1)
	fn update_rewards() -> Weight {
		Weight::from_ref_time(9_601_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: EscrowAnnuity RewardPerWrapped (r:0 w:1)
	fn set_reward_per_wrapped() -> Weight {
		Weight::from_ref_time(3_288_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: EscrowRewards Stake (r:1 w:0)
	// Storage: EscrowRewards RewardPerToken (r:1 w:0)
	// Storage: EscrowRewards RewardTally (r:1 w:1)
	// Storage: EscrowRewards TotalRewards (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:0)
	fn withdraw_rewards() -> Weight {
		Weight::from_ref_time(42_255_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Tokens Accounts (r:1 w:0)
	// Storage: EscrowAnnuity RewardPerBlock (r:0 w:1)
	fn update_rewards() -> Weight {
		Weight::from_ref_time(9_601_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: EscrowAnnuity RewardPerWrapped (r:0 w:1)
	fn set_reward_per_wrapped() -> Weight {
		Weight::from_ref_time(3_288_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
