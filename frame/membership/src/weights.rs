// This file is part of Substrate.

// Copyright (C) 2021 AXIA Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_membership
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-08-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_membership
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/membership/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_membership.
pub trait WeightInfo {
	fn add_member(m: u32, ) -> Weight;
	fn remove_member(m: u32, ) -> Weight;
	fn swap_member(m: u32, ) -> Weight;
	fn reset_member(m: u32, ) -> Weight;
	fn change_key(m: u32, ) -> Weight;
	fn set_prime(m: u32, ) -> Weight;
	fn clear_prime(m: u32, ) -> Weight;
}

/// Weights for pallet_membership using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Instance1Membership Members (r:1 w:1)
	// Storage: Instance2Collective Proposals (r:1 w:0)
	// Storage: Instance2Collective Members (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn add_member(m: u32, ) -> Weight {
		(23_668_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((142_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Membership Members (r:1 w:1)
	// Storage: Instance2Collective Proposals (r:1 w:0)
	// Storage: Instance1Membership Prime (r:1 w:0)
	// Storage: Instance2Collective Members (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn remove_member(m: u32, ) -> Weight {
		(29_149_000 as Weight)
			// Standard Error: 0
			.saturating_add((111_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Membership Members (r:1 w:1)
	// Storage: Instance2Collective Proposals (r:1 w:0)
	// Storage: Instance1Membership Prime (r:1 w:0)
	// Storage: Instance2Collective Members (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn swap_member(m: u32, ) -> Weight {
		(29_289_000 as Weight)
			// Standard Error: 0
			.saturating_add((126_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Membership Members (r:1 w:1)
	// Storage: Instance2Collective Proposals (r:1 w:0)
	// Storage: Instance1Membership Prime (r:1 w:0)
	// Storage: Instance2Collective Members (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn reset_member(m: u32, ) -> Weight {
		(30_178_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((286_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Membership Members (r:1 w:1)
	// Storage: Instance2Collective Proposals (r:1 w:0)
	// Storage: Instance1Membership Prime (r:1 w:1)
	// Storage: Instance2Collective Members (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn change_key(m: u32, ) -> Weight {
		(31_049_000 as Weight)
			// Standard Error: 0
			.saturating_add((121_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Instance1Membership Members (r:1 w:0)
	// Storage: Instance1Membership Prime (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn set_prime(m: u32, ) -> Weight {
		(8_006_000 as Weight)
			// Standard Error: 0
			.saturating_add((89_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance1Membership Prime (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn clear_prime(m: u32, ) -> Weight {
		(3_452_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Instance1Membership Members (r:1 w:1)
	// Storage: Instance2Collective Proposals (r:1 w:0)
	// Storage: Instance2Collective Members (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn add_member(m: u32, ) -> Weight {
		(23_668_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((142_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Membership Members (r:1 w:1)
	// Storage: Instance2Collective Proposals (r:1 w:0)
	// Storage: Instance1Membership Prime (r:1 w:0)
	// Storage: Instance2Collective Members (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn remove_member(m: u32, ) -> Weight {
		(29_149_000 as Weight)
			// Standard Error: 0
			.saturating_add((111_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Membership Members (r:1 w:1)
	// Storage: Instance2Collective Proposals (r:1 w:0)
	// Storage: Instance1Membership Prime (r:1 w:0)
	// Storage: Instance2Collective Members (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn swap_member(m: u32, ) -> Weight {
		(29_289_000 as Weight)
			// Standard Error: 0
			.saturating_add((126_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Membership Members (r:1 w:1)
	// Storage: Instance2Collective Proposals (r:1 w:0)
	// Storage: Instance1Membership Prime (r:1 w:0)
	// Storage: Instance2Collective Members (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn reset_member(m: u32, ) -> Weight {
		(30_178_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((286_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Membership Members (r:1 w:1)
	// Storage: Instance2Collective Proposals (r:1 w:0)
	// Storage: Instance1Membership Prime (r:1 w:1)
	// Storage: Instance2Collective Members (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn change_key(m: u32, ) -> Weight {
		(31_049_000 as Weight)
			// Standard Error: 0
			.saturating_add((121_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Instance1Membership Members (r:1 w:0)
	// Storage: Instance1Membership Prime (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn set_prime(m: u32, ) -> Weight {
		(8_006_000 as Weight)
			// Standard Error: 0
			.saturating_add((89_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance1Membership Prime (r:0 w:1)
	// Storage: Instance2Collective Prime (r:0 w:1)
	fn clear_prime(m: u32, ) -> Weight {
		(3_452_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
