use super::{mock, mock::*};
use frame_support::{assert_ok, BoundedVec};
use frame_system::RawOrigin;
fn origin(account: u64) -> mock::RuntimeOrigin {
	RawOrigin::Signed(account).into()
}

fn get_did_document_metadata() -> BoundedVec<u8, MetadataSize> {
	let did_document_metadata: BoundedVec<u8, MetadataSize> = r#"{
													"created": "2002-01-01T20:20:20Z",
													"updated": "2002-02-01T20:20:20Z",
													"deactivated": "2002-03-01T20:20:20Z",
													"versionId": "1",
												}"#
	.as_bytes()
	.to_vec()
	.try_into()
	.unwrap();
	did_document_metadata
}

fn get_public_key(pk: Vec<u8>) -> BoundedVec<u8, PublicKeySize> {
	let public_key: BoundedVec<u8, PublicKeySize> = pk.try_into().unwrap();
	public_key
}

fn get_did_resolution_metadata() -> BoundedVec<u8, MetadataSize> {
	let did_resolution_metadata: BoundedVec<u8, MetadataSize> = r#"{
														"accept": "application/did+ld+json"
													}"#
	.as_bytes()
	.to_vec()
	.try_into()
	.unwrap();
	did_resolution_metadata
}

fn get_did_uri() -> BoundedVec<u8, DIDURISize> {
	let did_uri: BoundedVec<u8, DIDURISize> = "did:alto:0x1234567890123456789012345678901234567890"
		.as_bytes()
		.to_vec()
		.try_into()
		.unwrap();
	did_uri
}
mod create_did {
	use super::*;
	use crate::{DIDDocument, DIDMetadataPayload, Error};
	use frame_support::assert_noop;

	#[test]
	fn create_did() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let did_uri = get_did_uri();

			let did_resolution_metadata = get_did_resolution_metadata();

			let did_document_metadata = get_did_document_metadata();

			let sig: BoundedVec<u8, MetadataSize> = BoundedVec::default();

			let did = DIDMetadataPayload {
				signatures: sig,
				did_resolution_metadata: Some(did_resolution_metadata),
				did_document_metadata: Some(did_document_metadata),
				did_ref: None,
				sender_account_id: 1,
			};

			assert_ok!(Verifiable::create_did(alice, did_uri.clone(), did));
			assert!(DIDDocument::<Test>::get(&did_uri).is_some());
		});
	}

	#[test]
	fn did_document_exists() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let did_uri = get_did_uri();

			let did_resolution_metadata = get_did_resolution_metadata();

			let did_document_metadata = get_did_document_metadata();

			let sig: BoundedVec<u8, MetadataSize> = BoundedVec::default();

			let did = DIDMetadataPayload {
				signatures: sig,
				did_resolution_metadata: Some(did_resolution_metadata),
				did_document_metadata: Some(did_document_metadata),
				did_ref: None,
				sender_account_id: 1,
			};

			assert_ok!(Verifiable::create_did(alice.clone(), did_uri.clone(), did.clone()));
			assert_noop!(Verifiable::create_did(alice, did_uri, did), Error::<Test>::DIDExists);
		});
	}
}

mod revoke_did {
	use super::*;
	use crate::{DIDDocument, DIDMetadataPayload, Error};
	use frame_support::assert_noop;

	#[test]
	fn revoke() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let did_uri = get_did_uri();

			let did_resolution_metadata = get_did_resolution_metadata();

			let did_document_metadata = get_did_document_metadata();

			let sig: BoundedVec<u8, MetadataSize> = BoundedVec::default();

			let did = DIDMetadataPayload {
				signatures: sig,
				did_resolution_metadata: Some(did_resolution_metadata),
				did_document_metadata: Some(did_document_metadata),
				did_ref: None,
				sender_account_id: 1,
			};

			assert_ok!(Verifiable::create_did(alice.clone(), did_uri.clone(), did));
			assert_ok!(Verifiable::revoke_did_document(alice, did_uri.clone()));
			assert!(DIDDocument::<Test>::get(&did_uri).is_none());
		});
	}

	#[test]
	fn revoke_non_existing_did_document() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let did_uri = get_did_uri();
			assert_noop!(
				Verifiable::revoke_did_document(alice, did_uri),
				Error::<Test>::DIDDoesNotExist
			);
		});
	}
}

mod update_did {
	use super::*;
	use crate::{DIDDocument, DIDMetadataPayload, Error};
	use frame_support::assert_noop;

	#[test]
	fn update_did() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let did_uri = get_did_uri();

			let did_resolution_metadata = get_did_resolution_metadata();

			let did_document_metadata = get_did_document_metadata();

			let sig: BoundedVec<u8, MetadataSize> = BoundedVec::default();

			let did = DIDMetadataPayload {
				signatures: sig.clone(),
				did_resolution_metadata: Some(did_resolution_metadata.clone()),
				did_document_metadata: Some(did_document_metadata.clone()),
				did_ref: None,
				sender_account_id: 1,
			};

