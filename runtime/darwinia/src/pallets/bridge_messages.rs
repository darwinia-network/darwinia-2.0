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

use pallet_bridge_messages::Instance1 as WithCrabMessages;

// darwinia
use crate::*;

impl bp_messages::source_chain::SenderOrigin<AccountId> for RuntimeOrigin {
	fn linked_account(&self) -> Option<AccountId> {
		match self.caller {
			OriginCaller::system(frame_system::RawOrigin::Signed(ref submitter)) =>
				Some(*submitter),
			_ => None,
		}
	}
}

frame_support::parameter_types! {
	pub const MaxMessagesToPruneAtOnce: bp_messages::MessageNonce = 8;
	pub const BridgedChainId: bp_runtime::ChainId = bp_runtime::CRAB_CHAIN_ID;
	pub const MaxUnconfirmedMessagesAtInboundLane: bp_messages::MessageNonce =
		bp_darwinia::MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX;
	pub const MaxUnrewardedRelayerEntriesAtInboundLane: bp_messages::MessageNonce =
		bp_darwinia::MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX;
	pub RootAccountForPayments: Option<AccountId> = None;
}

impl pallet_bridge_messages::Config<WithCrabMessages> for Runtime {
	type AccountIdConverter = bp_darwinia::AccountIdConverter;
	type BridgedChainId = BridgedChainId;
	type InboundMessageFee = bp_crab::Balance;
	type InboundPayload = bm_crab::FromCrabMessagePayload;
	type InboundRelayer = bp_crab::AccountId;
	type LaneMessageVerifier = bm_crab::ToCrabMessageVerifier<Self>;
	type MaxMessagesToPruneAtOnce = MaxMessagesToPruneAtOnce;
	type MaxUnconfirmedMessagesAtInboundLane = MaxUnconfirmedMessagesAtInboundLane;
	type MaxUnrewardedRelayerEntriesAtInboundLane = MaxUnrewardedRelayerEntriesAtInboundLane;
	type MaximalOutboundPayloadSize = bm_crab::ToCrabMaximalOutboundPayloadSize;
	type MessageDeliveryAndDispatchPayment = pallet_fee_market::s2s::FeeMarketPayment<Self, WithCrabFeeMarket, Balances>;
	type MessageDispatch = bm_crab::FromCrabMessageDispatch;
	type OnDeliveryConfirmed = pallet_fee_market::s2s::FeeMarketMessageConfirmedHandler<Self, WithCrabFeeMarket>;
	type OnMessageAccepted = pallet_fee_market::s2s::FeeMarketMessageAcceptedHandler<Self, WithCrabFeeMarket>;
	type OutboundMessageFee = bp_darwinia::Balance;
	type OutboundPayload = bm_crab::ToCrabMessagePayload;
	type Parameter = bm_crab::DarwiniaToCrabParameter;
	type RuntimeEvent = RuntimeEvent;
	type SourceHeaderChain = bm_crab::Crab;
	type TargetHeaderChain = bm_crab::Crab;
	type WeightInfo = ();
}
