use super::*;
use crate::Pallet as Verifiable;
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite};
use frame_support::BoundedVec;
use frame_system::RawOrigin;

use sp_runtime::traits::Bounded;
use sp_std::prelude::*;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}
pub fn origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
	RawOrigin::Signed(get_account::<T>(name))
}

const DID_URI: &str = "did:aloo:0x1234567890123456789012345678901234567890";
pub fn prepare_benchmark_data_did<T: Config>() {

	let did_uri: BoundedVec<u8, T::DIDURISize> = DID_URI
		.as_bytes()
		.to_vec()
		.try_into()
		.unwrap();

	let did_resolution_metadata: BoundedVec<u8, T::MetadataSize> =r#"{
																		"accept": "application/did+ld+json"
																	}"#
		.as_bytes().to_vec().try_into().unwrap();

	let did_document_metadata: BoundedVec<u8, T::MetadataSize> = r#"{
							"created": "2002-01-01T20:20:20Z",
							"updated": "2002-02-01T20:20:20Z",
							"deactivated": "2002-03-01T20:20:20Z",
							"versionId": "1",
						}"#
		.as_bytes()
		.to_vec()
		.try_into()
		.unwrap();

	let sig: BoundedVec<u8, T::MetadataSize> = BoundedVec::default();

	let did_input: DIDMetadataPayload<T::AccountId, T::MetadataSize> = DIDMetadataPayload {
		signatures: sig.clone(),
		did_resolution_metadata: Some(did_resolution_metadata.clone()),
		did_document_metadata: Some(did_document_metadata.clone()),
		did_ref: None,
		sender_account_id: get_account::<T>("BOB"),
	};
	Verifiable::<T>::create_did(origin::<T>("ALICE").into(),did_uri.clone(), did_input.clone()).unwrap();
}

benchmarks! {

	create_did {
		let alice: T::AccountId = get_account::<T>("ALICE");
		// let bob: T::AccountId = get_account::<T>("BOB");

		let did_uri: BoundedVec<u8, T::DIDURISize> = DID_URI
		.as_bytes()
		.to_vec()
		.try_into()
		.unwrap();

		let did_resolution_metadata: BoundedVec<u8, T::MetadataSize> =r#"{
																		"accept": "application/did+ld+json"
																	}"#
																	.as_bytes().to_vec().try_into().unwrap();

		let did_document_metadata: BoundedVec<u8, T::MetadataSize> = r#"{
							"created": "2002-01-01T20:20:20Z",
							"updated": "2002-02-01T20:20:20Z",
							"deactivated": "2002-03-01T20:20:20Z",
							"versionId": "1",
						}"#
				.as_bytes()
				.to_vec()
				.try_into()
				.unwrap();

		let sig: BoundedVec<u8, T::MetadataSize> = BoundedVec::default();

		let did_input: DIDMetadataPayload<T::AccountId, T::MetadataSize> = DIDMetadataPayload {
			signatures: sig.clone(),
			did_resolution_metadata: Some(did_resolution_metadata.clone()),
			did_document_metadata: Some(did_document_metadata.clone()),
			did_ref: None,
			sender_account_id: get_account::<T>("BOB"),
		};

	}: _(origin::<T>("ALICE"),did_uri.clone(), did_input)
	verify {
		assert!(DIDDocument::<T>::get(&did_uri).is_some());
		assert!(DIDDocument::<T>::get(&did_uri).unwrap() == DID {
			signatures: sig,
			did_resolution_metadata: Some(did_resolution_metadata),
			did_document_metadata: Some(did_document_metadata),
			block_number: 1u32.into(),
			updated_block_number: 1u32.into(),
			did_ref: None,
			sender_account_id: get_account::<T>("BOB"),
		});
	}

	update_did_document {
		prepare_benchmark_data_did::<T>();
		let alice: T::AccountId = get_account::<T>("ALICE");


		let did_uri: BoundedVec<u8, T::DIDURISize> = DID_URI.as_bytes().to_vec().try_into().unwrap();

		let did_resolution_metadata: BoundedVec<u8, T::MetadataSize> =r#"{
																		"accept": "application/did+ld+json"
																	}"#
																	.as_bytes().to_vec().try_into().unwrap();

		let did_document_metadata: BoundedVec<u8, T::MetadataSize> = r#"{
							"created": "2022-01-01T20:20:20Z",
							"updated": "2032-02-01T20:20:20Z",
							"deactivated": "20145-03-01T20:20:20Z",
							"versionId": "1",
						}"#
				.as_bytes()
				.to_vec()
				.try_into()
				.unwrap();

		let sig: BoundedVec<u8, T::MetadataSize> = BoundedVec::default();

		let  did_input: DIDMetadataPayload<T::AccountId, T::MetadataSize> = DIDMetadataPayload {
			signatures: sig.clone(),
			did_resolution_metadata: Some(did_resolution_metadata.clone()),
			did_document_metadata: Some(did_document_metadata.clone()),
			did_ref: None,
			sender_account_id: get_account::<T>("BOB"),
		};

	}: _(origin::<T>("ALICE"),did_uri.clone(), did_input)
	verify {
		assert!(DIDDocument::<T>::get(&did_uri).is_some());

		assert!(DIDDocument::<T>::get(&did_uri).unwrap().did_ref.is_none());
		assert_eq!(DIDDocument::<T>::get(&did_uri).unwrap().did_resolution_metadata ,Some(did_resolution_metadata));
		assert_eq!(DIDDocument::<T>::get(&did_uri).unwrap().did_document_metadata, Some(did_document_metadata));
		assert_eq!(DIDDocument::<T>::get(&did_uri).unwrap().sender_account_id , get_account::<T>("BOB"));
		// assert_eq!(DIDDocument::<T>::get(&did_uri).unwrap().block_number.);

	}

	revoke_did_document {
		prepare_benchmark_data_did::<T>();
		let alice: T::AccountId = get_account::<T>("ALICE");
		let did_uri: BoundedVec<u8, T::DIDURISize> = DID_URI.as_bytes().to_vec().try_into().unwrap();

	}: _(origin::<T>("ALICE"),did_uri.clone())
	verify {
		assert!(DIDDocument::<T>::get(&did_uri).is_none());
	}
}

impl_benchmark_test_suite!(Verifiable, crate::mock::new_test_ext(), crate::mock::Test);
