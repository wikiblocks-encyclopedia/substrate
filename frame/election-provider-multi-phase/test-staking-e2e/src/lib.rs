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

#![cfg(test)]
mod mock;

pub(crate) const LOG_TARGET: &str = "tests::e2e-epm";

use frame_support::assert_ok;
use mock::*;
use sp_core::Get;
use sp_npos_elections::{to_supports, StakedAssignment};
use sp_runtime::Perbill;

use crate::mock::RuntimeOrigin;

// syntactic sugar for logging.
#[macro_export]
macro_rules! log {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: crate::LOG_TARGET,
			concat!("🛠️  ", $patter)  $(, $values)*
		)
	};
}

fn log_current_time() {
	log!(
		trace,
		"block: {:?}, session: {:?}, era: {:?}, EPM phase: {:?} ts: {:?}",
		System::block_number(),
		Session::current_index(),
		Staking::current_era(),
		ElectionProviderMultiPhase::current_phase(),
		Timestamp::now()
	);
}

#[test]
fn block_progression_works() {
	ExtBuilder::default().build_and_execute(|| {
		assert_eq!(active_era(), 0);
		assert_eq!(Session::current_index(), 0);
		assert!(ElectionProviderMultiPhase::current_phase().is_off());

		assert!(start_next_active_era().is_ok());
		assert_eq!(active_era(), 1);
		assert_eq!(Session::current_index(), <SessionsPerEra as Get<u32>>::get());

		assert!(ElectionProviderMultiPhase::current_phase().is_off());

		roll_to_epm_signed();
		assert!(ElectionProviderMultiPhase::current_phase().is_signed());
	});

	ExtBuilder::default().build_and_execute(|| {
		assert_eq!(active_era(), 0);
		assert_eq!(Session::current_index(), 0);
		assert!(ElectionProviderMultiPhase::current_phase().is_off());

		assert!(start_next_active_era_delayed_solution().is_ok());
		// if the solution is delayed, EPM will end up in emergency mode..
		assert!(ElectionProviderMultiPhase::current_phase().is_emergency());
		// .. era won't progress..
		assert_eq!(active_era(), 0);
		// .. but session does.
		assert_eq!(Session::current_index(), 2);
	})
}

#[test]
/// Replicates the Kusama incident of 8th Dec 2022 and its resolution through the governance
/// fallback.
///
/// After enough slashes exceeded the `Staking::OffendingValidatorsThreshold`, the staking pallet
/// set `Forcing::ForceNew`. When a new session starts, staking will start to force a new era and
/// calls <EPM as election_provider>::elect(). If at this point EPM and the staking miners did not
/// have enough time to queue a new solution (snapshot + solution submission), the election request
/// fails. If there is no election fallback mechanism in place, EPM enters in emergency mode.
/// Recovery: Once EPM is in emergency mode, subsequent calls to `elect()` will fail until a new
/// solution is added to EPM's `QueuedSolution` queue. This can be achieved through
/// `Call::set_emergency_election_result` or `Call::governance_fallback` dispatchables. Once a new
/// solution is added to the queue, EPM phase transitions to `Phase::Off` and the election flow
/// restarts. Note that in this test case, the emergency throttling is disabled.
fn enters_emergency_phase_after_forcing_before_elect() {
	let epm_builder = EpmExtBuilder::default().disable_emergency_throttling();

	ExtBuilder::default().epm(epm_builder).build_and_execute(|| {
		log!(
			trace,
			"current validators (staking): {:?}",
			<Runtime as pallet_staking::SessionInterface<AccountId>>::validators()
		);
		let session_validators_before = Session::validators();

		roll_to_epm_off();
		assert!(ElectionProviderMultiPhase::current_phase().is_off());

		assert_eq!(pallet_staking::ForceEra::<Runtime>::get(), pallet_staking::Forcing::NotForcing);
		// slashes so that staking goes into `Forcing::ForceNew`.
		slash_through_offending_threshold();

		assert_eq!(pallet_staking::ForceEra::<Runtime>::get(), pallet_staking::Forcing::ForceNew);

		advance_session_delayed_solution();
		assert!(ElectionProviderMultiPhase::current_phase().is_emergency());
		log_current_time();

		let era_before_delayed_next = Staking::current_era();
		// try to advance 2 eras.
		assert!(start_next_active_era_delayed_solution().is_ok());
		assert_eq!(Staking::current_era(), era_before_delayed_next);
		assert!(start_next_active_era().is_err());
		assert_eq!(Staking::current_era(), era_before_delayed_next);

		// EPM is still in emergency phase.
		assert!(ElectionProviderMultiPhase::current_phase().is_emergency());

		// session validator set remains the same.
		assert_eq!(Session::validators(), session_validators_before);

		// performs recovery through the set emergency result.
		let supports = to_supports(&vec![
			StakedAssignment { who: 21, distribution: vec![(21, 10)] },
			StakedAssignment { who: 31, distribution: vec![(21, 10), (31, 10)] },
			StakedAssignment { who: 41, distribution: vec![(41, 10)] },
		]);
		assert!(ElectionProviderMultiPhase::set_emergency_election_result(
			RuntimeOrigin::root(),
			supports
		)
		.is_ok());

		// EPM can now roll to signed phase to proceed with elections. The validator set is the
		// expected (ie. set through `set_emergency_election_result`).
		roll_to_epm_signed();
		//assert!(ElectionProviderMultiPhase::current_phase().is_signed());
		assert_eq!(Session::validators(), vec![21, 31, 41]);
		assert_eq!(Staking::current_era(), era_before_delayed_next.map(|e| e + 1));
	});
}

