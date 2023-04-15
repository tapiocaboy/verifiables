use super::*;
use crate::Pallet as Verifiable;
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite};
use frame_support::{ BoundedVec};
use frame_system::RawOrigin;

use sp_runtime::traits::Bounded;
use sp_std::prelude::*;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}
pub fn origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
	RawOrigin::Signed(get_account::<T>(name))
}

pub fn prepare_benchmarks<T: Config>() {
	let alice: T::AccountId = get_account::<T>("ALICE");
	let bob: T::AccountId = get_account::<T>("BOB");

	// T::Currency::make_free_balance_be(&alice, BalanceOf::<T>::max_value());
	// T::Currency::make_free_balance_be(&bob, BalanceOf::<T>::max_value());
}

benchmarks! {

	create_did {
		prepare_benchmarks::<T>();
		let alice: T::AccountId = get_account::<T>("ALICE");
		let did_uri: BoundedVec<u8, T::DIDURISize> = "did:neos:0x1234567890123456789012345678901234567890"
		.as_bytes()
		.to_vec()
		.try_into()
		.unwrap();

		let did_resolution_metadata =r#"{
			"accept": "application/did+ld+json"
		}"#
		.as_bytes()
		.to_vec()
		.try_into()
		.unwrap();

		let did_document_metadata = r#"{
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
			signatures: sig,
			did_resolution_metadata: Some(did_resolution_metadata),
			did_document_metadata: Some(did_document_metadata),
			did_ref: None,
			sender_account_id: get_account::<T>("BOB"),
		};

	}: _(origin::<T>("ALICE"),did_uri.clone(), did_input)
	verify {
		assert!(DIDDocument::<T>::get(&did_uri).is_some());
	}
}

impl_benchmark_test_suite!(
	Verifiable,
	crate::mock::new_test_ext(),
	crate::mock::Test
);
