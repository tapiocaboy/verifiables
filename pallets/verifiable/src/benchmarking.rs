use super::*;
use crate::Pallet as Verifiable;
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite};
use frame_support::BoundedVec;
use frame_system::RawOrigin;

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
const VC_FINGERPRINT: &str = "vc_fingerprint";

pub fn prepare_benchmark_data_did<T: Config>() {
	let did_uri: BoundedVec<u8, T::DIDURISize> = DID_URI.as_bytes().to_vec().try_into().unwrap();

	let did_resolution_metadata: BoundedVec<u8, T::MetadataSize> = r#"{
																		"accept": "application/did+ld+json"
																	}"#
	.as_bytes()
	.to_vec()
	.try_into()
	.unwrap();

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
	Verifiable::<T>::create_did(origin::<T>("ALICE").into(), did_uri.clone(), did_input.clone())
		.unwrap();
}

pub fn prepare_benchmark_verifiable_credential<T: Config>() {
	let vc_fingerprint: BoundedVec<u8, T::VCFingerPrintSize> =
		"vc_fingerprint".as_bytes().to_vec().try_into().unwrap();
	let public_key: BoundedVec<u8, T::PublicKeySize> = vec![
		0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45, 94,
		199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
	]
	.try_into()
	.unwrap();
	let vc_metadata_inout = VerifiableCredentialMetadataPayload {
		account_id: Some(get_account::<T>("BOB")),
		public_key: public_key.clone(),
		active: Some(true),
	};
	Verifiable::<T>::create_verifiable_credential(
		origin::<T>("ALICE").into(),
		vc_fingerprint.clone(),
		vc_metadata_inout.clone(),
	)
	.unwrap();
}

