use frame_support::weights::Weight;

pub trait WeightInfo {
	fn create_did_document() -> Weight;
	fn update_did_document() -> Weight;
	fn revoke_did_document() -> Weight;
	fn trace_credential() -> Weight;
	fn revoke_verifiable_credential() -> Weight;
	fn create_verifiable_credential() -> Weight;
	fn update_verifiable_credential() -> Weight;
	fn delete_verifiable_credential() -> Weight;
	fn verify_verifiable_credential() -> Weight;
}

impl WeightInfo for () {
	fn create_did_document() -> Weight {
		Weight::from_ref_time(10_000_000_u64)
	}

	fn update_did_document() -> Weight {
		Weight::from_ref_time(10_000_000_u64)
	}

	fn create_verifiable_credential() -> Weight {
		Weight::from_ref_time(10_000_000_u64)
	}

	fn revoke_verifiable_credential() -> Weight {
		Weight::from_ref_time(10_000_000_u64)
	}

	fn revoke_did_document() -> Weight {
		Weight::from_ref_time(10_000_000_u64)
	}

	fn trace_credential() -> Weight {
		Weight::from_ref_time(10_000_000_u64)
	}

	fn update_verifiable_credential() -> Weight {
		Weight::from_ref_time(10_000_000_u64)
	}

	fn delete_verifiable_credential() -> Weight {
		Weight::from_ref_time(10_000_000_u64)
	}

	fn verify_verifiable_credential() -> Weight {
		Weight::from_ref_time(10_000_000_u64)
	}
}
