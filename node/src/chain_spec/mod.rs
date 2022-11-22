// This file is part of Darwinia.
//
// Copyright (C) 2018-2022 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

pub mod darwinia;
pub use darwinia::{self as darwinia_chain_spec, ChainSpec as DarwiniaChainSpec};

pub mod crab;
pub use darwinia::{self as crab_chain_spec, ChainSpec as CrabChainSpec};

pub mod pangolin;
pub use darwinia::{self as pangolin_chain_spec, ChainSpec as PangolinChainSpec};

// crates.io
use serde::{Deserialize, Serialize};
// darwinia
use dc_primitives::*;
// substrate
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// These are are testnet-only keys.
// address     = "0x75a1807b6aff253070b96ed9e43c0c5c17c7e1d4"
// public-key  = "0x036ae37e38766cd9be397dfd42486d8aeb46c30d4b0526d12dc9f5eb6a8e4c09f5"
// secret-seed = "0x63c24046f3b744c8cd8f74e91d9e3603577035f3119ac1389db0f461e591375d"
#[allow(unused)]
const COLLATOR_A: &str = "0x75a1807b6aff253070b96ed9e43c0c5c17c7e1d4";
// address     = "0x5f69def84585715b92d397b1e92d4bf26d48d6b7"
// public-key  = "0x03f6230f7fd8bd24a3814753c5bdd0417d5e00149e15b4bac130887e925c6a53a0"
// secret-seed = "0xee92a5c93339cb59bdad8c088c1b3adbae7ec94110681d871ab3beb8ac6530b2"
#[allow(unused)]
const COLLATOR_B: &str = "0x5f69def84585715b92d397b1e92d4bf26d48d6b7";
// address     = "0xa4e3cf11462254ed4b7ce00079eb11ca2a8b5393"
// public-key  = "0x0218893313cc713836d57c60daedd28ee0b0823a167469af37e16f970fdb5305ef"
// secret-seed = "0x68ade89c684eb715ad047d9a54f8a07457840194091622736d742503d148966a"
#[allow(unused)]
const COLLATOR_C: &str = "0xa4e3cf11462254ed4b7ce00079eb11ca2a8b5393";

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// This is the simplest bytecode to revert without returning any data.
/// We will pre-deploy it under all of our precompiles to ensure they can be called from within
/// contracts. (PUSH1 0x00 PUSH1 0x00 REVERT)
pub const REVERT_BYTECODE: [u8; 5] = [0x60, 0x00, 0x60, 0x00, 0xFD];

type AccountPublic = <Signature as Verify>::Signer;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}
impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_from_seed::<AuraId>(seed)
}
