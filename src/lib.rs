#![deny(clippy::all, clippy::perf, clippy::correctness)]
#![allow(clippy::too_many_arguments)]

pub mod post;
pub mod seal;

mod registry;
mod types;

pub use crate::registry::{RegisteredPoStProof, RegisteredSealProof};
pub use crate::types::{PrivateReplicaInfo, PublicReplicaInfo};

pub use filecoin_proofs_v1::storage_proofs::election_post::Candidate;
pub use filecoin_proofs_v1::storage_proofs::fr32;
pub use filecoin_proofs_v1::storage_proofs::sector::SectorId;
pub use filecoin_proofs_v1::types::{
    ChallengeSeed, Commitment, PaddedBytesAmount, PieceInfo, ProverId, Ticket, UnpaddedByteIndex,
    UnpaddedBytesAmount, SectorSize,
};
pub use filecoin_proofs_v1::SnarkProof;
