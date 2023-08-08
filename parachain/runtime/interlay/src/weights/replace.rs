
//! Autogenerated weights for replace
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

/// Weights for replace using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> replace::WeightInfo for WeightInfo<T> {

	/// Storage: VaultRegistry Vaults (r:1 w:1)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: Nomination Vaults (r:1 w:0)
	/// Proof: Nomination Vaults (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	/// Storage: Replace ReplaceBtcDustValue (r:1 w:0)
	/// Proof: Replace ReplaceBtcDustValue (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Oracle Aggregate (r:1 w:0)
	/// Proof: Oracle Aggregate (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: Fee ReplaceGriefingCollateral (r:1 w:0)
	/// Proof: Fee ReplaceGriefingCollateral (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	fn request_replace	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2035`
		//  Estimated: `3725`
		// Minimum execution time: 131_042_000 picoseconds.
		Weight::from_parts(135_822_000, 3725)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: VaultRegistry Vaults (r:1 w:1)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	fn withdraw_replace	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `701`
		//  Estimated: `3725`
		// Minimum execution time: 55_932_000 picoseconds.
		Weight::from_parts(58_086_000, 3725)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: VaultRegistry Vaults (r:2 w:2)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: Replace ReplaceBtcDustValue (r:1 w:0)
	/// Proof: Replace ReplaceBtcDustValue (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: VaultCapacity Stake (r:1 w:1)
	/// Proof: VaultCapacity Stake (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultCapacity RewardPerToken (r:2 w:0)
	/// Proof: VaultCapacity RewardPerToken (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: VaultCapacity RewardTally (r:2 w:2)
	/// Proof: VaultCapacity RewardTally (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	/// Storage: VaultCapacity TotalRewards (r:2 w:2)
	/// Proof: VaultCapacity TotalRewards (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultRewards Stake (r:1 w:1)
	/// Proof: VaultRewards Stake (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	/// Storage: VaultRewards RewardPerToken (r:2 w:0)
	/// Proof: VaultRewards RewardPerToken (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	/// Storage: VaultRewards RewardTally (r:2 w:2)
	/// Proof: VaultRewards RewardTally (max_values: None, max_size: Some(124), added: 2599, mode: MaxEncodedLen)
	/// Storage: VaultRewards TotalRewards (r:2 w:2)
	/// Proof: VaultRewards TotalRewards (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Fee Commission (r:1 w:0)
	/// Proof: Fee Commission (max_values: None, max_size: Some(86), added: 2561, mode: MaxEncodedLen)
	/// Storage: VaultStaking Nonce (r:1 w:0)
	/// Proof: VaultStaking Nonce (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	/// Proof: VaultStaking TotalCurrentStake (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: VaultStaking RewardPerToken (r:2 w:2)
	/// Proof: VaultStaking RewardPerToken (max_values: None, max_size: Some(117), added: 2592, mode: MaxEncodedLen)
	/// Storage: VaultStaking Stake (r:1 w:1)
	/// Proof: VaultStaking Stake (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: VaultStaking SlashPerToken (r:1 w:0)
	/// Proof: VaultStaking SlashPerToken (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: VaultStaking SlashTally (r:1 w:1)
	/// Proof: VaultStaking SlashTally (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: VaultStaking TotalStake (r:1 w:1)
	/// Proof: VaultStaking TotalStake (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: VaultStaking RewardTally (r:2 w:2)
	/// Proof: VaultStaking RewardTally (max_values: None, max_size: Some(149), added: 2624, mode: MaxEncodedLen)
	/// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	/// Proof: VaultRegistry SecureCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultRewards TotalStake (r:1 w:1)
	/// Proof: VaultRewards TotalStake (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultRewards RewardCurrencies (r:1 w:0)
	/// Proof: VaultRewards RewardCurrencies (max_values: None, max_size: Some(50), added: 2525, mode: MaxEncodedLen)
	/// Storage: Oracle Aggregate (r:1 w:0)
	/// Proof: Oracle Aggregate (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: VaultCapacity TotalStake (r:1 w:1)
	/// Proof: VaultCapacity TotalStake (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: VaultCapacity RewardCurrencies (r:1 w:0)
	/// Proof: VaultCapacity RewardCurrencies (max_values: None, max_size: Some(39), added: 2514, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	/// Proof: VaultRegistry TotalUserVaultCollateral (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultRegistry SystemCollateralCeiling (r:1 w:0)
	/// Proof: VaultRegistry SystemCollateralCeiling (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: Security Nonce (r:1 w:1)
	/// Proof: Security Nonce (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: System ParentHash (r:1 w:0)
	/// Proof: System ParentHash (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Replace ReplacePeriod (r:1 w:0)
	/// Proof: Replace ReplacePeriod (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Replace ReplaceRequests (r:0 w:1)
	/// Proof: Replace ReplaceRequests (max_values: None, max_size: Some(250), added: 2725, mode: MaxEncodedLen)
	fn accept_replace	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4306`
		//  Estimated: `6460`
		// Minimum execution time: 623_749_000 picoseconds.
		Weight::from_parts(628_388_000, 6460)
			.saturating_add(T::DbWeight::get().reads(42_u64))
			.saturating_add(T::DbWeight::get().writes(26_u64))
	}
	/// Storage: Replace ReplaceRequests (r:1 w:1)
	/// Proof: Replace ReplaceRequests (max_values: None, max_size: Some(250), added: 2725, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableInclusionCheck (r:1 w:0)
	/// Proof: BTCRelay DisableInclusionCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:1 w:0)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:1 w:0)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	/// Proof: BTCRelay StableBitcoinConfirmations (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay StableParachainConfirmations (r:1 w:0)
	/// Proof: BTCRelay StableParachainConfirmations (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VaultRegistry Vaults (r:2 w:2)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `h` is `[2, 10]`.
	/// The range of component `i` is `[1, 10]`.
	/// The range of component `o` is `[2, 3]`.
	/// The range of component `b` is `[541, 2048]`.
	fn execute_pending_replace	(h: u32, i: u32, o: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3962`
		//  Estimated: `6460`
		// Minimum execution time: 201_203_000 picoseconds.
		Weight::from_parts(166_625_346, 6460)
			// Standard Error: 77_399
			.saturating_add(Weight::from_parts(3_797_260, 0).saturating_mul(h.into()))
			// Standard Error: 69_776
			.saturating_add(Weight::from_parts(1_249_996, 0).saturating_mul(i.into()))
			// Standard Error: 422_991
			.saturating_add(Weight::from_parts(3_113_568, 0).saturating_mul(o.into()))
			// Standard Error: 426
			.saturating_add(Weight::from_parts(3_662, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Replace ReplaceRequests (r:1 w:1)
	/// Proof: Replace ReplaceRequests (max_values: None, max_size: Some(250), added: 2725, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableInclusionCheck (r:1 w:0)
	/// Proof: BTCRelay DisableInclusionCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:1 w:0)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:1 w:0)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	/// Proof: BTCRelay StableBitcoinConfirmations (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay StableParachainConfirmations (r:1 w:0)
	/// Proof: BTCRelay StableParachainConfirmations (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VaultRegistry Vaults (r:2 w:2)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	/// Proof: VaultRegistry SecureCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: Oracle Aggregate (r:1 w:0)
	/// Proof: Oracle Aggregate (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: VaultStaking Nonce (r:1 w:0)
	/// Proof: VaultStaking Nonce (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: VaultStaking TotalCurrentStake (r:1 w:0)
	/// Proof: VaultStaking TotalCurrentStake (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// The range of component `h` is `[2, 10]`.
	/// The range of component `i` is `[1, 10]`.
	/// The range of component `o` is `[2, 3]`.
	/// The range of component `b` is `[541, 2048]`.
	fn execute_cancelled_replace	(h: u32, i: u32, o: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4084`
		//  Estimated: `6460`
		// Minimum execution time: 244_751_000 picoseconds.
		Weight::from_parts(221_003_188, 6460)
			// Standard Error: 69_845
			.saturating_add(Weight::from_parts(3_504_737, 0).saturating_mul(h.into()))
			// Standard Error: 62_966
			.saturating_add(Weight::from_parts(526_198, 0).saturating_mul(i.into()))
			// Standard Error: 381_710
			.saturating_add(Weight::from_parts(2_470_971, 0).saturating_mul(o.into()))
			// Standard Error: 384
			.saturating_add(Weight::from_parts(4_539, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(14_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Replace ReplaceRequests (r:1 w:1)
	/// Proof: Replace ReplaceRequests (max_values: None, max_size: Some(250), added: 2725, mode: MaxEncodedLen)
	/// Storage: Replace ReplacePeriod (r:1 w:0)
	/// Proof: Replace ReplacePeriod (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VaultRegistry Vaults (r:2 w:2)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VaultStaking Nonce (r:1 w:0)
	/// Proof: VaultStaking Nonce (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: VaultStaking TotalCurrentStake (r:1 w:0)
	/// Proof: VaultStaking TotalCurrentStake (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: VaultRegistry MinimumCollateralVault (r:1 w:0)
	/// Proof: VaultRegistry MinimumCollateralVault (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	/// Proof: VaultRegistry SecureCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: Oracle Aggregate (r:1 w:0)
	/// Proof: Oracle Aggregate (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	/// Proof: VaultRegistry TotalUserVaultCollateral (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultCapacity Stake (r:1 w:0)
	/// Proof: VaultCapacity Stake (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultCapacity RewardPerToken (r:2 w:0)
	/// Proof: VaultCapacity RewardPerToken (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: VaultCapacity RewardTally (r:2 w:2)
	/// Proof: VaultCapacity RewardTally (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	/// Storage: VaultCapacity TotalRewards (r:2 w:2)
	/// Proof: VaultCapacity TotalRewards (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultRewards Stake (r:1 w:0)
	/// Proof: VaultRewards Stake (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	/// Storage: VaultRewards RewardPerToken (r:2 w:0)
	/// Proof: VaultRewards RewardPerToken (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	/// Storage: VaultRewards RewardTally (r:2 w:2)
	/// Proof: VaultRewards RewardTally (max_values: None, max_size: Some(124), added: 2599, mode: MaxEncodedLen)
	/// Storage: VaultRewards TotalRewards (r:2 w:2)
	/// Proof: VaultRewards TotalRewards (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Fee Commission (r:1 w:0)
	/// Proof: Fee Commission (max_values: None, max_size: Some(86), added: 2561, mode: MaxEncodedLen)
	/// Storage: VaultStaking RewardPerToken (r:2 w:2)
	/// Proof: VaultStaking RewardPerToken (max_values: None, max_size: Some(117), added: 2592, mode: MaxEncodedLen)
	/// Storage: VaultStaking Stake (r:1 w:1)
	/// Proof: VaultStaking Stake (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: VaultStaking SlashPerToken (r:1 w:0)
	/// Proof: VaultStaking SlashPerToken (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: VaultStaking SlashTally (r:1 w:1)
	/// Proof: VaultStaking SlashTally (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: VaultStaking TotalStake (r:1 w:1)
	/// Proof: VaultStaking TotalStake (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: VaultRewards TotalStake (r:1 w:0)
	/// Proof: VaultRewards TotalStake (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	fn cancel_replace	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4744`
		//  Estimated: `6460`
		// Minimum execution time: 499_671_000 picoseconds.
		Weight::from_parts(507_496_000, 6460)
			.saturating_add(T::DbWeight::get().reads(37_u64))
			.saturating_add(T::DbWeight::get().writes(19_u64))
	}
	/// Storage: Replace ReplacePeriod (r:0 w:1)
	/// Proof: Replace ReplacePeriod (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_replace_period	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 17_806_000 picoseconds.
		Weight::from_parts(18_467_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}