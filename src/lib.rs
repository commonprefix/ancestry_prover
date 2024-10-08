//! # Ethereum Ancestry Prover
//!
//! An all-in-one solution for proving that a beacon block is a predecessor of another beacon block.
//!
//! > **Note:** Currently, the proof is derived only from the `block_roots` beacon state property, thus the target block cannot be older than `SLOTS_PER_HISTORICAL_ROOT` (8192 blocks, ~27 hours). Support for older blocks is planned.
//!
//! ## Providers
//!
//! The library makes use of proof providers, which are responsible for fetching the necessary data from the Ethereum chain, and generating the proofs.
//!
//! Supported providers are:
//!
//! - `LodestarProvider`: Utilizes the [Lodestar](http://lodestar.chainsafe.io) beacon node.
//! - `StateProverProvider`: Uses [`state prover`](https://github.com/commonprefix/state-prover) to interact with the Lodestar API, useful for generating single merkle proofs.
//!
//! ## Usage
//!
//! ```rust
//! use ancestry_prover::{AncestryProver, LodestarProvider, verify};
//!
//! let prover_api = LodestarProvider::new("https://lodestar-mainnet.chainsafe.io".to_string());
//!
//! // Alternatively, you can use the StateProverProvider
//! // or a custom provider that implements the ProverProvider trait.
//! // let prover_api = StateProverProvider::new(
//! //     "mainnet".to_string(),
//! //     "http://127.0.0.1:3000".to_string(),
//! // );
//!
//! let prover = AncestryProver::new(prover_api);
//!
//! // This is the block slot that we want to prove
//! let target_block_slot = 8942024;
//!
//! let recent_block_slot = 8942159;
//! let recent_block_state_id =
//!     "0xca0ad12cf0a4d5935c1636a88bc7d22ccacc86637f406e799f3b20d22ca715f8";
//!
//! let proof = prover
//!     .prove(target_block_slot, recent_block_slot, recent_block_state_id)
//!     .await
//!     .unwrap();
//!
//! assert!(verify(
//!     &proof,
//!     target_block_slot,
//!     recent_block_slot,
//!     recent_block_state_id,
//! ));
//! ```
//!
//! This crate allows you to easily verify the ancestry of Ethereum beacon blocks using the provided providers or any custom provider that implements the `ProverProvider` trait.

pub mod errors;
pub mod lodestar_provider;
pub mod prover;
pub mod provider;
pub mod state_prover_provider;

pub use prover::verify;
pub use prover::AncestryProver;
pub use provider::{BlockRootsProof, ProofProvider};

pub use lodestar_provider::LodestarProvider;
pub use state_prover_provider::StateProverProvider;
