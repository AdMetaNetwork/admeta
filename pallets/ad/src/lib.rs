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
	use codec::{Codec, Decode, Encode};
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement, Randomness, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, Saturating, StaticLookup};

	pub type Url = Vec<u8>;
	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

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
	pub struct ImpressionAd<AccountId, Balance, BlockNumber> {
		// The account who proposed this ad
		pub proposer: AccountId,
		// The URL where this ad's metadata stores
		pub metadata: Url,
		// The bond reserved for this ad
		pub bond: Balance,
		// The cost per impression (CPI)
		pub cpi: Balance,
		// The total number of impressions in this ad
		pub amount: u32,
		// The end block of this ad
		pub end_block: BlockNumber,
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
	pub type ImpressionAds<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AdIndex,
		ImpressionAd<T::AccountId, BalanceOf<T>, BlockNumberOf<T>>,
		OptionQuery,
	>;

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
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
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

			let ad = ImpressionAd {
				proposer: who.clone(),
				metadata: ad_url,
				bond,
				cpi,
				amount,
				end_block,
				preference: ad_preference,
				approved: false,
			};

			ImpressionAds::insert(ad_index, ad);
			//AdCount::<T>::put(ad_index.saturating_add(T::AdIndex::from(1u8)));

			Self::deposit_event(Event::NewAdProposal(ad_index, who));

			Ok(())
		}
	}
}
