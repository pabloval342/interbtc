//! Autogenerated weights for vault_registry
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-13, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/interbtc-standalone
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// vault-registry
// --extrinsic
// *
// --steps
// 100
// --repeat
// 10
// --output
// crates/vault-registry/src/default_weights.rs
// --template
// .deploy/weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for vault_registry.
pub trait WeightInfo {
	fn register_vault() -> Weight;
	fn deposit_collateral() -> Weight;
	fn withdraw_collateral() -> Weight;
	fn update_public_key() -> Weight;
	fn register_address() -> Weight;
	fn accept_new_issues() -> Weight;
	fn set_minimum_collateral() -> Weight;
	fn set_system_collateral_ceiling() -> Weight;
	fn set_secure_collateral_threshold() -> Weight;
	fn set_premium_redeem_threshold() -> Weight;
	fn set_liquidation_collateral_threshold() -> Weight;
	fn report_undercollateralized_vault() -> Weight;
}

/// Weights for vault_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: VaultRegistry MinimumCollateralVault (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: VaultRegistry SystemCollateralCeiling (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: VaultStaking Nonce (r:1 w:0)
	// Storage: VaultStaking Stake (r:1 w:1)
	// Storage: VaultStaking SlashPerToken (r:1 w:0)
	// Storage: VaultStaking SlashTally (r:1 w:1)
	// Storage: VaultStaking TotalStake (r:1 w:1)
	// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	// Storage: VaultStaking RewardTally (r:2 w:2)
	// Storage: VaultStaking RewardPerToken (r:2 w:0)
	fn register_vault() -> Weight {
		(136_098_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(15 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: VaultRegistry Vaults (r:1 w:0)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: VaultRegistry SystemCollateralCeiling (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: VaultStaking Nonce (r:1 w:0)
	// Storage: VaultStaking Stake (r:1 w:1)
	// Storage: VaultStaking SlashPerToken (r:1 w:0)
	// Storage: VaultStaking SlashTally (r:1 w:1)
	// Storage: VaultStaking TotalStake (r:1 w:1)
	// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	// Storage: VaultStaking RewardTally (r:2 w:2)
	// Storage: VaultStaking RewardPerToken (r:2 w:0)
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	fn deposit_collateral() -> Weight {
		(167_605_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(17 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: VaultRegistry Vaults (r:1 w:0)
	// Storage: VaultStaking Nonce (r:1 w:0)
	// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: VaultStaking Stake (r:1 w:1)
	// Storage: VaultStaking SlashPerToken (r:1 w:0)
	// Storage: VaultStaking SlashTally (r:1 w:1)
	// Storage: VaultRegistry PremiumRedeemThreshold (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: VaultStaking TotalStake (r:1 w:1)
	// Storage: VaultStaking RewardTally (r:2 w:2)
	// Storage: VaultStaking RewardPerToken (r:2 w:0)
	fn withdraw_collateral() -> Weight {
		(175_284_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(17 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: VaultRegistry Vaults (r:1 w:1)
	fn update_public_key() -> Weight {
		(27_632_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry ReservedAddresses (r:1 w:1)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	fn register_address() -> Weight {
		(35_385_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: VaultRegistry Vaults (r:1 w:1)
	fn accept_new_issues() -> Weight {
		(14_679_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry MinimumCollateralVault (r:0 w:1)
	fn set_minimum_collateral() -> Weight {
		(4_059_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry SystemCollateralCeiling (r:0 w:1)
	fn set_system_collateral_ceiling() -> Weight {
		(4_101_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry SecureCollateralThreshold (r:0 w:1)
	fn set_secure_collateral_threshold() -> Weight {
		(4_216_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry PremiumRedeemThreshold (r:0 w:1)
	fn set_premium_redeem_threshold() -> Weight {
		(4_132_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry LiquidationCollateralThreshold (r:0 w:1)
	fn set_liquidation_collateral_threshold() -> Weight {
		(4_130_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: VaultRegistry LiquidationCollateralThreshold (r:1 w:0)
	// Storage: VaultStaking Nonce (r:1 w:0)
	// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: VaultStaking Stake (r:1 w:1)
	// Storage: VaultStaking SlashPerToken (r:1 w:0)
	// Storage: VaultStaking SlashTally (r:1 w:1)
	// Storage: VaultStaking TotalStake (r:1 w:1)
	// Storage: VaultStaking RewardTally (r:2 w:2)
	// Storage: VaultStaking RewardPerToken (r:2 w:0)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:1)
	// Storage: VaultRegistry SystemCollateralCeiling (r:1 w:0)
	// Storage: VaultRegistry LiquidationVault (r:1 w:1)
	// Storage: VaultRewards Stake (r:1 w:1)
	// Storage: VaultRewards TotalStake (r:1 w:1)
	// Storage: VaultRewards RewardTally (r:2 w:2)
	// Storage: VaultRewards RewardPerToken (r:2 w:0)
	fn report_undercollateralized_vault() -> Weight {
		(363_346_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(28 as Weight))
			.saturating_add(T::DbWeight::get().writes(16 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: VaultRegistry MinimumCollateralVault (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: VaultRegistry SystemCollateralCeiling (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: VaultStaking Nonce (r:1 w:0)
	// Storage: VaultStaking Stake (r:1 w:1)
	// Storage: VaultStaking SlashPerToken (r:1 w:0)
	// Storage: VaultStaking SlashTally (r:1 w:1)
	// Storage: VaultStaking TotalStake (r:1 w:1)
	// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	// Storage: VaultStaking RewardTally (r:2 w:2)
	// Storage: VaultStaking RewardPerToken (r:2 w:0)
	fn register_vault() -> Weight {
		(136_098_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(15 as Weight))
			.saturating_add(RocksDbWeight::get().writes(9 as Weight))
	}
	// Storage: VaultRegistry Vaults (r:1 w:0)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: VaultRegistry SystemCollateralCeiling (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: VaultStaking Nonce (r:1 w:0)
	// Storage: VaultStaking Stake (r:1 w:1)
	// Storage: VaultStaking SlashPerToken (r:1 w:0)
	// Storage: VaultStaking SlashTally (r:1 w:1)
	// Storage: VaultStaking TotalStake (r:1 w:1)
	// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	// Storage: VaultStaking RewardTally (r:2 w:2)
	// Storage: VaultStaking RewardPerToken (r:2 w:0)
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	fn deposit_collateral() -> Weight {
		(167_605_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(17 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	// Storage: VaultRegistry Vaults (r:1 w:0)
	// Storage: VaultStaking Nonce (r:1 w:0)
	// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: VaultStaking Stake (r:1 w:1)
	// Storage: VaultStaking SlashPerToken (r:1 w:0)
	// Storage: VaultStaking SlashTally (r:1 w:1)
	// Storage: VaultRegistry PremiumRedeemThreshold (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: VaultStaking TotalStake (r:1 w:1)
	// Storage: VaultStaking RewardTally (r:2 w:2)
	// Storage: VaultStaking RewardPerToken (r:2 w:0)
	fn withdraw_collateral() -> Weight {
		(175_284_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(17 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	// Storage: VaultRegistry Vaults (r:1 w:1)
	fn update_public_key() -> Weight {
		(27_632_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry ReservedAddresses (r:1 w:1)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	fn register_address() -> Weight {
		(35_385_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: VaultRegistry Vaults (r:1 w:1)
	fn accept_new_issues() -> Weight {
		(14_679_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry MinimumCollateralVault (r:0 w:1)
	fn set_minimum_collateral() -> Weight {
		(4_059_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry SystemCollateralCeiling (r:0 w:1)
	fn set_system_collateral_ceiling() -> Weight {
		(4_101_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry SecureCollateralThreshold (r:0 w:1)
	fn set_secure_collateral_threshold() -> Weight {
		(4_216_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry PremiumRedeemThreshold (r:0 w:1)
	fn set_premium_redeem_threshold() -> Weight {
		(4_132_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry LiquidationCollateralThreshold (r:0 w:1)
	fn set_liquidation_collateral_threshold() -> Weight {
		(4_130_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: VaultRegistry LiquidationCollateralThreshold (r:1 w:0)
	// Storage: VaultStaking Nonce (r:1 w:0)
	// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: VaultStaking Stake (r:1 w:1)
	// Storage: VaultStaking SlashPerToken (r:1 w:0)
	// Storage: VaultStaking SlashTally (r:1 w:1)
	// Storage: VaultStaking TotalStake (r:1 w:1)
	// Storage: VaultStaking RewardTally (r:2 w:2)
	// Storage: VaultStaking RewardPerToken (r:2 w:0)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:1)
	// Storage: VaultRegistry SystemCollateralCeiling (r:1 w:0)
	// Storage: VaultRegistry LiquidationVault (r:1 w:1)
	// Storage: VaultRewards Stake (r:1 w:1)
	// Storage: VaultRewards TotalStake (r:1 w:1)
	// Storage: VaultRewards RewardTally (r:2 w:2)
	// Storage: VaultRewards RewardPerToken (r:2 w:0)
	fn report_undercollateralized_vault() -> Weight {
		(363_346_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(28 as Weight))
			.saturating_add(RocksDbWeight::get().writes(16 as Weight))
	}
}

