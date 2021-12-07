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
	use codec::{Decode, Encode};
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{Currency, OnUnbalanced, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, Saturating};
	use sp_std::prelude::*;

	pub type Url = Vec<u8>;
	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::NegativeImbalance;

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

	/// This defines impression ads, which pays by CPI
	#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct ImpressionAd<T: Config> {
		// The account who proposed this ad
		pub proposer: T::AccountId,
		// The URL where this ad's metadata stores
		pub metadata: Url,
		// The bond reserved for this ad
		pub bond: BalanceOf<T>,
		// The cost per impression (CPI)
		pub cpi: BalanceOf<T>,
		// The total number of impressions in this ad
		pub amount: u32,
		// The end block of this ad
		pub end_block: BlockNumberOf<T>,
		// The preference of target group
		pub preference: AdPreference,
		// The approval status
		pub approved: bool,
	}

	// TODO ActionAd will be implemented

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type AdIndex: Parameter
			+ MaybeSerializeDeserialize
			+ Bounded
			+ AtLeast32BitUnsigned
			+ Copy
			+ MaxEncodedLen
			+ Default;

		/// Origin from which approvals must come.
		type ApproveOrigin: EnsureOrigin<Self::Origin>;

		/// Origin from which rejections must come.
		type RejectOrigin: EnsureOrigin<Self::Origin>;

		/// Handler for the unbalanced decrease when slashing for a rejected proposal.
		type OnSlash: OnUnbalanced<NegativeImbalanceOf<Self>>;

		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// Maximum acceptable Ad metadata length

		#[pallet::constant]
		type MaxAdDataLength: Get<u32>;

		/// The base deposit amount of an ad proposal
		#[pallet::constant]
		type AdDepositBase: Get<BalanceOf<Self>>;

		/// The deposit amount per byte of an ad's metadata
		#[pallet::constant]
		type AdDepositPerByte: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Number of ad proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn ad_count)]
	pub type AdCount<T: Config> = StorageValue<_, T::AdIndex, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn impression_ads)]
	/// Impression ads storage
	pub type ImpressionAds<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AdIndex, ImpressionAd<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NewAdProposal(T::AdIndex, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		AdCountOverflow,
		AdDataTooLarge,
		InsufficientProposalBalance,
		InvalidAdIndex,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn propose_ad(
			origin: OriginFor<T>,
			ad_url: Url,
			cpi: BalanceOf<T>,
			amount: u32,
			end_block: BlockNumberOf<T>,
			ad_preference: AdPreference,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::create_proposal(who, ad_url, cpi, amount, end_block, ad_preference)?;

			Ok(())
		}
		#[pallet::weight(10_000)]
		pub fn approve_ad(origin: OriginFor<T>, ad_index: T::AdIndex) -> DispatchResult {
			T::ApproveOrigin::ensure_origin(origin)?;

			// Set approved to true
			ImpressionAds::<T>::mutate(ad_index, |ad_op| {
				if let Some(ad) = ad_op {
					ad.approved = true;
					Ok(())
				} else {
					Err(Error::<T>::InvalidAdIndex)
				}
			})?;

			Ok(())
		}
		#[pallet::weight(10_000)]
		pub fn reject_ad(origin: OriginFor<T>, ad_index: T::AdIndex) -> DispatchResult {
			T::RejectOrigin::ensure_origin(origin)?;

			// Slash the bond and unreserve the ad payment
			let ad = Self::impression_ads(ad_index).ok_or(Error::<T>::InvalidAdIndex)?;
			let imbalance = T::Currency::slash_reserved(&ad.proposer, ad.bond).0;
			T::OnSlash::on_unbalanced(imbalance);
			let ad_cost = ad.cpi * ad.amount.into();
			T::Currency::unreserve(&ad.proposer, ad_cost);
			// Remove this proposal
			ImpressionAds::<T>::remove(ad_index);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Calculate the next ad index
		fn next_ad_id() -> Result<T::AdIndex, Error<T>> {
			match Self::ad_count() {
				Some(id) => {
					// Ensure id won't overflow
					ensure!(id < T::AdIndex::max_value(), Error::<T>::AdCountOverflow);
					Ok(id.saturating_add(T::AdIndex::from(1u8)))
				},
				// Start count from 1
				None => Ok(T::AdIndex::min_value().saturating_add(T::AdIndex::from(1u8))),
			}
		}
		/// Create an ad proposal
		fn create_proposal(
			who: T::AccountId,
			ad_url: Url,
			cpi: BalanceOf<T>,
			amount: u32,
			end_block: BlockNumberOf<T>,
			ad_preference: AdPreference,
		) -> Result<(), Error<T>> {
			ensure!(ad_url.len() <= T::MaxAdDataLength::get() as usize, Error::<T>::AdDataTooLarge);

			let ad_index = Self::next_ad_id()?;

			let bond =
				T::AdDepositBase::get() + T::AdDepositPerByte::get() * (ad_url.len() as u32).into();

			let ad_cost = cpi * amount.into();

			T::Currency::reserve(&who, bond + ad_cost)
				.map_err(|_| Error::<T>::InsufficientProposalBalance)?;

			let ad = ImpressionAd::<T> {
				proposer: who.clone(),
				metadata: ad_url,
				bond,
				cpi,
				amount,
				end_block,
				preference: ad_preference,
				approved: false,
			};

			<ImpressionAds<T>>::insert(ad_index, ad);

			Self::deposit_event(Event::NewAdProposal(ad_index, who));

			Ok(())
		}
	}
}
