// This file is part of a fork of Substrate which has had various changes.

// Copyright (C) Parity Technologies (UK) Ltd.
// Copyright (C) 2022-2023 Luke Parker
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

use crate::{self as frame_system, *};
use frame_support::{
	parameter_types,
	traits::{ConstU32, ConstU64},
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage, Perbill,
};

type Block = mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
	}
);

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
const MAX_BLOCK_WEIGHT: Weight = Weight::from_parts(1024, u64::MAX);

parameter_types! {
	pub Version: RuntimeVersion = RuntimeVersion {
		spec_name: sp_version::create_runtime_str!("test"),
		impl_name: sp_version::create_runtime_str!("system-test"),
		spec_version: 1,
		impl_version: 1,
		apis: sp_version::create_apis_vec!([]),
		transaction_version: 1,
		state_version: 1,
	};
	pub const DbWeight: RuntimeDbWeight = RuntimeDbWeight {
		read: 10,
		write: 100,
	};
	pub RuntimeBlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
		.base_block(Weight::from_parts(10, 0))
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = Weight::from_parts(5, 0);
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAX_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.base_extrinsic = Weight::from_parts(10, 0);
			weights.max_total = Some(MAX_BLOCK_WEIGHT);
			weights.reserved = Some(
				MAX_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAX_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(Perbill::from_percent(0))
		.build_or_panic();
	pub RuntimeBlockLength: limits::BlockLength =
		limits::BlockLength::max_with_normal_ratio(1024, NORMAL_DISPATCH_RATIO);
}

parameter_types! {
	pub static Killed: Vec<u64> = vec![];
}

pub struct RecordKilled;
impl OnKilledAccount<u64> for RecordKilled {
	fn on_killed_account(who: &u64) {
		Killed::mutate(|r| r.push(*who))
	}
}

impl Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = RuntimeBlockWeights;
	type BlockLength = RuntimeBlockLength;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<10>;
	type DbWeight = DbWeight;
	type Version = Version;
	type PalletInfo = PalletInfo;
	type AccountData = u32;
	type OnNewAccount = ();
	type OnKilledAccount = RecordKilled;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

pub type SysEvent = frame_system::Event<Test>;

/// A simple call, which one doesn't matter.
pub const CALL: &<Test as Config>::RuntimeCall =
	&RuntimeCall::System(frame_system::Call::set_heap_pages { pages: 0u64 });

/// Create new externalities for `System` module tests.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities =
		RuntimeGenesisConfig::default().build_storage().unwrap().into();
	// Add to each test the initial weight of a block
	ext.execute_with(|| {
		System::register_extra_weight_unchecked(
			<Test as crate::Config>::BlockWeights::get().base_block,
			DispatchClass::Mandatory,
		)
	});
	ext
}
