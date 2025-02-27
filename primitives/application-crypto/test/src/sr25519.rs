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

//! Integration tests for sr25519

use sp_api::{ApiExt, ProvideRuntimeApi};
use sp_application_crypto::sr25519::AppPair;
use sp_core::{
	crypto::{ByteArray, Pair},
	testing::SR25519,
};
use sp_keystore::{testing::MemoryKeystore, Keystore, KeystoreExt};
use std::sync::Arc;
use substrate_test_runtime_client::{
	runtime::TestAPI, DefaultTestClientBuilderExt, TestClientBuilder, TestClientBuilderExt,
};

#[test]
fn sr25519_works_in_runtime() {
	let keystore = Arc::new(MemoryKeystore::new());
	let test_client = TestClientBuilder::new().build();

	let mut runtime_api = test_client.runtime_api();
	runtime_api.register_extension(KeystoreExt::new(keystore.clone()));

	let (signature, public) = runtime_api
		.test_sr25519_crypto(test_client.chain_info().genesis_hash)
		.expect("Tests `sr25519` crypto.");

	let supported_keys = keystore.keys(SR25519).unwrap();
	assert!(supported_keys.contains(&public.to_raw_vec()));
	assert!(AppPair::verify(&signature, "sr25519", &public));
}
