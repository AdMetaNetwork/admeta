#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use admeta_common::{AdData, TargetTag};
	use codec::{Decode, Encode};
	use frame_support::{
		dispatch::DispatchResult, pallet_prelude::*, traits::Randomness, BoundedVec,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded};
	use sp_std::prelude::*;

	/// This defines user
	#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct User<T: Config> {
		pub age: u8,
		pub tag: TargetTag,
		pub ad_display: bool,
		pub matched_ads: BoundedVec<(T::AccountId, T::AdIndex), T::MaxMatchedAds>,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type AdData: AdData<Self::BlockNumber, Self::AdIndex, Self::AccountId>;
		type AdIndex: Parameter
			+ MaybeSerializeDeserialize
			+ Bounded
			+ AtLeast32BitUnsigned
			+ Copy
			+ MaxEncodedLen
			+ Default;

		/// Maximum number of matched ads per user
		#[pallet::constant]
		type MaxMatchedAds: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_user)]
	pub type Users<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, User<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NewUserAdded(T::AccountId),
		UserSetAdDisplay(T::AccountId, bool),
		RewardClaimed(T::AccountId, T::AdIndex),
		RewardNotClaimed(T::AccountId, T::AdIndex),
	}

	#[pallet::error]
	pub enum Error<T> {
		UserAlreadyExists,
		UserDoesNotExist,
		AdNotForThisUser,
		RewardClaimPaymentError,
		MatchedAdsBoundaryExceeded,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Matching happens in every block's on_idle() function, to avoid congestion
		fn on_idle(block_number: T::BlockNumber, remaining_weight: Weight) -> Weight {
			log::info!("on_idle #{:?}, {:?})", block_number, remaining_weight);
			Self::do_matching(block_number);
			// TODO calculate the actual consumed weights
			300
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn add_profile(origin: OriginFor<T>, age: u8, tag: TargetTag) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check if user exists
			if Users::<T>::contains_key(&who) {
				Err(Error::<T>::UserAlreadyExists.into())
			} else {
				let user = User::<T> {
					age,
					tag,
					ad_display: false,
					// try_into() should be always successful
					matched_ads: Vec::new().try_into().unwrap(),
				};
				Users::<T>::insert(who.clone(), user);
				Self::deposit_event(Event::NewUserAdded(who));
				Ok(())
			}
		}
		#[pallet::weight(10_000)]
		pub fn set_ad_display(origin: OriginFor<T>, ad_display: bool) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Set ad_display
			Users::<T>::mutate(who.clone(), |user_op| {
				if let Some(user) = user_op {
					user.ad_display = ad_display;
					Self::deposit_event(Event::UserSetAdDisplay(who, ad_display));
					Ok(())
				} else {
					Err(Error::<T>::UserDoesNotExist)
				}
			})?;

			Ok(())
		}
		#[pallet::weight(100)]
		pub fn claim_reward(
			origin: OriginFor<T>,
			proposer: T::AccountId,
			ad_index: T::AdIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Users::<T>::mutate(who.clone(), |user_op| {
				if let Some(user) = user_op {
					let mut ad_claimed = false;

					// TODO Refactor this code, to find the id and remove it instead of iterating
					// the whole vector
					user.matched_ads.retain(|(ad_proposer, ad_id)| {
						// Only the ad_id that equals to ad_index gets removed
						if *ad_proposer == proposer && *ad_id == ad_index {
							ad_claimed = true;
							false
						} else {
							true
						}
					});

					if ad_claimed {
						T::AdData::claim_reward_for_user(proposer, ad_index, who.clone())
							.map_err(|_| Error::<T>::RewardClaimPaymentError)?;
						Self::deposit_event(Event::RewardClaimed(who, ad_index));
						Ok(())
					} else {
						Self::deposit_event(Event::RewardNotClaimed(who, ad_index));
						Err(Error::<T>::AdNotForThisUser)
					}
				} else {
					Err(Error::<T>::UserDoesNotExist)
				}
			})?;

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Do matching for users and ads
		fn do_matching(block_number: T::BlockNumber) {
			for iter in Users::<T>::iter() {
				// Start matching if there is no matched ads and ad_display is true
				if (iter.1.matched_ads.len() as u32) < T::MaxMatchedAds::get() && iter.1.ad_display
				{
					if let Some((ad_proposer, ad_index)) =
						T::AdData::match_ad_for_user(iter.1.age, iter.1.tag, block_number)
					{
						// Push matched ad to user's matched_ad vector
						Users::<T>::mutate(&iter.0, |user_op| {
							if let Some(user) = user_op {
								// Err should never thrown here
								let _ = user.matched_ads.try_push((ad_proposer, ad_index));
							}
						});
					}
				} else {
					continue
				}
			}
		}
	}
}
