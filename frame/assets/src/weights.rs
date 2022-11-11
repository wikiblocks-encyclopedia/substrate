// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_assets
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm2`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_assets
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/assets/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_assets.
pub trait WeightInfo {
	fn create() -> Weight;
	fn force_create() -> Weight;
	fn destroy(c: u32, s: u32, a: u32, ) -> Weight;
	fn mint() -> Weight;
	fn burn() -> Weight;
	fn transfer() -> Weight;
	fn transfer_keep_alive() -> Weight;
	fn force_transfer() -> Weight;
	fn freeze() -> Weight;
	fn thaw() -> Weight;
	fn freeze_asset() -> Weight;
	fn thaw_asset() -> Weight;
	fn transfer_ownership() -> Weight;
	fn set_team() -> Weight;
	fn set_metadata(n: u32, s: u32, ) -> Weight;
	fn clear_metadata() -> Weight;
	fn force_set_metadata(n: u32, s: u32, ) -> Weight;
	fn force_clear_metadata() -> Weight;
	fn force_asset_status() -> Weight;
	fn approve_transfer() -> Weight;
	fn transfer_approved() -> Weight;
	fn cancel_approval() -> Weight;
	fn force_cancel_approval() -> Weight;
}

/// Weights for pallet_assets using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Assets Asset (r:1 w:1)
	fn create() -> Weight {
		// Minimum execution time: 33_241 nanoseconds.
		Weight::from_ref_time(33_873_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn force_create() -> Weight {
		// Minimum execution time: 19_883 nanoseconds.
		Weight::from_ref_time(20_651_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:5002 w:5001)
	// Storage: System Account (r:5000 w:5000)
	// Storage: Assets Metadata (r:1 w:0)
	// Storage: Assets Approvals (r:501 w:500)
	/// The range of component `c` is `[0, 5000]`.
	/// The range of component `s` is `[0, 5000]`.
	/// The range of component `a` is `[0, 500]`.
	fn destroy(c: u32, s: u32, a: u32, ) -> Weight {
		// Minimum execution time: 76_222_544 nanoseconds.
		Weight::from_ref_time(76_864_587_000 as u64)
			// Standard Error: 127_086
			.saturating_add(Weight::from_ref_time(8_645_143 as u64).saturating_mul(c as u64))
			// Standard Error: 127_086
			.saturating_add(Weight::from_ref_time(11_281_301 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(a as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(a as u64)))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	fn mint() -> Weight {
		// Minimum execution time: 36_782 nanoseconds.
		Weight::from_ref_time(37_340_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	fn burn() -> Weight {
		// Minimum execution time: 44_425 nanoseconds.
		Weight::from_ref_time(45_485_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		// Minimum execution time: 58_294 nanoseconds.
		Weight::from_ref_time(59_447_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive() -> Weight {
		// Minimum execution time: 46_704 nanoseconds.
		Weight::from_ref_time(47_521_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn force_transfer() -> Weight {
		// Minimum execution time: 57_647 nanoseconds.
		Weight::from_ref_time(58_417_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Account (r:1 w:1)
	fn freeze() -> Weight {
		// Minimum execution time: 26_827 nanoseconds.
		Weight::from_ref_time(27_373_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Account (r:1 w:1)
	fn thaw() -> Weight {
		// Minimum execution time: 26_291 nanoseconds.
		Weight::from_ref_time(26_854_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn freeze_asset() -> Weight {
		// Minimum execution time: 22_694 nanoseconds.
		Weight::from_ref_time(23_613_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn thaw_asset() -> Weight {
		// Minimum execution time: 22_572 nanoseconds.
		Weight::from_ref_time(24_121_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:0)
	fn transfer_ownership() -> Weight {
		// Minimum execution time: 23_949 nanoseconds.
		Weight::from_ref_time(24_347_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn set_team() -> Weight {
		// Minimum execution time: 23_102 nanoseconds.
		Weight::from_ref_time(23_518_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(_n: u32, s: u32, ) -> Weight {
		// Minimum execution time: 41_032 nanoseconds.
		Weight::from_ref_time(42_845_624 as u64)
			// Standard Error: 1_274
			.saturating_add(Weight::from_ref_time(1_875 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	fn clear_metadata() -> Weight {
		// Minimum execution time: 42_570 nanoseconds.
		Weight::from_ref_time(42_957_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(n: u32, s: u32, ) -> Weight {
		// Minimum execution time: 22_768 nanoseconds.
		Weight::from_ref_time(23_868_816 as u64)
			// Standard Error: 612
			.saturating_add(Weight::from_ref_time(1_602 as u64).saturating_mul(n as u64))
			// Standard Error: 612
			.saturating_add(Weight::from_ref_time(2_097 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	fn force_clear_metadata() -> Weight {
		// Minimum execution time: 41_863 nanoseconds.
		Weight::from_ref_time(42_643_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn force_asset_status() -> Weight {
		// Minimum execution time: 21_747 nanoseconds.
		Weight::from_ref_time(22_595_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn approve_transfer() -> Weight {
		// Minimum execution time: 45_602 nanoseconds.
		Weight::from_ref_time(46_004_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Assets Approvals (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_approved() -> Weight {
		// Minimum execution time: 70_944 nanoseconds.
		Weight::from_ref_time(71_722_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn cancel_approval() -> Weight {
		// Minimum execution time: 46_316 nanoseconds.
		Weight::from_ref_time(46_910_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn force_cancel_approval() -> Weight {
		// Minimum execution time: 47_145 nanoseconds.
		Weight::from_ref_time(47_611_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Assets Asset (r:1 w:1)
	fn create() -> Weight {
		// Minimum execution time: 33_241 nanoseconds.
		Weight::from_ref_time(33_873_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn force_create() -> Weight {
		// Minimum execution time: 19_883 nanoseconds.
		Weight::from_ref_time(20_651_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:5002 w:5001)
	// Storage: System Account (r:5000 w:5000)
	// Storage: Assets Metadata (r:1 w:0)
	// Storage: Assets Approvals (r:501 w:500)
	/// The range of component `c` is `[0, 5000]`.
	/// The range of component `s` is `[0, 5000]`.
	/// The range of component `a` is `[0, 500]`.
	fn destroy(c: u32, s: u32, a: u32, ) -> Weight {
		// Minimum execution time: 76_222_544 nanoseconds.
		Weight::from_ref_time(76_864_587_000 as u64)
			// Standard Error: 127_086
			.saturating_add(Weight::from_ref_time(8_645_143 as u64).saturating_mul(c as u64))
			// Standard Error: 127_086
			.saturating_add(Weight::from_ref_time(11_281_301 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(c as u64)))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(a as u64)))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(c as u64)))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(a as u64)))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	fn mint() -> Weight {
		// Minimum execution time: 36_782 nanoseconds.
		Weight::from_ref_time(37_340_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	fn burn() -> Weight {
		// Minimum execution time: 44_425 nanoseconds.
		Weight::from_ref_time(45_485_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		// Minimum execution time: 58_294 nanoseconds.
		Weight::from_ref_time(59_447_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive() -> Weight {
		// Minimum execution time: 46_704 nanoseconds.
		Weight::from_ref_time(47_521_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn force_transfer() -> Weight {
		// Minimum execution time: 57_647 nanoseconds.
		Weight::from_ref_time(58_417_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Account (r:1 w:1)
	fn freeze() -> Weight {
		// Minimum execution time: 26_827 nanoseconds.
		Weight::from_ref_time(27_373_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Account (r:1 w:1)
	fn thaw() -> Weight {
		// Minimum execution time: 26_291 nanoseconds.
		Weight::from_ref_time(26_854_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn freeze_asset() -> Weight {
		// Minimum execution time: 22_694 nanoseconds.
		Weight::from_ref_time(23_613_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn thaw_asset() -> Weight {
		// Minimum execution time: 22_572 nanoseconds.
		Weight::from_ref_time(24_121_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:0)
	fn transfer_ownership() -> Weight {
		// Minimum execution time: 23_949 nanoseconds.
		Weight::from_ref_time(24_347_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn set_team() -> Weight {
		// Minimum execution time: 23_102 nanoseconds.
		Weight::from_ref_time(23_518_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(_n: u32, s: u32, ) -> Weight {
		// Minimum execution time: 41_032 nanoseconds.
		Weight::from_ref_time(42_845_624 as u64)
			// Standard Error: 1_274
			.saturating_add(Weight::from_ref_time(1_875 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	fn clear_metadata() -> Weight {
		// Minimum execution time: 42_570 nanoseconds.
		Weight::from_ref_time(42_957_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(n: u32, s: u32, ) -> Weight {
		// Minimum execution time: 22_768 nanoseconds.
		Weight::from_ref_time(23_868_816 as u64)
			// Standard Error: 612
			.saturating_add(Weight::from_ref_time(1_602 as u64).saturating_mul(n as u64))
			// Standard Error: 612
			.saturating_add(Weight::from_ref_time(2_097 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	fn force_clear_metadata() -> Weight {
		// Minimum execution time: 41_863 nanoseconds.
		Weight::from_ref_time(42_643_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn force_asset_status() -> Weight {
		// Minimum execution time: 21_747 nanoseconds.
		Weight::from_ref_time(22_595_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn approve_transfer() -> Weight {
		// Minimum execution time: 45_602 nanoseconds.
		Weight::from_ref_time(46_004_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Assets Approvals (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_approved() -> Weight {
		// Minimum execution time: 70_944 nanoseconds.
		Weight::from_ref_time(71_722_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn cancel_approval() -> Weight {
		// Minimum execution time: 46_316 nanoseconds.
		Weight::from_ref_time(46_910_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn force_cancel_approval() -> Weight {
		// Minimum execution time: 47_145 nanoseconds.
		Weight::from_ref_time(47_611_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
}
