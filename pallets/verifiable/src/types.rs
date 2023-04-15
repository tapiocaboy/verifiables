#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{traits::Get, BoundedVec, CloneNoBound, PartialEqNoBound, RuntimeDebugNoBound};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

use frame_support::codec::{Decode, Encode, MaxEncodedLen};

use sp_core::ed25519::Signature;
use sp_runtime::RuntimeDebug;

/// Stores Signatures by DID Controllers
/// A DID can have at least a controller
#[derive(
	Encode, Decode, CloneNoBound, PartialEqNoBound, Eq, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,
)]
#[scale_info(skip_type_params(MetadataSize))]
pub struct DIDSignature<MetadataSize>
where
	MetadataSize: Get<u32>,
{
	pub public_key: BoundedVec<u8, MetadataSize>,
	pub proof: Signature,
	pub active: bool,
}

#[derive(
	Encode,
	Decode,
	CloneNoBound,
	PartialEqNoBound,
	Eq,
	RuntimeDebugNoBound,
	TypeInfo,
	MaxEncodedLen,
	Default,
)]
#[codec(mel_bound(AccountId: MaxEncodedLen, BlockNumber: MaxEncodedLen))]
#[scale_info(skip_type_params(MetadataSize))]
pub struct DID<AccountId, BlockNumber, MetadataSize>
where
	AccountId: Clone + PartialEq + Debug,
	BlockNumber: Clone + PartialEq + Debug + PartialOrd,
	MetadataSize: Get<u32>,
{
	pub signatures: BoundedVec<u8, MetadataSize>,
	pub did_resolution_metadata: Option<BoundedVec<u8, MetadataSize>>,

	// DID Document Metadata
	pub did_document_metadata: Option<BoundedVec<u8, MetadataSize>>,

	pub block_number: BlockNumber,

	// Updated timestamp
	pub updated_block_number: BlockNumber,
	// IPFS  URI of the DID document
	pub did_ref: Option<BoundedVec<u8, MetadataSize>>,

	// Sender AccountId
	pub sender_account_id: AccountId,
}

#[derive(
	Encode,
	Decode,
	CloneNoBound,
	PartialEqNoBound,
	Eq,
	RuntimeDebugNoBound,
	TypeInfo,
	MaxEncodedLen,
	Default,
)]
#[scale_info(skip_type_params(MetadataSize))]
#[codec(mel_bound(AccountId: MaxEncodedLen))]
pub struct DIDMetadataPayload<AccountId, MetadataSize>
where
	AccountId: Clone + PartialEq + Debug,
	MetadataSize: Get<u32>,
{
	pub signatures: BoundedVec<u8, MetadataSize>,
	pub did_resolution_metadata: Option<BoundedVec<u8, MetadataSize>>,

	// DID Document Metadata
	pub did_document_metadata: Option<BoundedVec<u8, MetadataSize>>,

	// IPFS  URI of the DID document
	pub did_ref: Option<BoundedVec<u8, MetadataSize>>,

	// Sender AccountId
	pub sender_account_id: AccountId,
}

#[derive(
	Encode,
	Decode,
	CloneNoBound,
	PartialEqNoBound,
	Eq,
	RuntimeDebugNoBound,
	TypeInfo,
	MaxEncodedLen,
	Default,
)]
#[scale_info(skip_type_params(PublicKeySize))]
#[codec(mel_bound(AccountId: MaxEncodedLen))]
pub struct VerifiableCredentialMetadataPayload<AccountId, PublicKeySize>
where
	AccountId: Clone + PartialEq + Debug,
	PublicKeySize: Get<u32>,
{
	// Controller's AccountId
	pub account_id: Option<AccountId>,

	// Holder's public key
	pub public_key: BoundedVec<u8, PublicKeySize>,

	// active
	pub active: Option<bool>,
}

#[derive(
	Encode,
	Decode,
	CloneNoBound,
	PartialEqNoBound,
	Eq,
	RuntimeDebugNoBound,
	TypeInfo,
	MaxEncodedLen,
	Default,
)]
#[scale_info(skip_type_params(PublicKeySize))]
#[codec(mel_bound(AccountId: MaxEncodedLen, BlockNumber: MaxEncodedLen))]
pub struct VerifiableCredentialMetadata<AccountId, BlockNumber, PublicKeySize>
where
	AccountId: Clone + PartialEq + Debug,
	BlockNumber: Clone + PartialEq + Debug + PartialOrd,
	PublicKeySize: Get<u32>,
{
	// Controller's AccountId
	pub account_id: Option<AccountId>,

	// Holder's public key
	pub public_key: BoundedVec<u8, PublicKeySize>,

	// Block number
	pub block_number: BlockNumber,

	// Block number
	pub updated_block_number: BlockNumber,

	// active
	pub active: Option<bool>,
}

impl<MetadataSize> DIDSignature<MetadataSize>
where
	MetadataSize: Get<u32>,
{
	pub fn default() -> Self {
		Self {
			public_key: BoundedVec::default(),
			proof: Signature::from_raw([0; 64]),
			active: true,
		}
	}
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum VerifiableCredentialStatus {
	Pending,
	Created,
	Scanned,
	Suspended,
	Resumed,
	Migrated,
	Split,
	Merged,
	Frozen,
}

#[derive(
	Encode, Decode, CloneNoBound, PartialEqNoBound, Eq, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,
)]
#[codec(mel_bound(AccountId: MaxEncodedLen, BlockNumber: MaxEncodedLen))]
pub struct VerifiableCredentialLog<AccountId, BlockNumber>
where
	AccountId: Clone + PartialEq + Debug,
	BlockNumber: Clone + PartialEq + Debug + PartialOrd,
{
	pub account_id: Option<AccountId>,

	pub status: VerifiableCredentialStatus,

	pub block_number: Option<BlockNumber>,
}

impl<AccountId, BlockNumber> VerifiableCredentialLog<AccountId, BlockNumber>
where
	AccountId: Clone + PartialEq + Debug,
	BlockNumber: Clone + PartialEq + Debug + PartialOrd,
{
	pub fn default() -> Self {
		Self { account_id: None, status: VerifiableCredentialStatus::Pending, block_number: None }
	}
}
