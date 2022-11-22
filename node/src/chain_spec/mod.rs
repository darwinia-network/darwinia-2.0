mod darwinia;
pub use darwinia::{
	self as darwinia_chain_spec, ChainSpec as DarwiniaChainSpec
};

mod crab;
pub use darwinia::{
	self as crab_chain_spec, ChainSpec as CrabChainSpec
};

mod pangolin;
pub use darwinia::{
	self as pangolin_chain_spec, ChainSpec as PangolinChainSpec
};

// darwinia
use dc_primitives::*;
// substrate
use sp_core::{Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

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