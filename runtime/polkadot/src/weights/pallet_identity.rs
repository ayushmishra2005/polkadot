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
//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/pallet_identity.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_identity.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity Registrars (r:1 w:1)
	fn add_registrar(r: u32, ) -> Weight {
		(20_155_000 as Weight)
			// Standard Error: 32_000
			.saturating_add((401_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	fn set_identity(r: u32, x: u32, ) -> Weight {
		(49_692_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((269_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 2_000
			.saturating_add((1_222_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:1 w:1)
	fn set_subs_new(s: u32, ) -> Weight {
		(40_188_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((6_476_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:1)
	fn set_subs_old(p: u32, ) -> Weight {
		(38_386_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((2_103_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(50_788_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((32_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((2_080_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 1_000
			.saturating_add((666_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		(51_067_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((327_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((1_324_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		(46_593_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((226_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((1_302_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	fn set_fee(r: u32, ) -> Weight {
		(8_653_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((324_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	fn set_account_id(r: u32, ) -> Weight {
		(8_751_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((322_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	fn set_fields(r: u32, ) -> Weight {
		(8_558_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((325_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		(33_343_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((289_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((1_317_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(61_987_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((89_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((2_091_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 1_000
			.saturating_add((9_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	fn add_sub(s: u32, ) -> Weight {
		(53_828_000 as Weight)
			// Standard Error: 0
			.saturating_add((213_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	fn rename_sub(s: u32, ) -> Weight {
		(16_636_000 as Weight)
			// Standard Error: 0
			.saturating_add((21_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	fn remove_sub(s: u32, ) -> Weight {
		(54_576_000 as Weight)
			// Standard Error: 0
			.saturating_add((186_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	fn quit_sub(s: u32, ) -> Weight {
		(33_552_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((193_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
