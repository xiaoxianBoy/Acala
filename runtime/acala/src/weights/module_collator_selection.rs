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

//! Autogenerated weights for module_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-02-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-21-58`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

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

/// Weight functions for module_collator_selection.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_collator_selection::WeightInfo for WeightInfo<T> {
	// Storage: `CollatorSelection::Invulnerables` (r:0 w:1)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(321), added: 816, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 10]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `997`
		//  Estimated: `0`
		// Minimum execution time: 11_219 nanoseconds.
		Weight::from_parts(11_747_379, 0)
			// Standard Error: 1_722
			.saturating_add(Weight::from_parts(18_868, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `CollatorSelection::DesiredCandidates` (r:0 w:1)
	// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_desired_candidates() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `997`
		//  Estimated: `0`
		// Minimum execution time: 11_096 nanoseconds.
		Weight::from_parts(11_686_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `CollatorSelection::CandidacyBond` (r:0 w:1)
	// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn set_candidacy_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `997`
		//  Estimated: `0`
		// Minimum execution time: 11_166 nanoseconds.
		Weight::from_parts(11_606_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `CollatorSelection::NonCandidates` (r:1 w:1)
	// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Candidates` (r:1 w:1)
	// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::DesiredCandidates` (r:1 w:0)
	// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(321), added: 816, mode: `MaxEncodedLen`)
	// Storage: `Session::NextKeys` (r:1 w:0)
	// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[5, 50]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2325 + c * (60 ±0)`
		//  Estimated: `5736 + c * (61 ±0)`
		// Minimum execution time: 51_904 nanoseconds.
		Weight::from_parts(52_741_336, 5736)
			// Standard Error: 3_407
			.saturating_add(Weight::from_parts(454_069, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 61).saturating_mul(c.into()))
	}
	// Storage: `CollatorSelection::Candidates` (r:1 w:1)
	// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::DesiredCandidates` (r:1 w:0)
	// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(321), added: 816, mode: `MaxEncodedLen`)
	// Storage: `Session::NextKeys` (r:1 w:0)
	// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Balances::Reserves` (r:1 w:0)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 50]`.
	fn register_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1838 + c * (55 ±0)`
		//  Estimated: `5295 + c * (55 ±0)`
		// Minimum execution time: 28_745 nanoseconds.
		Weight::from_parts(33_254_719, 5295)
			// Standard Error: 3_004
			.saturating_add(Weight::from_parts(411_310, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 55).saturating_mul(c.into()))
	}
	// Storage: `CollatorSelection::Candidates` (r:1 w:1)
	// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `Session::CurrentIndex` (r:1 w:0)
	// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `CollatorSelection::NonCandidates` (r:0 w:1)
	// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[6, 50]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1628 + c * (39 ±0)`
		//  Estimated: `3109 + c * (39 ±0)`
		// Minimum execution time: 22_637 nanoseconds.
		Weight::from_parts(22_211_807, 3109)
			// Standard Error: 2_394
			.saturating_add(Weight::from_parts(322_364, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 39).saturating_mul(c.into()))
	}
	// Storage: `CollatorSelection::NonCandidates` (r:1 w:1)
	// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	// Storage: `Session::CurrentIndex` (r:1 w:0)
	// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	fn withdraw_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3193`
		//  Estimated: `4678`
		// Minimum execution time: 54_873 nanoseconds.
		Weight::from_parts(57_284_000, 4678)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::SessionPoints` (r:1 w:0)
	// Proof: `CollatorSelection::SessionPoints` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn note_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2138`
		//  Estimated: `6196`
		// Minimum execution time: 59_072 nanoseconds.
		Weight::from_parts(60_661_000, 6196)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: `CollatorSelection::Candidates` (r:1 w:0)
	// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(321), added: 816, mode: `MaxEncodedLen`)
	fn new_session() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2687`
		//  Estimated: `3086`
		// Minimum execution time: 21_814 nanoseconds.
		Weight::from_parts(23_406_000, 3086)
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: `Session::Validators` (r:1 w:0)
	// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `CollatorSelection::Candidates` (r:1 w:0)
	// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::SessionPoints` (r:0 w:50)
	// Proof: `CollatorSelection::SessionPoints` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[5, 50]`.
	/// The range of component `c` is `[5, 50]`.
	fn start_session(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1411 + c * (64 ±0)`
		//  Estimated: `3086 + c * (64 ±0)`
		// Minimum execution time: 19_276 nanoseconds.
		Weight::from_parts(14_412_023, 3086)
			// Standard Error: 1_676
			.saturating_add(Weight::from_parts(2_526, 0).saturating_mul(r.into()))
			// Standard Error: 1_676
			.saturating_add(Weight::from_parts(1_246_330, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(c.into()))
	}
	// Storage: `CollatorSelection::SessionPoints` (r:51 w:50)
	// Proof: `CollatorSelection::SessionPoints` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Candidates` (r:1 w:1)
	// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `Session::CurrentIndex` (r:1 w:0)
	// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `CollatorSelection::NonCandidates` (r:0 w:1)
	// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[5, 50]`.
	/// The range of component `c` is `[5, 50]`.
	fn end_session(_r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3458 + c * (49 ±0)`
		//  Estimated: `3509 + c * (2519 ±0)`
		// Minimum execution time: 30_087 nanoseconds.
		Weight::from_parts(353_296_954, 3509)
			// Standard Error: 18_547
			.saturating_add(Weight::from_parts(4_189_757, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(50))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2519).saturating_mul(c.into()))
	}
}
