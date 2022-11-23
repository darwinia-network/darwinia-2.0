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

// crates.io
use codec::{Decode, Encode};
use scale_info::TypeInfo;
// paritytech
use frame_support::{weights::Weight, RuntimeDebug};
use sp_runtime::{FixedPointNumber, FixedU128};
use sp_std::ops::RangeInclusive;
// darwinia
use crate::*;
use bp_messages::{source_chain::*, target_chain::*, *};
use bp_runtime::*;
use bridge_runtime_common::{
	lanes::*,
	messages::{source::*, target::*, *},
};

/// Message delivery proof for CrabParachain -> Crab messages.
pub type ToCrabMessagesDeliveryProof = FromBridgedChainMessagesDeliveryProof<bp_crab::Hash>;
/// Message proof for Crab -> CrabParachain  messages.
pub type FromCrabMessagesProof = FromBridgedChainMessagesProof<bp_crab::Hash>;

/// Message payload for CrabParachain -> Crab messages.
pub type ToCrabMessagePayload = FromThisChainMessagePayload<WithCrabMessageBridge>;
/// Message payload for Crab -> CrabParachain messages.
pub type FromCrabMessagePayload = FromBridgedChainMessagePayload<WithCrabMessageBridge>;

/// Message verifier for CrabParachain -> Crab messages.
pub type ToCrabMessageVerifier<R> =
FromThisChainMessageVerifier<WithCrabMessageBridge, R, WithCrabFeeMarket>;

/// Encoded Crab Call as it comes from Crab.
pub type FromCrabEncodedCall = FromBridgedChainEncodedMessageCall<Call>;

/// Call-dispatch based message dispatch for Crab -> CrabParachain messages.
pub type FromCrabMessageDispatch =
FromBridgedChainMessageDispatch<WithCrabMessageBridge, Runtime, Ring, WithCrabDispatch>;

pub const INITIAL_CRAB_TO_CRAB_PARACHAIN_CONVERSION_RATE: FixedU128 =
	FixedU128::from_inner(FixedU128::DIV);

frame_support::parameter_types! {
	/// CrabParachain to Crab conversion rate. Initially we trate both tokens as equal.
	pub storage CrabToCrabParachainConversionRate: FixedU128 = INITIAL_CRAB_TO_CRAB_PARACHAIN_CONVERSION_RATE;
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum CrabParachainToCrabParameter {
	/// The conversion formula we use is: `CrabTokens = CrabParachainTokens *
	/// conversion_rate`.
	CrabToCrabParachainConversionRate(FixedU128),
}
impl Parameter for CrabParachainToCrabParameter {
	fn save(&self) {
		match *self {
			CrabParachainToCrabParameter::CrabToCrabParachainConversionRate(
				ref conversion_rate,
			) => CrabToCrabParachainConversionRate::set(conversion_rate),
		}
	}
}

/// Crab <-> CrabParachain message bridge.
#[derive(Clone, Copy, RuntimeDebug)]
pub struct WithCrabMessageBridge;
impl MessageBridge for WithCrabMessageBridge {
	type BridgedChain = Crab;
	type ThisChain = CrabParachain;

	const BRIDGED_CHAIN_ID: ChainId = CRAB_CHAIN_ID;
	const BRIDGED_MESSAGES_PALLET_NAME: &'static str =
		bp_crab_parachain::WITH_CRAB_PARACHAIN_MESSAGES_PALLET_NAME;
	const RELAYER_FEE_PERCENT: u32 = 10;
	const THIS_CHAIN_ID: ChainId = CRAB_PARACHAIN_CHAIN_ID;
}

#[derive(Clone, Copy, RuntimeDebug)]
pub struct CrabParachain;
impl ChainWithMessages for CrabParachain {
	type AccountId = bp_crab_parachain::AccountId;
	type Balance = bp_crab_parachain::Balance;
	type Hash = bp_crab_parachain::Hash;
	type Signature = bp_crab_parachain::Signature;
	type Signer = bp_crab_parachain::AccountPublic;
	type Weight = Weight;
}
impl ThisChainWithMessages for CrabParachain {
	type Call = Call;
	type Origin = Origin;

	fn is_message_accepted(_send_origin: &Self::Origin, lane: &LaneId) -> bool {
		*lane == CRAB_CRAB_PARACHAIN_LANE
	}

	fn maximal_pending_messages_at_outbound_lane() -> MessageNonce {
		MessageNonce::MAX
	}
}

#[derive(Clone, Copy, RuntimeDebug)]
pub struct Crab;
impl ChainWithMessages for Crab {
	type AccountId = bp_crab::AccountId;
	type Balance = bp_crab::Balance;
	type Hash = bp_crab::Hash;
	type Signature = bp_crab::Signature;
	type Signer = bp_crab::AccountPublic;
	type Weight = Weight;
}
impl BridgedChainWithMessages for Crab {
	fn maximal_extrinsic_size() -> u32 {
		bp_crab::Crab::max_extrinsic_size()
	}

	fn message_weight_limits(_message_payload: &[u8]) -> RangeInclusive<Self::Weight> {
		let upper_limit =
			target::maximal_incoming_message_dispatch_weight(bp_crab::Crab::max_extrinsic_weight());
		0..=upper_limit
	}
}
impl TargetHeaderChain<ToCrabMessagePayload, <Self as ChainWithMessages>::AccountId> for Crab {
	type Error = &'static str;
	type MessagesDeliveryProof = ToCrabMessagesDeliveryProof;

	fn verify_message(payload: &ToCrabMessagePayload) -> Result<(), Self::Error> {
		source::verify_chain_message::<WithCrabMessageBridge>(payload)
	}

	fn verify_messages_delivery_proof(
		proof: Self::MessagesDeliveryProof,
	) -> Result<(LaneId, InboundLaneData<bp_crab::AccountId>), Self::Error> {
		source::verify_messages_delivery_proof::<WithCrabMessageBridge, Runtime, WithCrabGrandpa>(
			proof,
		)
	}
}
impl SourceHeaderChain<<Self as ChainWithMessages>::Balance> for Crab {
	type Error = &'static str;
	type MessagesProof = FromCrabMessagesProof;

	fn verify_messages_proof(
		proof: Self::MessagesProof,
		messages_count: u32,
	) -> Result<ProvedMessages<Message<<Self as ChainWithMessages>::Balance>>, Self::Error> {
		target::verify_messages_proof::<WithCrabMessageBridge, Runtime, WithCrabGrandpa>(
			proof,
			messages_count,
		)
	}
}