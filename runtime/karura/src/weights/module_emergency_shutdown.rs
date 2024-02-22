// This file is part of Acala.

// Copyright (C) 2020-2024 Acala Foundation.
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

//! Autogenerated weights for module_emergency_shutdown
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-02-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-26-6`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_emergency_shutdown.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_emergency_shutdown::WeightInfo for WeightInfo<T> {
	// Storage: `EmergencyShutdown::IsShutdown` (r:1 w:1)
	// Proof: `EmergencyShutdown::IsShutdown` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	// Storage: `CdpEngine::CollateralParams` (r:1 w:0)
	// Proof: `CdpEngine::CollateralParams` (`max_values`: None, `max_size`: Some(135), added: 2610, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 4]`.
	fn emergency_shutdown(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1286`
		//  Estimated: `3600`
		// Minimum execution time: 17_630 nanoseconds.
		Weight::from_parts(18_603_808, 3600)
			// Standard Error: 10_654
			.saturating_add(Weight::from_parts(400_284, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `EmergencyShutdown::IsShutdown` (r:1 w:0)
	// Proof: `EmergencyShutdown::IsShutdown` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	// Storage: `CdpEngine::CollateralParams` (r:1 w:0)
	// Proof: `CdpEngine::CollateralParams` (`max_values`: None, `max_size`: Some(135), added: 2610, mode: `MaxEncodedLen`)
	// Storage: `EmergencyShutdown::CanRefund` (r:0 w:1)
	// Proof: `EmergencyShutdown::CanRefund` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn open_collateral_refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1310`
		//  Estimated: `3600`
		// Minimum execution time: 17_646 nanoseconds.
		Weight::from_parts(18_003_000, 3600)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `EmergencyShutdown::CanRefund` (r:1 w:0)
	// Proof: `EmergencyShutdown::CanRefund` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `CdpEngine::CollateralParams` (r:1 w:0)
	// Proof: `CdpEngine::CollateralParams` (`max_values`: None, `max_size`: Some(135), added: 2610, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 4]`.
	fn refund_collaterals(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2059 + c * (33 ±0)`
		//  Estimated: `3612`
		// Minimum execution time: 47_810 nanoseconds.
		Weight::from_parts(50_185_527, 3612)
			// Standard Error: 33_860
			.saturating_add(Weight::from_parts(1_756_630, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
