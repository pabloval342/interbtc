//! Autogenerated weights for replace
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-08, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/interbtc-standalone
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// replace
// --extrinsic
// *
// --steps
// 100
// --repeat
// 10
// --output
// crates/replace/src/default_weights.rs
// --template
// .deploy/weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for replace.
pub trait WeightInfo {
	fn request_replace() -> Weight;
	fn withdraw_replace() -> Weight;
	fn accept_replace() -> Weight;
	fn execute_replace() -> Weight;
	fn cancel_replace() -> Weight;
	fn set_replace_period() -> Weight;
}

/// Weights for replace using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: Nomination Vaults (r:1 w:0)
	// Storage: Replace ReplaceBtcDustValue (r:1 w:0)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: Fee ReplaceGriefingCollateral (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	fn request_replace() -> Weight {
		Weight::from_ref_time(91_841_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn withdraw_replace() -> Weight {
		Weight::from_ref_time(62_541_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:2 w:2)
	// Storage: VaultRegistry ReservedAddresses (r:1 w:1)
	// Storage: Replace ReplaceBtcDustValue (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: Staking Nonce (r:1 w:0)
	// Storage: Staking Stake (r:1 w:1)
	// Storage: Staking SlashPerToken (r:1 w:0)
	// Storage: Staking SlashTally (r:1 w:1)
	// Storage: Staking TotalStake (r:1 w:1)
	// Storage: Staking TotalCurrentStake (r:1 w:1)
	// Storage: Staking RewardTally (r:1 w:1)
	// Storage: Staking RewardPerToken (r:1 w:0)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: Security Nonce (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: Replace ReplacePeriod (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: Replace ReplaceRequests (r:0 w:1)
	fn accept_replace() -> Weight {
		Weight::from_ref_time(301_210_000 as u64)
			.saturating_add(T::DbWeight::get().reads(22 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Replace ReplaceRequests (r:1 w:1)
	// Storage: VaultRegistry Vaults (r:2 w:2)
	// Storage: BTCRelay DisableInclusionCheck (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: BTCRelay Chains (r:1 w:0)
	// Storage: BTCRelay BlockHeaders (r:1 w:0)
	// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: BTCRelay StableParachainConfirmations (r:1 w:0)
	// Storage: Rewards Stake (r:2 w:2)
	// Storage: Rewards TotalStake (r:1 w:1)
	// Storage: Rewards RewardTally (r:2 w:2)
	// Storage: Rewards RewardPerToken (r:1 w:0)
	fn execute_replace() -> Weight {
		Weight::from_ref_time(188_303_000 as u64)
			.saturating_add(T::DbWeight::get().reads(17 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Replace ReplaceRequests (r:1 w:1)
	// Storage: VaultRegistry Vaults (r:2 w:2)
	// Storage: Replace ReplacePeriod (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: Staking Nonce (r:1 w:0)
	// Storage: Staking TotalCurrentStake (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: Staking Stake (r:1 w:1)
	// Storage: Staking SlashPerToken (r:1 w:0)
	// Storage: Staking SlashTally (r:1 w:1)
	// Storage: Staking TotalStake (r:1 w:1)
	fn cancel_replace() -> Weight {
		Weight::from_ref_time(190_415_000 as u64)
			.saturating_add(T::DbWeight::get().reads(16 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: Replace ReplacePeriod (r:0 w:1)
	fn set_replace_period() -> Weight {
		Weight::from_ref_time(3_247_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: Nomination Vaults (r:1 w:0)
	// Storage: Replace ReplaceBtcDustValue (r:1 w:0)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: Fee ReplaceGriefingCollateral (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	fn request_replace() -> Weight {
		Weight::from_ref_time(91_841_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn withdraw_replace() -> Weight {
		Weight::from_ref_time(62_541_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:2 w:2)
	// Storage: VaultRegistry ReservedAddresses (r:1 w:1)
	// Storage: Replace ReplaceBtcDustValue (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: Staking Nonce (r:1 w:0)
	// Storage: Staking Stake (r:1 w:1)
	// Storage: Staking SlashPerToken (r:1 w:0)
	// Storage: Staking SlashTally (r:1 w:1)
	// Storage: Staking TotalStake (r:1 w:1)
	// Storage: Staking TotalCurrentStake (r:1 w:1)
	// Storage: Staking RewardTally (r:1 w:1)
	// Storage: Staking RewardPerToken (r:1 w:0)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: Security Nonce (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: Replace ReplacePeriod (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: Replace ReplaceRequests (r:0 w:1)
	fn accept_replace() -> Weight {
		Weight::from_ref_time(301_210_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(22 as u64))
			.saturating_add(RocksDbWeight::get().writes(12 as u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Replace ReplaceRequests (r:1 w:1)
	// Storage: VaultRegistry Vaults (r:2 w:2)
	// Storage: BTCRelay DisableInclusionCheck (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: BTCRelay Chains (r:1 w:0)
	// Storage: BTCRelay BlockHeaders (r:1 w:0)
	// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: BTCRelay StableParachainConfirmations (r:1 w:0)
	// Storage: Rewards Stake (r:2 w:2)
	// Storage: Rewards TotalStake (r:1 w:1)
	// Storage: Rewards RewardTally (r:2 w:2)
	// Storage: Rewards RewardPerToken (r:1 w:0)
	fn execute_replace() -> Weight {
		Weight::from_ref_time(188_303_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(17 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Replace ReplaceRequests (r:1 w:1)
	// Storage: VaultRegistry Vaults (r:2 w:2)
	// Storage: Replace ReplacePeriod (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: Staking Nonce (r:1 w:0)
	// Storage: Staking TotalCurrentStake (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: Staking Stake (r:1 w:1)
	// Storage: Staking SlashPerToken (r:1 w:0)
	// Storage: Staking SlashTally (r:1 w:1)
	// Storage: Staking TotalStake (r:1 w:1)
	fn cancel_replace() -> Weight {
		Weight::from_ref_time(190_415_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(16 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	// Storage: Replace ReplacePeriod (r:0 w:1)
	fn set_replace_period() -> Weight {
		Weight::from_ref_time(3_247_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}