#[test]
/// Continuously slash 10% of the active validators per era.
///
/// Since the `OffendingValidatorsThreshold` is only checked per era staking does not force a new
/// era even as the number of active validators is decreasing across eras. When processing a new
/// slash, staking calculates the offending threshold based on the length of the current list of
/// active validators. Thus, slashing a percentage of the current validators that is lower than
/// `OffendingValidatorsThreshold` will never force a new era. However, as the slashes progress, if
/// the subsequent elections do not meet the minimum election untrusted score, the election will
/// fail and enter in emenergency mode.
fn continous_slashes_below_offending_threshold() {
	let staking_builder = StakingExtBuilder::default().validator_count(10);
	let epm_builder = EpmExtBuilder::default().disable_emergency_throttling();

	ExtBuilder::default()
		.staking(staking_builder)
		.epm(epm_builder)
		.build_and_execute(|| {
			assert_eq!(Session::validators().len(), 10);
			let mut active_validator_set = Session::validators();

			roll_to_epm_signed();

			// set a minimum election score.
			assert!(set_minimum_election_score(500, 1000, 500).is_ok());

			// slash 10% of the active validators and progress era until the minimum trusted score
			// is reached.
			while active_validator_set.len() > 0 {
				let slashed = slash_percentage(Perbill::from_percent(10));
				assert_eq!(slashed.len(), 1);

				// break loop when era does not progress; EPM is in emergency phase as election
				// failed due to election minimum score.
				if start_next_active_era().is_err() {
					assert!(ElectionProviderMultiPhase::current_phase().is_emergency());
					break
				}

				active_validator_set = Session::validators();

				log!(
					trace,
					"slashed 10% of active validators ({:?}). After slash: {:?}",
					slashed,
					active_validator_set
				);
			}
		});
}

#[test]
/// When validators are slashed, they are chilled and removed from the current `VoterList`. Thus,
/// the slashed validator should not be considered in the next validator set. However, if the
/// slashed validator sets its intention to validate again in the same era when it was slashed and
/// chilled, the validator may not be removed from the active validator set across eras, provided
/// it would selected in the subsequent era if there was no slash. Nominators of the slashed
/// validator will also be slashed and chilled, as expected, but the nomination intentions will
/// remain after the validator re-set the intention to be validating again.
///
/// This behaviour is due to removing implicit chill upon slash
/// <https://github.com/paritytech/substrate/pull/12420>.
///
/// Related to <https://github.com/paritytech/substrate/issues/13714>.
fn set_validation_intention_after_chilled() {
	use frame_election_provider_support::SortedListProvider;
	use pallet_staking::{Event, Forcing, Nominators};

	let staking_builder = StakingExtBuilder::default();
	let epm_builder = EpmExtBuilder::default();

	ExtBuilder::default()
		.staking(staking_builder)
		.epm(epm_builder)
		.build_and_execute(|| {
			assert_eq!(active_era(), 0);
			// validator is part of the validator set.
			assert!(Session::validators().contains(&81));
			assert!(<Runtime as pallet_staking::Config>::VoterList::contains(&81));

			// nominate validator 81.
			assert_ok!(Staking::nominate(RuntimeOrigin::signed(21), vec![81]));
			assert_eq!(Nominators::<Runtime>::get(21).unwrap().targets, vec![81]);

			// validator is slashed. it is removed from the `VoterList` through chilling but in the
			// current era, the validator is still part of the active validator set.
			add_slash(&81);
			assert!(Session::validators().contains(&81));
			assert!(!<Runtime as pallet_staking::Config>::VoterList::contains(&81));
			assert_eq!(
				staking_events(),
				[
					Event::Chilled { stash: 81 },
					Event::ForceEra { mode: Forcing::ForceNew },
					Event::SlashReported {
						validator: 81,
						slash_era: 0,
						fraction: Perbill::from_percent(10)
					}
				],
			);

			// after the nominator is slashed and chilled, the nominations remain.
			assert_eq!(Nominators::<Runtime>::get(21).unwrap().targets, vec![81]);

			// validator sets intention to stake again in the same era it was chilled.
			assert_ok!(Staking::validate(RuntimeOrigin::signed(81), Default::default()));

			// progress era and check that the slashed validator is still part of the validator
			// set.
			assert!(start_next_active_era().is_ok());
			assert_eq!(active_era(), 1);
			assert!(Session::validators().contains(&81));
			assert!(<Runtime as pallet_staking::Config>::VoterList::contains(&81));

			// nominations are still active as before the slash.
			assert_eq!(Nominators::<Runtime>::get(21).unwrap().targets, vec![81]);
		})
}
