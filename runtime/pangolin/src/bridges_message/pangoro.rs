// This file is part of Darwinia.
//
// Copyright (C) 2018-2023 Darwinia Network
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

// crates.io
use codec::{Decode, Encode};
use scale_info::TypeInfo;
// paritytech
use frame_support::{weights::Weight, RuntimeDebug};
use sp_runtime::{FixedPointNumber, FixedU128};
// darwinia
use crate::*;
use bp_messages::{source_chain::*, target_chain::*, *};
use bp_polkadot_core::parachains::ParaId;
use bp_runtime::*;
use bridge_runtime_common::{
	lanes::*,
	messages::{source::*, target::*, *},
};
use darwinia_common_runtime::*;

/// Message delivery proof for Pangolin -> Pangoro messages.
pub type ToPangoroMessagesDeliveryProof = FromBridgedChainMessagesDeliveryProof<bp_pangoro::Hash>;
/// Message proof for Pangoro -> Pangolin messages.
pub type FromPangoroMessagesProof = FromBridgedChainMessagesProof<bp_pangoro::Hash>;

/// Message payload for Pangolin -> Pangoro messages.
pub type ToPangoroMessagePayload = FromThisChainMessagePayload<WithPangoroMessageBridge>;
/// Message payload for Pangoro -> Pangolin messages.
pub type FromPangoroMessagePayload = FromBridgedChainMessagePayload<WithPangoroMessageBridge>;

/// Message verifier for Pangolin -> Pangoro messages.
pub type ToPangoroMessageVerifier<R> =
	FromThisChainMessageVerifier<WithPangoroMessageBridge, R, WithPangoroFeeMarket>;

/// Encoded Pangoro Call as it comes from Pangoro.
pub type FromPangoroEncodedCall = FromBridgedChainEncodedMessageCall<RuntimeCall>;

/// Call-dispatch based message dispatch for Pangoro -> Pangolin messages.
pub type FromPangoroMessageDispatch = FromBridgedChainMessageDispatch<
	WithPangoroMessageBridge,
	Runtime,
	Balances,
	WithPangoroDispatch,
>;

pub const INITIAL_PANGORO_TO_PANGOLIN_CONVERSION_RATE: FixedU128 =
	FixedU128::from_inner(FixedU128::DIV);

frame_support::parameter_types! {
	/// Pangolin to Pangoro conversion rate. Initially we treate both tokens as equal.
	pub storage PangoroToPangolinConversionRate: FixedU128 = INITIAL_PANGORO_TO_PANGOLIN_CONVERSION_RATE;
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum PangolinToPangoroParameter {
	/// The conversion formula we use is: `PangoroTokens = PangolinTokens *
	/// conversion_rate`.
	PangoroToPangolinConversionRate(FixedU128),
}
impl Parameter for PangolinToPangoroParameter {
	fn save(&self) {
		match *self {
			PangolinToPangoroParameter::PangoroToPangolinConversionRate(ref conversion_rate) =>
				PangoroToPangolinConversionRate::set(conversion_rate),
		}
	}
}

pub type ToPangoroMaximalOutboundPayloadSize =
	bridge_runtime_common::messages::source::FromThisChainMaximalOutboundPayloadSize<
		WithPangoroMessageBridge,
	>;

/// Pangoro <-> Pangolin message bridge.
#[derive(Clone, Copy, RuntimeDebug)]
pub struct WithPangoroMessageBridge;
impl MessageBridge for WithPangoroMessageBridge {
	type BridgedChain = Pangoro;
	type ThisChain = Pangolin;

	const BRIDGED_CHAIN_ID: bp_runtime::ChainId = PANGORO_CHAIN_ID;
	const BRIDGED_MESSAGES_PALLET_NAME: &'static str =
		bridge_runtime_common::pallets::WITH_PANGOLIN_MESSAGES_PALLET_NAME;
	const RELAYER_FEE_PERCENT: u32 = 10;
	const THIS_CHAIN_ID: bp_runtime::ChainId = PANGOLIN_CHAIN_ID;
}

#[derive(Clone, Copy, RuntimeDebug)]
pub struct Pangolin;
impl ChainWithMessages for Pangolin {
	type AccountId = bp_pangolin::AccountId;
	type Balance = bp_pangolin::Balance;
	type Hash = bp_pangolin::Hash;
	type Signature = bp_pangolin::Signature;
	type Signer = bp_pangolin::AccountPublic;
}
impl ThisChainWithMessages for Pangolin {
	type RuntimeCall = RuntimeCall;
	type RuntimeOrigin = RuntimeOrigin;

	fn is_message_accepted(_send_origin: &Self::RuntimeOrigin, lane: &LaneId) -> bool {
		*lane == PANGORO_PANGOLIN_LANE
	}

	fn maximal_pending_messages_at_outbound_lane() -> MessageNonce {
		MessageNonce::MAX
	}
}

#[derive(Clone, Copy, RuntimeDebug)]
pub struct Pangoro;
impl ChainWithMessages for Pangoro {
	type AccountId = bp_pangoro::AccountId;
	type Balance = bp_pangoro::Balance;
	type Hash = bp_pangoro::Hash;
	type Signature = bp_pangoro::Signature;
	type Signer = bp_pangoro::AccountPublic;
}
impl BridgedChainWithMessages for Pangoro {
	fn maximal_extrinsic_size() -> u32 {
		bp_pangoro::DarwiniaLike::max_extrinsic_size()
	}

	fn verify_dispatch_weight(_message_payload: &[u8], payload_weight: &Weight) -> bool {
		let upper_limit = target::maximal_incoming_message_dispatch_weight(
			bp_pangoro::DarwiniaLike::max_extrinsic_weight(),
		);
		payload_weight.all_lte(upper_limit)
	}
}
impl TargetHeaderChain<ToPangoroMessagePayload, <Self as ChainWithMessages>::AccountId>
	for Pangoro
{
	type Error = &'static str;
	type MessagesDeliveryProof = ToPangoroMessagesDeliveryProof;

	fn verify_message(payload: &ToPangoroMessagePayload) -> Result<(), Self::Error> {
		source::verify_chain_message::<WithPangoroMessageBridge>(payload)
	}

	fn verify_messages_delivery_proof(
		proof: Self::MessagesDeliveryProof,
	) -> Result<(LaneId, InboundLaneData<bp_pangoro::AccountId>), Self::Error> {
		source::verify_messages_delivery_proof_from_parachain::<
			WithPangoroMessageBridge,
			bp_pangoro::Header,
			Runtime,
			WithMoonbaseParachainsInstance,
		>(ParaId(2105), proof)
	}
}
impl SourceHeaderChain<<Self as ChainWithMessages>::Balance> for Pangoro {
	type Error = &'static str;
	type MessagesProof = FromPangoroMessagesProof;

	fn verify_messages_proof(
		proof: Self::MessagesProof,
		messages_count: u32,
	) -> Result<ProvedMessages<Message<<Self as ChainWithMessages>::Balance>>, Self::Error> {
		target::verify_messages_proof_from_parachain::<
			WithPangoroMessageBridge,
			bp_pangoro::Header,
			Runtime,
			WithMoonbaseParachainsInstance,
		>(ParaId(2105), proof, messages_count)
	}
}
