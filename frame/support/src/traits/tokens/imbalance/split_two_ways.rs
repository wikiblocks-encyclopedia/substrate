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

//! Means for splitting an imbalance into two and hanlding them differently.

use super::super::imbalance::{Imbalance, OnUnbalanced};
use sp_runtime::traits::Saturating;
use sp_std::{marker::PhantomData, ops::Div};

/// Split an unbalanced amount two ways between a common divisor.
pub struct SplitTwoWays<Balance, Imbalance, Target1, Target2, const PART1: u32, const PART2: u32>(
	PhantomData<(Balance, Imbalance, Target1, Target2)>,
);

impl<
		Balance: From<u32> + Saturating + Div<Output = Balance>,
		I: Imbalance<Balance>,
		Target1: OnUnbalanced<I>,
		Target2: OnUnbalanced<I>,
		const PART1: u32,
		const PART2: u32,
	> OnUnbalanced<I> for SplitTwoWays<Balance, I, Target1, Target2, PART1, PART2>
{
	fn on_nonzero_unbalanced(amount: I) {
		let total: u32 = PART1 + PART2;
		let amount1 = amount.peek().saturating_mul(PART1.into()) / total.into();
		let (imb1, imb2) = amount.split(amount1);
		Target1::on_unbalanced(imb1);
		Target2::on_unbalanced(imb2);
	}
}
