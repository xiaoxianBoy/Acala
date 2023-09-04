// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_transaction_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-47-187`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_transaction_payment.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_transaction_payment::WeightInfo for WeightInfo<T> {
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `TransactionPayment::AlternativeFeeSwapPath` (r:0 w:1)
	// Proof: `TransactionPayment::AlternativeFeeSwapPath` (`max_values`: None, `max_size`: Some(213), added: 2688, mode: `MaxEncodedLen`)
	fn set_alternative_fee_swap_path() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1405`
		//  Estimated: `3633`
		// Minimum execution time: 34_517 nanoseconds.
		Weight::from_parts(35_440_000, 3633)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: `TransactionPayment::PoolSize` (r:1 w:1)
	// Proof: `TransactionPayment::PoolSize` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Dex::TradingPairStatuses` (r:4 w:0)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Dex::LiquidityPool` (r:1 w:0)
	// Proof: `Dex::LiquidityPool` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `StableAsset::Pools` (r:1 w:0)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `AggregatedDex::AggregatedSwapPaths` (r:1 w:0)
	// Proof: `AggregatedDex::AggregatedSwapPaths` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `TransactionPayment::TokenExchangeRate` (r:0 w:1)
	// Proof: `TransactionPayment::TokenExchangeRate` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `TransactionPayment::SwapBalanceThreshold` (r:0 w:1)
	// Proof: `TransactionPayment::SwapBalanceThreshold` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn enable_charge_fee_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2272`
		//  Estimated: `11670`
		// Minimum execution time: 117_669 nanoseconds.
		Weight::from_parts(120_001_000, 11670)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: `TransactionPayment::TokenExchangeRate` (r:1 w:1)
	// Proof: `TransactionPayment::TokenExchangeRate` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `TransactionPayment::SwapBalanceThreshold` (r:0 w:1)
	// Proof: `TransactionPayment::SwapBalanceThreshold` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `TransactionPayment::GlobalFeeSwapPath` (r:0 w:1)
	// Proof: `TransactionPayment::GlobalFeeSwapPath` (`max_values`: None, `max_size`: Some(224), added: 2699, mode: `MaxEncodedLen`)
	// Storage: `TransactionPayment::PoolSize` (r:0 w:1)
	// Proof: `TransactionPayment::PoolSize` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn disable_charge_fee_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1778`
		//  Estimated: `6234`
		// Minimum execution time: 100_559 nanoseconds.
		Weight::from_parts(102_219_000, 6234)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	fn with_fee_path() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `666`
		//  Estimated: `0`
		// Minimum execution time: 8_529 nanoseconds.
		Weight::from_parts(8_856_000, 0)
	}
	fn with_fee_currency() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `666`
		//  Estimated: `0`
		// Minimum execution time: 9_244 nanoseconds.
		Weight::from_parts(9_862_000, 0)
	}
	fn with_fee_aggregated_path() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `666`
		//  Estimated: `0`
		// Minimum execution time: 9_563 nanoseconds.
		Weight::from_parts(9_999_000, 0)
	}
	// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:1)
	// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn on_finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `955`
		//  Estimated: `1501`
		// Minimum execution time: 10_040 nanoseconds.
		Weight::from_parts(10_294_000, 1501)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
