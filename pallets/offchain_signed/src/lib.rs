// #![cfg_attr(not(feature = "std"), no_std)]
// // Re-export pallet items so that they can be accessed from the crate namespace.
// pub use pallet::*;

// #[frame_support::pallet]
// pub mod pallet {
// 	use frame_support::pallet_prelude::*;
// 	use frame_system::pallet_prelude::*;

// 	#[pallet::pallet]
// 	#[pallet::generate_store(pub(super) trait Store)]
// 	pub struct Pallet<T>(_);

// 	/// Configure the pallet by specifying the parameters and types on which it depends.
// 	#[pallet::config]
// 	pub trait Config: frame_system::Config {
// 		/// Because this pallet emits events, it depends on the runtime's definition of an event.
// 		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
// 	}
// 	// Pallets use events to inform users when important changes are made.
// 	// Event documentation should end with an array that provides descriptive names for parameters.
// 	#[pallet::event]
// 	#[pallet::generate_deposit(pub(super) fn deposit_event)]
// 	pub enum Event<T: Config> {
// 		/// Event emitted when a claim has been created.
// 		ClaimCreated { who: T::AccountId, claim: T::Hash },
// 		/// Event emitted when a claim is revoked by the owner.
// 		ClaimRevoked { who: T::AccountId, claim: T::Hash },
// 	}
// 	#[pallet::error]
// 	pub enum Error<T> {
// 		/// The claim already exists.
// 		AlreadyClaimed,
// 		/// The claim does not exist, so it cannot be revoked.
// 		NoSuchClaim,
// 		/// The claim is owned by another account, so caller can't revoke it.
// 		NotClaimOwner,
// 	}
// 	#[pallet::storage]
// 	pub(super) type Claims<T: Config> =
// 		StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber)>;

// 	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
// 	// These functions materialize as "extrinsics", which are often compared to transactions.
// 	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
// 	#[pallet::call]
// 	impl<T: Config> Pallet<T> {
// 		#[pallet::weight(0)]
// 		#[pallet::call_index(1)]
// 		pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
// 			// Check that the extrinsic was signed and get the signer.
// 			// This function will return an error if the extrinsic is not signed.
// 			let sender = ensure_signed(origin)?;

// 			// Verify that the specified claim has not already been stored.
// 			ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);

// 			// Get the block number from the FRAME System pallet.
// 			let current_block = <frame_system::Pallet<T>>::block_number();

// 			// Store the claim with the sender and block number.
// 			Claims::<T>::insert(&claim, (&sender, current_block));

// 			// Emit an event that the claim was created.
// 			Self::deposit_event(Event::ClaimCreated { who: sender, claim });

// 			Ok(())
// 		}

// 		#[pallet::weight(0)]
// 		#[pallet::call_index(2)]
// 		pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
// 			// Check that the extrinsic was signed and get the signer.
// 			// This function will return an error if the extrinsic is not signed.
// 			let sender = ensure_signed(origin)?;

// 			// Get owner of the claim, if none return an error.
// 			let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

// 			// Verify that sender of the current call is the claim owner.
// 			ensure!(sender == owner, Error::<T>::NotClaimOwner);

// 			// Remove claim from storage.
// 			Claims::<T>::remove(&claim);

// 			// Emit an event that the claim was erased.
// 			Self::deposit_event(Event::ClaimRevoked { who: sender, claim });
// 			Ok(())
// 		}
// 	}
// }

#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

use sp_core::{crypto::KeyTypeId};

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify, MultiSignature, MultiSigner
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
	type RuntimeAppPublic = Public;
	type GenericSignature = sp_core::sr25519::Signature;
	type GenericPublic = sp_core::sr25519::Public;
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use log::{info, error};
	use frame_system::offchain::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
	// pub trait Config: frame_system::Config {
			/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		type Call: From<Call<Self>>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			log::info!("###### Called do_something.");
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Offchain worker entry point.
		///
		/// By implementing `fn offchain_worker` you declare a new offchain worker.
		/// This function will be called when the node is fully synced and a new best block is
		/// successfully imported.
		/// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
		/// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
		/// so the code should be able to handle that.
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!("##### Hello from pallet-ocw.");
			// The entry point of your code called by offchain worker
			let something:u32 = 22;
			let signer = Signer::<T, T::AuthorityId>::all_accounts();
			log::info!("##### offchain_worker step 1");
			let results = signer.send_signed_transaction(|_account| {
				// Call::on_chain_call { key: val }
				Call::do_something { something }
			});
			log::info!("##### offchain_worker step 2");
			for (acc, res) in &results {
				match res {
					Ok(()) => log::info!("##### [{:?}]: submit transaction success.", acc.id),
					Err(e) => log::info!("###### [{:?}]: submit transaction failure. Reason: {:?}", acc.id, e),
				}
			}
			log::info!("##### offchain_worker step 3");

		
			// Ok(())		
		}
		// ...
	}
}