			assert_ok!(Verifiable::create_did(alice.clone(), did_uri.clone(), did));
			let did = DIDMetadataPayload {
				signatures: sig,
				did_resolution_metadata: Some(did_resolution_metadata),
				did_document_metadata: Some(did_document_metadata),
				did_ref: None,
				sender_account_id: 2,
			};
			assert_ok!(Verifiable::update_did_document(alice, did_uri.clone(), did));
			assert!(DIDDocument::<Test>::get(&did_uri).is_some());

			let result = DIDDocument::<Test>::get(&did_uri).unwrap();
			assert_eq!(result.sender_account_id, 2);
		});
	}

	#[test]
	fn update_non_existing_did() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let did_uri = get_did_uri();

			let did_resolution_metadata = get_did_resolution_metadata();

			let did_document_metadata = get_did_document_metadata();

			let sig: BoundedVec<u8, MetadataSize> = BoundedVec::default();

			let did = DIDMetadataPayload {
				signatures: sig,
				did_resolution_metadata: Some(did_resolution_metadata),
				did_document_metadata: Some(did_document_metadata),
				did_ref: None,
				sender_account_id: 2,
			};
			assert_noop!(
				Verifiable::update_did_document(alice, did_uri, did),
				Error::<Test>::DIDDoesNotExist
			);
		});
	}
}

mod create_verifiable_credential {
	use super::*;
	use crate::{Error, VerifiableCredential, VerifiableCredentialMetadataPayload};
	use frame_support::assert_noop;

	#[test]
	fn create_verifiable_credential() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let vc_fingerprint: BoundedVec<u8, VCFingerPrintSize> =
				"vc_fingerprint".as_bytes().to_vec().try_into().unwrap();
			let public_key: BoundedVec<u8, PublicKeySize> = vec![
				0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45,
				94, 199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
			]
			.try_into()
			.unwrap();
			let vc_metadata_inout = VerifiableCredentialMetadataPayload {
				account_id: Some(1),
				public_key: public_key.clone(),
				active: Some(true),
			};
			assert_ok!(Verifiable::create_verifiable_credential(
				alice,
				vc_fingerprint.clone(),
				vc_metadata_inout
			));
			assert!(VerifiableCredential::<Test>::get(&vc_fingerprint).is_some());
			let result = VerifiableCredential::<Test>::get(&vc_fingerprint).unwrap();
			assert_eq!(result.account_id.unwrap(), 1);
			assert_eq!(result.active.unwrap(), true);
			assert_eq!(result.public_key, public_key);
		});
	}

	#[test]
	fn create_existing_verifiable_credential() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let vc_fingerprint: BoundedVec<u8, VCFingerPrintSize> =
				"vc_fingerprint".as_bytes().to_vec().try_into().unwrap();
			let public_key: BoundedVec<u8, PublicKeySize> = vec![
				0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45,
				94, 199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
			]
			.try_into()
			.unwrap();
			let vc_metadata_input = VerifiableCredentialMetadataPayload {
				account_id: Some(1),
				public_key,
				active: Some(true),
			};
			assert_ok!(Verifiable::create_verifiable_credential(
				alice.clone(),
				vc_fingerprint.clone(),
				vc_metadata_input.clone()
			));
			assert_noop!(
				Verifiable::create_verifiable_credential(alice, vc_fingerprint, vc_metadata_input),
				Error::<Test>::VerifiableCredentialFingerPrintExists
			);
		});
	}

	#[test]
	fn invalid_public_key() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let vc_fingerprint: BoundedVec<u8, VCFingerPrintSize> =
				"vc_fingerprint".as_bytes().to_vec().try_into().unwrap();

			let public_key: BoundedVec<u8, PublicKeySize> = vec![255, 32].try_into().unwrap();
			let vc_metadata_input = VerifiableCredentialMetadataPayload {
				account_id: Some(1),
				public_key,
				active: Some(true),
			};

			assert_noop!(
				Verifiable::create_verifiable_credential(alice, vc_fingerprint, vc_metadata_input),
				Error::<Test>::InvalidPublicKey
			);
		});
	}
}

mod update_verifiable_credential {
	use super::*;
	use crate::{Error, VerifiableCredential, VerifiableCredentialMetadataPayload};
	use frame_support::assert_noop;

	#[test]
	fn update_verifiable_credential() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let vc_fingerprint: BoundedVec<u8, VCFingerPrintSize> =
				"vc_fingerprint".as_bytes().to_vec().try_into().unwrap();
			let public_key = get_public_key(vec![
				0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45,
				94, 199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
			]);
			let vc_metadata_input = VerifiableCredentialMetadataPayload {
				account_id: Some(1),
				public_key,
				active: Some(true),
			};
			assert_ok!(Verifiable::create_verifiable_credential(
				alice.clone(),
				vc_fingerprint.clone(),
				vc_metadata_input
			));

			let public_key = get_public_key(vec![
				1, 1, 1, 1, 1, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45, 94,
				199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
			]);
			let vc_metadata_input = VerifiableCredentialMetadataPayload {
				account_id: Some(2),
				public_key: public_key.clone(),
				active: Some(false),
			};
			assert_ok!(Verifiable::update_verifiable_credential(
				alice,
				vc_fingerprint.clone(),
				vc_metadata_input
			));

