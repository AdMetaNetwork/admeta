#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::prelude::*;

pub trait AdData<BlockNumberType, AdIndexType, AccountIdType> {
	fn match_ad_for_user(
		age: u8,
		tag: TargetTag,
		block_number: BlockNumberType,
	) -> Option<(AccountIdType, AdIndexType)>;
	fn claim_reward_for_user(
		proposer: AccountIdType,
		ad_index: AdIndexType,
		user: AccountIdType,
	) -> DispatchResult;
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct ValueRange {
	min: u8,
	max: u8,
}

impl ValueRange {
	/// Check if min value is less or equal to max value
	pub fn self_check(&self) -> bool {
		self.min <= self.max
	}
	/// Check if the given value is in range
	pub fn is_in_range(&self, v: u8) -> bool {
		v >= self.min && v <= self.max
	}
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum TargetTag {
	DeFi,
	GameFi,
	Metaverse,
}

/// This struct specifies what kinds of conditions the target group should fulfill
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct AdPreference {
	pub age: ValueRange,
	pub tags: Vec<TargetTag>,
}
