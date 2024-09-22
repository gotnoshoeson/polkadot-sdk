// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_xcm_bridge_hub_router`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-696hpswk-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("asset-hub-rococo-dev")`, DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/polkadot-sdk/.git/.artifacts/bench.json
// --pallet=pallet_xcm_bridge_hub_router
// --chain=asset-hub-rococo-dev
// --header=./cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/assets/asset-hub-rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_xcm_bridge_hub_router`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm_bridge_hub_router::WeightInfo for WeightInfo<T> {
	/// Storage: `XcmpQueue::InboundXcmpSuspended` (r:1 w:0)
	/// Proof: `XcmpQueue::InboundXcmpSuspended` (`max_values`: Some(1), `max_size`: Some(4002), added: 4497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: Some(1282), added: 1777, mode: `MaxEncodedLen`)
	/// Storage: `ToWestendXcmRouter::DeliveryFeeFactor` (r:1 w:1)
	/// Proof: `ToWestendXcmRouter::DeliveryFeeFactor` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn on_initialize_when_non_congested() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153`
		//  Estimated: `5487`
		// Minimum execution time: 12_993_000 picoseconds.
		Weight::from_parts(13_428_000, 0)
			.saturating_add(Weight::from_parts(0, 5487))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcmpQueue::InboundXcmpSuspended` (r:1 w:0)
	/// Proof: `XcmpQueue::InboundXcmpSuspended` (`max_values`: Some(1), `max_size`: Some(4002), added: 4497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: Some(1282), added: 1777, mode: `MaxEncodedLen`)
	fn on_initialize_when_congested() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `5487`
		// Minimum execution time: 6_305_000 picoseconds.
		Weight::from_parts(6_536_000, 0)
			.saturating_add(Weight::from_parts(0, 5487))
			.saturating_add(T::DbWeight::get().reads(2))
	}
}
