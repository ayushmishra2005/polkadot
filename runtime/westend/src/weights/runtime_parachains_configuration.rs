// Copyright 2017-2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `runtime_parachains::configuration`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::configuration
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/runtime_parachains_configuration.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for runtime_parachains::configuration.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::configuration::WeightInfo for WeightInfo<T> {
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_validation_upgrade_frequency() -> Weight {
		(9_127_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_validation_upgrade_delay() -> Weight {
		(9_294_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_code_retention_period() -> Weight {
		(12_984_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_max_code_size() -> Weight {
		(13_002_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_max_pov_size() -> Weight {
		(12_987_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_max_head_data_size() -> Weight {
		(12_934_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_parathread_cores() -> Weight {
		(12_917_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_parathread_retries() -> Weight {
		(12_933_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_group_rotation_frequency() -> Weight {
		(12_925_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_chain_availability_period() -> Weight {
		(12_855_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_thread_availability_period() -> Weight {
		(13_047_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_scheduling_lookahead() -> Weight {
		(12_943_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_max_validators_per_core() -> Weight {
		(13_080_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_max_validators() -> Weight {
		(13_083_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_dispute_period() -> Weight {
		(12_889_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_dispute_post_conclusion_acceptance_period() -> Weight {
		(12_883_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_dispute_max_spam_slots() -> Weight {
		(12_946_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_dispute_conclusion_by_time_out_period() -> Weight {
		(12_836_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_no_show_slots() -> Weight {
		(12_923_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_n_delay_tranches() -> Weight {
		(13_015_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_zeroth_delay_tranche_width() -> Weight {
		(13_051_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_needed_approvals() -> Weight {
		(12_932_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_relay_vrf_modulo_samples() -> Weight {
		(13_026_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_max_upward_queue_count() -> Weight {
		(12_938_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_max_upward_queue_size() -> Weight {
		(12_915_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_max_downward_message_size() -> Weight {
		(9_444_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_ump_service_total_weight() -> Weight {
		(12_985_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_max_upward_message_size() -> Weight {
		(12_931_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_max_upward_message_num_per_candidate() -> Weight {
		(13_088_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn set_hrmp_open_request_ttl() -> Weight {
		(2_000_000_000_000 as Weight)
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_hrmp_sender_deposit() -> Weight {
		(13_052_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_hrmp_recipient_deposit() -> Weight {
		(12_806_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_hrmp_channel_max_capacity() -> Weight {
		(12_976_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_hrmp_channel_max_total_size() -> Weight {
		(12_966_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_hrmp_max_parachain_inbound_channels() -> Weight {
		(13_042_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_hrmp_max_parathread_inbound_channels() -> Weight {
		(12_948_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_hrmp_channel_max_message_size() -> Weight {
		(12_915_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_hrmp_max_parachain_outbound_channels() -> Weight {
		(12_911_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_hrmp_max_parathread_outbound_channels() -> Weight {
		(12_717_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration PendingConfig (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	fn set_hrmp_max_message_num_per_candidate() -> Weight {
		(12_776_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
