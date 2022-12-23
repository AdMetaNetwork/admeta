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
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::StaticLookup;

	use sp_std::prelude::*;

	pub type TokenIdOf<T> = <T as orml_nft::Config>::TokenId;
	pub type ClassIdOf<T> = <T as orml_nft::Config>::ClassId;
	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

	#[pallet::config]
	pub trait Config: frame_system::Config + orml_nft::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NFTClassCreated(T::AccountId, ClassIdOf<T>),
		NFTMinted(T::AccountId, ClassIdOf<T>, TokenIdOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ClassIdNotFound,
		NoPermission,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_class(
			origin: OriginFor<T>,
			metadata: Vec<u8>,
			data: T::ClassData,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let class_id = orml_nft::Pallet::<T>::create_class(&who, metadata, data)?;
			Self::deposit_event(Event::NFTClassCreated(who, class_id));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn mint(
			origin: OriginFor<T>,
			recipient: <T::Lookup as sp_runtime::traits::StaticLookup>::Source,
			class_id: ClassIdOf<T>,
			metadata: Vec<u8>,
			data: T::TokenData,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let recipient = T::Lookup::lookup(recipient)?;

			let class_info =
				orml_nft::Pallet::<T>::classes(class_id).ok_or(Error::<T>::ClassIdNotFound)?;
			ensure!(who == class_info.owner, Error::<T>::NoPermission);

			let token_id = orml_nft::Pallet::<T>::mint(&recipient, class_id, metadata, data)?;
			Self::deposit_event(Event::NFTMinted(recipient, class_id, token_id));

			Ok(())
		}
	}
}