			assert!(VerifiableCredential::<Test>::get(&vc_fingerprint).is_some());
			let result = VerifiableCredential::<Test>::get(&vc_fingerprint).unwrap();
			assert_eq!(result.account_id.unwrap(), 2);
			assert_eq!(result.active.unwrap(), false);
			assert_eq!(result.public_key, public_key);
		});
	}

	#[test]
	fn update_non_existing_verifiable_credential() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let vc_fingerprint: BoundedVec<u8, VCFingerPrintSize> =
				"vc_fingerprint".as_bytes().to_vec().try_into().unwrap();
			let public_key = get_public_key(vec![
				0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45,
				94, 199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
			]);
			let vc_metadata_input = VerifiableCredentialMetadataPayload {
				account_id: Some(1),
				public_key,
				active: Some(true),
			};

			assert_noop!(
				Verifiable::update_verifiable_credential(alice, vc_fingerprint, vc_metadata_input),
				Error::<Test>::VerifiableCredentialFingerPrintDoesNotExist
			);
		});
	}
}

mod revoke_verifiable_credential {
	use super::*;
	use crate::{Error, VerifiableCredential, VerifiableCredentialMetadataPayload};
	use frame_support::assert_noop;

	#[test]
	fn revoke_verifiable_credential() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let vc_fingerprint: BoundedVec<u8, VCFingerPrintSize> =
				"vc_fingerprint".as_bytes().to_vec().try_into().unwrap();
			let public_key = get_public_key(vec![
				0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45,
				94, 199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
			]);
			let vc_metadata_input = VerifiableCredentialMetadataPayload {
				account_id: Some(1),
				public_key,
				active: Some(true),
			};
			assert_ok!(Verifiable::create_verifiable_credential(
				alice.clone(),
				vc_fingerprint.clone(),
				vc_metadata_input
			));
			assert!(VerifiableCredential::<Test>::get(&vc_fingerprint).is_some());

			assert_ok!(Verifiable::revoke_verifiable_credential(alice, vc_fingerprint.clone()));

			assert!(VerifiableCredential::<Test>::get(&vc_fingerprint).is_none());
		});
	}

	#[test]
	fn revoke_non_exising_verifiable_credential() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let vc_fingerprint: BoundedVec<u8, VCFingerPrintSize> =
				"vc_fingerprint".as_bytes().to_vec().try_into().unwrap();

			assert_noop!(
				Verifiable::revoke_verifiable_credential(alice, vc_fingerprint),
				Error::<Test>::VerifiableCredentialFingerPrintDoesNotExist
			);
		});
	}
}

mod trace_credential {

	use super::*;
	use crate::{
		pallet::VerifiableCredentialTrail, VerifiableCredential, VerifiableCredentialLog,
		VerifiableCredentialMetadataPayload, VerifiableCredentialStatus,
	};

	#[test]
	fn trace_credential() {
		new_test_ext().execute_with(|| {
			let alice: mock::RuntimeOrigin = origin(ALICE);
			let vc_fingerprint: BoundedVec<u8, VCFingerPrintSize> =
				"vc_fingerprint".as_bytes().to_vec().try_into().unwrap();
			let public_key = get_public_key(vec![
				0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45,
				94, 199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
			]);
			let vc_metadata_input = VerifiableCredentialMetadataPayload {
				account_id: Some(1),
				public_key,
				active: Some(true),
			};
			assert_ok!(Verifiable::create_verifiable_credential(
				alice.clone(),
				vc_fingerprint.clone(),
				vc_metadata_input
			));
			assert!(VerifiableCredential::<Test>::get(&vc_fingerprint).is_some());

			assert_ok!(Verifiable::trace_credential(
				alice.clone(),
				Some(BOB),
				vc_fingerprint.clone(),
				VerifiableCredentialStatus::Created
			));

			let result = VerifiableCredentialTrail::<Test>::get(&vc_fingerprint).unwrap();
			assert_eq!(result.len(), 1);
			assert_eq!(result[0].account_id, Some(BOB));
			assert_eq!(result[0].status, VerifiableCredentialStatus::Created);

			assert_ok!(Verifiable::trace_credential(
				alice,
				Some(CHARLIE),
				vc_fingerprint.clone(),
				VerifiableCredentialStatus::Scanned
			));
			let result = VerifiableCredentialTrail::<Test>::get(&vc_fingerprint).unwrap();
			assert_eq!(result.len(), 2);
			assert_eq!(result[1].account_id, Some(CHARLIE));
			assert_eq!(result[1].status, VerifiableCredentialStatus::Scanned);

			assert_eq!(
				result,
				vec![
					VerifiableCredentialLog {
						account_id: Some(BOB),
						status: VerifiableCredentialStatus::Created,
						block_number: Some(0),
					},
					VerifiableCredentialLog {
						account_id: Some(CHARLIE),
						status: VerifiableCredentialStatus::Scanned,
						block_number: Some(0),
					},
				]
			);
		});
	}
}
