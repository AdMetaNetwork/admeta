#![cfg_attr(not(feature = "std"), no_std)]

pub trait AdData<AdIndexType> {
	fn decrease_ad_amount(new_amount: AdIndexType);
}
