#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::pallet_prelude::{ValueQuery, *};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proof<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, BlockNumberFor<T>),
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
		ClaimTransfered(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ClaimTooLong,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_claim(
			origin: OriginFor<T>,
			claim: BoundedVec<u8, T::MaxClaimLength>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proof::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			Proof::<T>::insert(&claim, (sender.clone(), frame_system::Pallet::<T>::block_number()));

			Self::deposit_event(Event::ClaimCreated(sender, claim));

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			claim: BoundedVec<u8, T::MaxClaimLength>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proof::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let owner = Proof::<T>::get(&claim)
				.unwrap_or_else(|| return Err(Error::<T>::ClaimNotExist.into()))
				.0;

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proof::<T>::remove(&claim);

			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			claim: BoundedVec<u8, T::MaxClaimLength>,
			dest: T::AccountId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proof::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let owner = Proof::<T>::get(&claim)
				.unwrap_or_else(|| return Err(Error::<T>::ClaimNotExist.into()))
				.0;

			let ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proof::<T>::insert(&claim, (dest.clone(), frame_system::Pallet::<T>::block_number()));

			Self::deposit_event(Event::ClaimTransfered(sender, dest, claim));

			Ok(())
		}
	}
}
