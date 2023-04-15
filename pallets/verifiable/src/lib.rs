#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod types;
mod weights;

pub use pallet::*;
pub use types::*;
pub use weights::WeightInfo;

use frame_support::{
	dispatch::DispatchResultWithPostInfo,
	ensure,
	traits::{Get, StorageVersion},
	BoundedVec,
};
use frame_system::pallet_prelude::*;
use sp_core::sp_std::str;
use sp_std::prelude::*;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use core::marker::PhantomData;

	use frame_support::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MetadataSize: Get<u32>;

		#[pallet::constant]
		type MaxDIDsPerAccount: Get<u32>;

		#[pallet::constant]
		type PublicKeySize: Get<u32>;

		#[pallet::constant]
		type VCFingerPrintSize: Get<u32>;

		#[pallet::constant]
		type DIDURISize: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn get_did_document)]
	pub type DIDDocument<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::DIDURISize>,
		DID<T::AccountId, T::BlockNumber, T::MetadataSize>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_did_accounts)]
	pub(super) type DIDs<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<u8, T::MetadataSize>>;

	#[pallet::storage]
	#[pallet::getter(fn get_verifiable_credential_trail)]
	pub(super) type VerifiableCredentialTrail<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::VCFingerPrintSize>,
		BoundedVec<VerifiableCredentialLog<T::AccountId, T::BlockNumber>, T::MetadataSize>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_verifiable_credential_hash)]
	pub type VerifiableCredential<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		// VC fingerprint
		BoundedVec<u8, T::VCFingerPrintSize>,
		// VC metadata
		VerifiableCredentialMetadata<T::AccountId, T::BlockNumber, T::PublicKeySize>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event returns DID Document hash, DID URI, Sender's AccountId
		DIDDocumentCreated {
			did_uri: BoundedVec<u8, T::DIDURISize>,
			account_id: T::AccountId,
			created_block_number: T::BlockNumber,
		},

		/// DID Document Updated
		DIDDocumentUpdated {
			did_uri: BoundedVec<u8, T::DIDURISize>,
			account_id: T::AccountId,
			updated_block_number: T::BlockNumber,
		},

		/// DID Document revoked
		DIDDocumentRevoked {
			did_uri: BoundedVec<u8, T::DIDURISize>,
			account_id: T::AccountId,
			revoked_block_number: T::BlockNumber,
		},

		/// Verifiable credential fingerprint created
		VerifiableCredentialFingerPrintCreated {
			vc_finger_print: BoundedVec<u8, T::VCFingerPrintSize>,
			account_id: T::AccountId,
			created_block_number: T::BlockNumber,
		},

		/// Verifiable credential fingerprint updated
		VerifiableCredentialFingerPrintUpdated {
			vc_finger_print: BoundedVec<u8, T::VCFingerPrintSize>,
			account_id: T::AccountId,
			updated_block_number: T::BlockNumber,
		},

		/// Verifiable credential fingerprint revoked
		VerifiableCredentialFingerPrintRevoked {
			vc_finger_print: BoundedVec<u8, T::VCFingerPrintSize>,
			account_id: T::AccountId,
			revoked_block_number: T::BlockNumber,
		},
		VerifiableCredentialEvent {
			vc_finger_print: BoundedVec<u8, T::VCFingerPrintSize>,
			origin: T::AccountId,
			block_number: T::BlockNumber,
			status: VerifiableCredentialStatus,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// DID Document already exists
		DIDExists,

		/// DID Document URI is invalid
		InvalidDIDURI,

		/// DID Document signature is invalid
		InvalidDIDSignature,

		/// InvalidDID
		InvalidDID,

		/// DID Document not exists
		DIDDoesNotExist,

		/// Verifiable credential fingerprint already exists
		VerifiableCredentialFingerPrintExists,

		/// Verifiable credential fingerprint does not exists
		VerifiableCredentialFingerPrintDoesNotExist,

		/// VerifiableCredentialInactive
		VerifiableCredentialInactive,

		/// Limit Reached
		VerifiableCredentialLogLimitReached,

		/// InvalidPublicKey
		InvalidPublicKey,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::create_did_document())]
		pub fn create_did(
			origin: OriginFor<T>,
			did_uri: BoundedVec<u8, T::DIDURISize>,
			did_input: DIDMetadataPayload<T::AccountId, T::MetadataSize>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let block_number = <frame_system::Pallet<T>>::block_number();
			let did_document = DID {
				signatures: did_input.signatures,
				did_resolution_metadata: did_input.did_resolution_metadata,
				did_document_metadata: did_input.did_document_metadata,
				block_number,
				updated_block_number: block_number,
				did_ref: did_input.did_ref,
				sender_account_id: did_input.sender_account_id,
			};

			ensure!(!DIDDocument::<T>::contains_key(&did_uri), Error::<T>::DIDExists);

			DIDDocument::<T>::insert(did_uri.clone(), did_document);

			let event = Event::DIDDocumentCreated {
				did_uri,
				account_id: who,
				created_block_number: block_number,
			};
			Self::deposit_event(event);

			Ok(().into())
		}

		/// Update a DID Document
		/// #Arguments
		/// * `did_uri` - DID URI
		/// * `did_document` - DID Document
		/// #Returns
		/// * `DispatchResultWithPostInfo`
		/// #Errors
		/// * `DispatchError::Other` - DID Document does not exist
		#[pallet::weight(T::WeightInfo::update_did_document())]
		pub fn update_did_document(
			origin: OriginFor<T>,
			did_uri: BoundedVec<u8, T::DIDURISize>,
			did_input: DIDMetadataPayload<T::AccountId, T::MetadataSize>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let block_number = <frame_system::Pallet<T>>::block_number();

			DIDDocument::<T>::mutate(did_uri.clone(), |did| match did {
				| None => Err(Error::<T>::DIDDoesNotExist),
				| Some(did) => {
					let did_document = DID {
						signatures: did_input.signatures,
						did_resolution_metadata: did_input.did_resolution_metadata,
						did_document_metadata: did_input.did_document_metadata,
						block_number: did.block_number,
						updated_block_number: block_number,
						did_ref: did_input.did_ref,
						sender_account_id: did_input.sender_account_id,
					};

					*did = did_document;
					let event = Event::DIDDocumentUpdated {
						did_uri,
						account_id: who,
						updated_block_number: block_number,
					};
					Self::deposit_event(event);
					Ok(())
				},
			})?;

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::revoke_did_document())]
		pub fn revoke_did_document(
			origin: OriginFor<T>,
			did_uri: BoundedVec<u8, T::DIDURISize>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(DIDDocument::<T>::contains_key(&did_uri), Error::<T>::DIDDoesNotExist);
			let block_number = <frame_system::Pallet<T>>::block_number();
			DIDDocument::<T>::remove(&did_uri);

			let event = Event::DIDDocumentRevoked {
				did_uri,
				account_id: who,
				revoked_block_number: block_number,
			};

			Self::deposit_event(event);

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::create_verifiable_credential())]
		pub fn create_verifiable_credential(
			origin: OriginFor<T>,
			vc_finger_print: BoundedVec<u8, T::VCFingerPrintSize>,
			verifiable_credential_input_metadata: VerifiableCredentialMetadataPayload<
				T::AccountId,
				T::PublicKeySize,
			>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(
				Self::is_substrate_public_key(
					verifiable_credential_input_metadata.public_key.clone()
				),
				Error::<T>::InvalidPublicKey
			);

			ensure!(
				!VerifiableCredential::<T>::contains_key(&vc_finger_print),
				Error::<T>::VerifiableCredentialFingerPrintExists
			);

			let block_number = <frame_system::Pallet<T>>::block_number();

			let verifiable_credential_metadata = VerifiableCredentialMetadata {
				account_id: verifiable_credential_input_metadata.account_id,
				public_key: verifiable_credential_input_metadata.public_key,
				block_number,
				updated_block_number: block_number,
				active: Some(true),
			};

			VerifiableCredential::<T>::insert(
				vc_finger_print.clone(),
				verifiable_credential_metadata,
			);
			let event = Event::VerifiableCredentialFingerPrintCreated {
				vc_finger_print,
				account_id: who,
				created_block_number: block_number,
			};

			Self::deposit_event(event);

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::revoke_verifiable_credential())]
		pub fn revoke_verifiable_credential(
			origin: OriginFor<T>,
			vc_finger_print: BoundedVec<u8, T::VCFingerPrintSize>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(
				VerifiableCredential::<T>::contains_key(&vc_finger_print),
				Error::<T>::VerifiableCredentialFingerPrintDoesNotExist
			);

			let block_number = <frame_system::Pallet<T>>::block_number();

			VerifiableCredential::<T>::remove(&vc_finger_print);

			let event = Event::VerifiableCredentialFingerPrintRevoked {
				vc_finger_print,
				account_id: who,
				revoked_block_number: block_number,
			};

			Self::deposit_event(event);
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::update_verifiable_credential())]
		pub fn update_verifiable_credential(
			origin: OriginFor<T>,
			vc_finger_print: BoundedVec<u8, T::VCFingerPrintSize>,
			verifiable_credential_input_metadata: VerifiableCredentialMetadataPayload<
				T::AccountId,
				T::PublicKeySize,
			>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			VerifiableCredential::<T>::mutate(vc_finger_print.clone(), |vc| match vc {
				| None => Err(Error::<T>::VerifiableCredentialFingerPrintDoesNotExist),
				| Some(vc) => {
					let block_number = <frame_system::Pallet<T>>::block_number();
					let verifiable_credential_metadata = VerifiableCredentialMetadata {
						account_id: verifiable_credential_input_metadata.account_id,
						public_key: verifiable_credential_input_metadata.public_key,
						block_number: vc.block_number,
						updated_block_number: block_number,
						active: verifiable_credential_input_metadata.active,
					};

					*vc = verifiable_credential_metadata;
					let event = Event::VerifiableCredentialFingerPrintUpdated {
						vc_finger_print,
						account_id: who,
						updated_block_number: block_number,
					};
					Self::deposit_event(event);
					Ok(())
				},
			})?;

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::trace_credential())]
		pub fn trace_credential(
			origin: OriginFor<T>,
			account_id: Option<T::AccountId>,
			vc_finger_print: BoundedVec<u8, T::VCFingerPrintSize>,
			status: VerifiableCredentialStatus,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(
				VerifiableCredential::<T>::contains_key(&vc_finger_print),
				Error::<T>::VerifiableCredentialFingerPrintDoesNotExist
			);

			let verifiable_credential_metadata =
				VerifiableCredential::<T>::get(&vc_finger_print)
					.ok_or(Error::<T>::VerifiableCredentialFingerPrintDoesNotExist)?;
			ensure!(
				verifiable_credential_metadata.active == Some(true),
				Error::<T>::VerifiableCredentialInactive
			);

			let block_number = <frame_system::Pallet<T>>::block_number();
			let mut vc_log =
				VerifiableCredentialLog { account_id, block_number: Some(block_number), status };

			match VerifiableCredentialTrail::<T>::get(&vc_finger_print) {
				None => {
					let mut arr: BoundedVec<
						VerifiableCredentialLog<T::AccountId, T::BlockNumber>,
						T::MetadataSize,
					> = Default::default();
					vc_log.status = VerifiableCredentialStatus::Created;
					arr.try_push(vc_log.clone())
						.map_err(|_| Error::<T>::VerifiableCredentialLogLimitReached)?;
					VerifiableCredentialTrail::<T>::insert(&vc_finger_print, arr);
					let event = Event::VerifiableCredentialEvent {
						vc_finger_print: vc_finger_print.clone(),
						origin: who,
						block_number,
						status: VerifiableCredentialStatus::Created,
					};
					Self::deposit_event(event);
				},
				Some(_) => {
					VerifiableCredentialTrail::<T>::try_mutate(
						vc_finger_print.clone(),
						|x| -> DispatchResult {
							let x = x
								.as_mut()
								.ok_or(Error::<T>::VerifiableCredentialFingerPrintDoesNotExist)?;
							x.try_push(vc_log.clone())
								.map_err(|_| Error::<T>::VerifiableCredentialLogLimitReached)?;
							let event = Event::VerifiableCredentialEvent {
								vc_finger_print,
								origin: who,
								block_number,
								status: vc_log.status,
							};
							Self::deposit_event(event);
							Ok(())
						},
					)?;
				},
			}
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Increment the cluster id generator and return the id
		fn is_substrate_public_key(public_key: BoundedVec<u8, T::PublicKeySize>) -> bool {
			T::AccountId::decode(&mut &public_key.clone().to_vec()[..]).is_ok()
		}
	}
}