benchmarks! {

	create_did {
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
		assert_last_event::<T>(Event::<T>::DIDDocumentCreated {
			did_uri,
			account_id: get_account::<T>("ALICE"),
			created_block_number: 1u32.into(),
		}.into());
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

		let did_input: DIDMetadataPayload<T::AccountId, T::MetadataSize> = DIDMetadataPayload {
			signatures: sig.clone(),
			did_resolution_metadata: Some(did_resolution_metadata.clone()),
			did_document_metadata: Some(did_document_metadata.clone()),
			did_ref: None,
			sender_account_id: get_account::<T>("BOB"),
		};

	}: _(origin::<T>("ALICE"), did_uri.clone(), did_input)
	verify {
		assert!(DIDDocument::<T>::get(&did_uri).is_some());

		assert!(DIDDocument::<T>::get(&did_uri).unwrap().did_ref.is_none());
		assert_eq!(DIDDocument::<T>::get(&did_uri).unwrap().did_resolution_metadata ,Some(did_resolution_metadata));
		assert_eq!(DIDDocument::<T>::get(&did_uri).unwrap().did_document_metadata, Some(did_document_metadata));
		assert_eq!(DIDDocument::<T>::get(&did_uri).unwrap().sender_account_id , get_account::<T>("BOB"));
		assert_last_event::<T>(Event::<T>::DIDDocumentUpdated {
			did_uri,
			account_id: get_account::<T>("ALICE"),
			updated_block_number: 1u32.into(),
		}.into());
	}

	revoke_did_document {
		prepare_benchmark_data_did::<T>();
		let alice: T::AccountId = get_account::<T>("ALICE");
		let did_uri: BoundedVec<u8, T::DIDURISize> = DID_URI.as_bytes().to_vec().try_into().unwrap();

	}: _(origin::<T>("ALICE"),did_uri.clone())
	verify {
		assert!(DIDDocument::<T>::get(&did_uri).is_none());
		assert_last_event::<T>(Event::<T>::DIDDocumentRevoked {
			did_uri,
			account_id: get_account::<T>("ALICE"),
			revoked_block_number: 1u32.into(),
		}.into());
	}

	create_verifiable_credential{
		let vc_fingerprint: BoundedVec<u8, T::VCFingerPrintSize> = VC_FINGERPRINT.as_bytes().to_vec().try_into().unwrap();
		let public_key: BoundedVec<u8, T::PublicKeySize> = vec![
			0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45,
			94, 199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
		]
			.try_into()
			.unwrap();
		let vc_metadata_input : VerifiableCredentialMetadataPayload<T::AccountId, T::PublicKeySize>= VerifiableCredentialMetadataPayload {
			account_id: Some(get_account::<T>("BOB")),
			public_key: public_key.clone(),
			active: Some(true),
		};

	}: _(origin::<T>("ALICE"), vc_fingerprint.clone(),vc_metadata_input.clone())
	verify {
		assert!(VerifiableCredential::<T>::get(&vc_fingerprint).is_some());
		assert!(VerifiableCredential::<T>::get(&vc_fingerprint).unwrap() == VerifiableCredentialMetadata {
			account_id: Some(get_account::<T>("BOB")),
			public_key: public_key,
			active: Some(true),
			block_number: 1u32.into(),
			updated_block_number: 1u32.into(),
		});

		assert_last_event::<T>(Event::<T>::VerifiableCredentialFingerPrintCreated {
			vc_fingerprint,
			account_id: get_account::<T>("ALICE"),
			created_block_number: 1u32.into(),
		}.into());
	}

	update_verifiable_credential{
		prepare_benchmark_verifiable_credential::<T>();
		let vc_fingerprint: BoundedVec<u8, T::VCFingerPrintSize> = VC_FINGERPRINT.as_bytes().to_vec().try_into().unwrap();
		let public_key: BoundedVec<u8, T::PublicKeySize> = vec![
			10, 10, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45,
			94, 199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
		]
		.try_into()
		.unwrap();
		let vc_metadata_input : VerifiableCredentialMetadataPayload<T::AccountId, T::PublicKeySize>= VerifiableCredentialMetadataPayload {
			account_id: Some(get_account::<T>("BOB")),
			public_key: public_key.clone(),
			active: Some(true),
		};

	}: _(origin::<T>("ALICE"), vc_fingerprint.clone(),vc_metadata_input.clone())
	verify {
		assert!(VerifiableCredential::<T>::get(&vc_fingerprint).is_some());
		assert_eq!(VerifiableCredential::<T>::get(&vc_fingerprint).unwrap().account_id, Some(get_account::<T>("BOB")));
		assert_eq!(VerifiableCredential::<T>::get(&vc_fingerprint).unwrap().public_key, public_key);
		assert_eq!(VerifiableCredential::<T>::get(&vc_fingerprint).unwrap().active, Some(true));
		assert_last_event::<T>(Event::<T>::VerifiableCredentialFingerPrintUpdated {
			vc_fingerprint,
			account_id:  get_account::<T>("ALICE"),
			updated_block_number: 1u32.into(),
		}.into());

	}
	revoke_verifiable_credential{
		prepare_benchmark_verifiable_credential::<T>();
		let vc_fingerprint: BoundedVec<u8, T::VCFingerPrintSize> = VC_FINGERPRINT.as_bytes().to_vec().try_into().unwrap();


	}: _(origin::<T>("ALICE"),vc_fingerprint.clone())
	verify {
		assert!(VerifiableCredential::<T>::get(&vc_fingerprint).is_none());
		assert_last_event::<T>(Event::<T>::VerifiableCredentialFingerPrintRevoked {
				vc_fingerprint,
				account_id: get_account::<T>("ALICE"),
				revoked_block_number: 1u32.into(),
			}.into());
	}

	trace_credential {
		prepare_benchmark_verifiable_credential::<T>();
		let vc_fingerprint: BoundedVec<u8, T::VCFingerPrintSize> = VC_FINGERPRINT.as_bytes().to_vec().try_into().unwrap();
	}: _(origin::<T>("ALICE"),
		Some(get_account::<T>("BOB")),
		vc_fingerprint.clone(),
		VerifiableCredentialStatus::Created
	)
	verify {
		assert!(VerifiableCredentialTrail::<T>::get(&vc_fingerprint).is_some());
		let results = VerifiableCredentialTrail::<T>::get(&vc_fingerprint).unwrap();
		assert_eq!(results[0].account_id, Some(get_account::<T>("BOB")));
		assert_eq!(results[0].status, VerifiableCredentialStatus::Created);
		assert_last_event::<T>(Event::<T>::VerifiableCredentialEvent {
			vc_fingerprint,
			origin: get_account::<T>("ALICE"),
			block_number: 1u32.into(),
			status: VerifiableCredentialStatus::Created,
		}.into());
	}
}

impl_benchmark_test_suite!(Verifiable, crate::mock::new_test_ext(), crate::mock::Test);
