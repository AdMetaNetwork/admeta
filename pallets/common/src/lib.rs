#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::prelude::*;

pub trait AdData<AdIndexType> {
	fn decrease_ad_amount(new_amount: AdIndexType);
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct ValueRange {
	min: u8,
	max: u8,
}

impl ValueRange {
	fn is_in_range(&self, v: u8) -> bool {
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
