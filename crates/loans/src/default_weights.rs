//! Autogenerated weights for loans
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-09, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-testnet-latest"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --chain
// kintsugi-testnet-latest
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// loans
// --extrinsic
// *
// --steps
// 100
// --repeat
// 10
// --output
// ./crates/loans/src/default_weights.rs
// --template
// .deploy/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for loans.
pub trait WeightInfo {
	fn add_market() -> Weight;
	fn activate_market() -> Weight;
	fn update_rate_model() -> Weight;
	fn update_market() -> Weight;
	fn force_update_market() -> Weight;
	fn add_reward() -> Weight;
	fn update_market_reward_speed() -> Weight;
	fn claim_reward() -> Weight;
	fn claim_reward_for_market() -> Weight;
	fn mint() -> Weight;
	fn borrow() -> Weight;
	fn redeem() -> Weight;
	fn redeem_all() -> Weight;
	fn repay_borrow() -> Weight;
	fn repay_borrow_all() -> Weight;
	fn deposit_all_collateral() -> Weight;
	fn withdraw_all_collateral() -> Weight;
	fn add_reserves() -> Weight;
	fn reduce_reserves() -> Weight;
	fn liquidate_borrow() -> Weight;
	fn reduce_incentive_reserves() -> Weight;
}

/// Weights for loans using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Loans Markets (r:2 w:1)
	// Storage: Loans UnderlyingAssetId (r:1 w:1)
	// Storage: Loans MinExchangeRate (r:1 w:0)
	// Storage: Loans ExchangeRate (r:0 w:1)
	// Storage: Loans BorrowIndex (r:0 w:1)
	fn add_market() -> Weight {
		Weight::from_ref_time(61_934_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Loans Markets (r:1 w:1)
	fn activate_market() -> Weight {
		Weight::from_ref_time(36_233_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Loans Markets (r:1 w:1)
	fn update_rate_model() -> Weight {
		Weight::from_ref_time(37_665_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Loans Markets (r:1 w:1)
	fn update_market() -> Weight {
		Weight::from_ref_time(40_371_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Loans UnderlyingAssetId (r:1 w:1)
	// Storage: Loans Markets (r:1 w:1)
	fn force_update_market() -> Weight {
		Weight::from_ref_time(47_215_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn add_reward() -> Weight {
		Weight::from_ref_time(87_936_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Loans RewardSupplySpeed (r:1 w:1)
	// Storage: Loans RewardBorrowSpeed (r:1 w:1)
	// Storage: Loans RewardSupplyState (r:1 w:1)
	// Storage: Loans RewardBorrowState (r:1 w:1)
	fn update_market_reward_speed() -> Weight {
		Weight::from_ref_time(69_970_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Loans RewardSupplyState (r:1 w:1)
	// Storage: Loans RewardSupplySpeed (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Loans RewardSupplierIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:2)
	// Storage: Loans RewardBorrowState (r:1 w:1)
	// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	// Storage: Loans TotalBorrows (r:1 w:0)
	// Storage: Loans BorrowIndex (r:1 w:0)
	// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	// Storage: Loans AccountBorrows (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	fn claim_reward() -> Weight {
		Weight::from_ref_time(200_241_000 as u64)
			.saturating_add(T::DbWeight::get().reads(17 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: Loans RewardSupplyState (r:1 w:1)
	// Storage: Loans RewardSupplySpeed (r:1 w:0)
	// Storage: Loans Markets (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Loans RewardSupplierIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:2)
	// Storage: Loans RewardBorrowState (r:1 w:1)
	// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	// Storage: Loans TotalBorrows (r:1 w:0)
	// Storage: Loans BorrowIndex (r:1 w:0)
	// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	// Storage: Loans AccountBorrows (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	fn claim_reward_for_market() -> Weight {
		Weight::from_ref_time(187_065_000 as u64)
			.saturating_add(T::DbWeight::get().reads(16 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Loans RewardSupplyState (r:1 w:1)
	// Storage: Loans RewardSupplySpeed (r:1 w:0)
	// Storage: Loans RewardSupplierIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	// Storage: Loans UnderlyingAssetId (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Loans TotalBorrows (r:1 w:0)
	// Storage: Loans TotalReserves (r:1 w:0)
	// Storage: Loans MinExchangeRate (r:1 w:0)
	// Storage: Loans AccountDeposits (r:1 w:0)
	fn mint() -> Weight {
		Weight::from_ref_time(264_108_000 as u64)
			.saturating_add(T::DbWeight::get().reads(18 as u64))
			.saturating_add(T::DbWeight::get().writes(9 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Loans TotalBorrows (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	// Storage: Loans TotalReserves (r:1 w:0)
	// Storage: Loans AccountDeposits (r:1 w:0)
	// Storage: Loans UnderlyingAssetId (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Loans MinExchangeRate (r:1 w:0)
	// Storage: Loans MaxExchangeRate (r:1 w:0)
	// Storage: Loans AccountBorrows (r:1 w:1)
	// Storage: Loans RewardBorrowState (r:1 w:1)
	// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	// Storage: Loans BorrowIndex (r:1 w:0)
	fn borrow() -> Weight {
		Weight::from_ref_time(253_698_000 as u64)
			.saturating_add(T::DbWeight::get().reads(20 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Loans AccountDeposits (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Loans UnderlyingAssetId (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: System Account (r:1 w:0)
	// Storage: Loans TotalBorrows (r:1 w:0)
	// Storage: Loans TotalReserves (r:1 w:0)
	// Storage: Loans MinExchangeRate (r:1 w:0)
	// Storage: Loans MaxExchangeRate (r:1 w:0)
	// Storage: Loans RewardSupplyState (r:1 w:1)
	// Storage: Loans RewardSupplySpeed (r:1 w:0)
	// Storage: Loans RewardSupplierIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	fn redeem() -> Weight {
		Weight::from_ref_time(354_348_000 as u64)
			.saturating_add(T::DbWeight::get().reads(19 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Loans AccountDeposits (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Loans UnderlyingAssetId (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Loans TotalBorrows (r:1 w:0)
	// Storage: Loans TotalReserves (r:1 w:0)
	// Storage: Loans MinExchangeRate (r:1 w:0)
	// Storage: Loans MaxExchangeRate (r:1 w:0)
	// Storage: Loans RewardSupplyState (r:1 w:1)
	// Storage: Loans RewardSupplySpeed (r:1 w:0)
	// Storage: Loans RewardSupplierIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	fn redeem_all() -> Weight {
		Weight::from_ref_time(327_185_000 as u64)
			.saturating_add(T::DbWeight::get().reads(19 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Loans AccountBorrows (r:1 w:1)
	// Storage: Loans BorrowIndex (r:1 w:0)
	// Storage: Loans RewardBorrowState (r:1 w:1)
	// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: Loans TotalBorrows (r:1 w:1)
	fn repay_borrow() -> Weight {
		Weight::from_ref_time(170_250_000 as u64)
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Loans AccountBorrows (r:1 w:1)
	// Storage: Loans BorrowIndex (r:1 w:0)
	// Storage: Loans RewardBorrowState (r:1 w:1)
	// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: Loans TotalBorrows (r:1 w:1)
	fn repay_borrow_all() -> Weight {
		Weight::from_ref_time(188_607_000 as u64)
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Loans UnderlyingAssetId (r:1 w:0)
	// Storage: Loans AccountDeposits (r:1 w:1)
	fn deposit_all_collateral() -> Weight {
		Weight::from_ref_time(96_663_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Loans AccountDeposits (r:1 w:1)
	// Storage: Loans UnderlyingAssetId (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Loans TotalBorrows (r:1 w:0)
	// Storage: Loans TotalReserves (r:1 w:0)
	// Storage: Loans MinExchangeRate (r:1 w:0)
	// Storage: Loans MaxExchangeRate (r:1 w:0)
	// Storage: Loans AccountBorrows (r:1 w:0)
	fn withdraw_all_collateral() -> Weight {
		Weight::from_ref_time(377_365_000 as u64)
			.saturating_add(T::DbWeight::get().reads(15 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn liquidate_borrow() -> Weight {
		Weight::from_ref_time(637_956_000 as u64)
			.saturating_add(T::DbWeight::get().reads(40 as u64))
			.saturating_add(T::DbWeight::get().writes(20 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Loans TotalReserves (r:1 w:1)
	fn add_reserves() -> Weight {
		Weight::from_ref_time(125_792_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Loans TotalReserves (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn reduce_reserves() -> Weight {
		Weight::from_ref_time(110_861_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	fn reduce_incentive_reserves() -> Weight {
		Weight::from_ref_time(370_601_000 as u64)
			.saturating_add(T::DbWeight::get().reads(22 as u64))
			.saturating_add(T::DbWeight::get().writes(9 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Loans Markets (r:2 w:1)
	// Storage: Loans UnderlyingAssetId (r:1 w:1)
	// Storage: Loans MinExchangeRate (r:1 w:0)
	// Storage: Loans ExchangeRate (r:0 w:1)
	// Storage: Loans BorrowIndex (r:0 w:1)
	fn add_market() -> Weight {
		Weight::from_ref_time(61_934_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Loans Markets (r:1 w:1)
	fn activate_market() -> Weight {
		Weight::from_ref_time(36_233_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Loans Markets (r:1 w:1)
	fn update_rate_model() -> Weight {
		Weight::from_ref_time(37_665_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Loans Markets (r:1 w:1)
	fn update_market() -> Weight {
		Weight::from_ref_time(40_371_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Loans UnderlyingAssetId (r:1 w:1)
	// Storage: Loans Markets (r:1 w:1)
	fn force_update_market() -> Weight {
		Weight::from_ref_time(47_215_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn add_reward() -> Weight {
		Weight::from_ref_time(87_936_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Loans RewardSupplySpeed (r:1 w:1)
	// Storage: Loans RewardBorrowSpeed (r:1 w:1)
	// Storage: Loans RewardSupplyState (r:1 w:1)
	// Storage: Loans RewardBorrowState (r:1 w:1)
	fn update_market_reward_speed() -> Weight {
		Weight::from_ref_time(69_970_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Loans RewardSupplyState (r:1 w:1)
	// Storage: Loans RewardSupplySpeed (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Loans RewardSupplierIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:2)
	// Storage: Loans RewardBorrowState (r:1 w:1)
	// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	// Storage: Loans TotalBorrows (r:1 w:0)
	// Storage: Loans BorrowIndex (r:1 w:0)
	// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	// Storage: Loans AccountBorrows (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	fn claim_reward() -> Weight {
		Weight::from_ref_time(200_241_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(17 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	// Storage: Loans RewardSupplyState (r:1 w:1)
	// Storage: Loans RewardSupplySpeed (r:1 w:0)
	// Storage: Loans Markets (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Loans RewardSupplierIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:2)
	// Storage: Loans RewardBorrowState (r:1 w:1)
	// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	// Storage: Loans TotalBorrows (r:1 w:0)
	// Storage: Loans BorrowIndex (r:1 w:0)
	// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	// Storage: Loans AccountBorrows (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	fn claim_reward_for_market() -> Weight {
		Weight::from_ref_time(187_065_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(16 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Loans RewardSupplyState (r:1 w:1)
	// Storage: Loans RewardSupplySpeed (r:1 w:0)
	// Storage: Loans RewardSupplierIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	// Storage: Loans UnderlyingAssetId (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Loans TotalBorrows (r:1 w:0)
	// Storage: Loans TotalReserves (r:1 w:0)
	// Storage: Loans MinExchangeRate (r:1 w:0)
	// Storage: Loans AccountDeposits (r:1 w:0)
	fn mint() -> Weight {
		Weight::from_ref_time(264_108_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(18 as u64))
			.saturating_add(RocksDbWeight::get().writes(9 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Loans TotalBorrows (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	// Storage: Loans TotalReserves (r:1 w:0)
	// Storage: Loans AccountDeposits (r:1 w:0)
	// Storage: Loans UnderlyingAssetId (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Loans MinExchangeRate (r:1 w:0)
	// Storage: Loans MaxExchangeRate (r:1 w:0)
	// Storage: Loans AccountBorrows (r:1 w:1)
	// Storage: Loans RewardBorrowState (r:1 w:1)
	// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	// Storage: Loans BorrowIndex (r:1 w:0)
	fn borrow() -> Weight {
		Weight::from_ref_time(253_698_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(20 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Loans AccountDeposits (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Loans UnderlyingAssetId (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: System Account (r:1 w:0)
	// Storage: Loans TotalBorrows (r:1 w:0)
	// Storage: Loans TotalReserves (r:1 w:0)
	// Storage: Loans MinExchangeRate (r:1 w:0)
	// Storage: Loans MaxExchangeRate (r:1 w:0)
	// Storage: Loans RewardSupplyState (r:1 w:1)
	// Storage: Loans RewardSupplySpeed (r:1 w:0)
	// Storage: Loans RewardSupplierIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	fn redeem() -> Weight {
		Weight::from_ref_time(354_348_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(19 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Loans AccountDeposits (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Loans UnderlyingAssetId (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Loans TotalBorrows (r:1 w:0)
	// Storage: Loans TotalReserves (r:1 w:0)
	// Storage: Loans MinExchangeRate (r:1 w:0)
	// Storage: Loans MaxExchangeRate (r:1 w:0)
	// Storage: Loans RewardSupplyState (r:1 w:1)
	// Storage: Loans RewardSupplySpeed (r:1 w:0)
	// Storage: Loans RewardSupplierIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	fn redeem_all() -> Weight {
		Weight::from_ref_time(327_185_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(19 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Loans AccountBorrows (r:1 w:1)
	// Storage: Loans BorrowIndex (r:1 w:0)
	// Storage: Loans RewardBorrowState (r:1 w:1)
	// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: Loans TotalBorrows (r:1 w:1)
	fn repay_borrow() -> Weight {
		Weight::from_ref_time(170_250_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(13 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Loans AccountBorrows (r:1 w:1)
	// Storage: Loans BorrowIndex (r:1 w:0)
	// Storage: Loans RewardBorrowState (r:1 w:1)
	// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	// Storage: Loans RewardAccrued (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: Loans TotalBorrows (r:1 w:1)
	fn repay_borrow_all() -> Weight {
		Weight::from_ref_time(188_607_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(13 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Loans UnderlyingAssetId (r:1 w:0)
	// Storage: Loans AccountDeposits (r:1 w:1)
	fn deposit_all_collateral() -> Weight {
		Weight::from_ref_time(96_663_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Loans AccountDeposits (r:1 w:1)
	// Storage: Loans UnderlyingAssetId (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Loans TotalBorrows (r:1 w:0)
	// Storage: Loans TotalReserves (r:1 w:0)
	// Storage: Loans MinExchangeRate (r:1 w:0)
	// Storage: Loans MaxExchangeRate (r:1 w:0)
	// Storage: Loans AccountBorrows (r:1 w:0)
	fn withdraw_all_collateral() -> Weight {
		Weight::from_ref_time(377_365_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(15 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn liquidate_borrow() -> Weight {
		Weight::from_ref_time(637_956_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(40 as u64))
			.saturating_add(RocksDbWeight::get().writes(20 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Loans TotalReserves (r:1 w:1)
	fn add_reserves() -> Weight {
		Weight::from_ref_time(125_792_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Loans Markets (r:2 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	// Storage: Loans TotalReserves (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn reduce_reserves() -> Weight {
		Weight::from_ref_time(110_861_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	fn reduce_incentive_reserves() -> Weight {
		Weight::from_ref_time(370_601_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(22 as u64))
			.saturating_add(RocksDbWeight::get().writes(9 as u64))
	}
}
