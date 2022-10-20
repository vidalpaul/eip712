//! # EIP-712
//! This crate provides a simple implementation of the [EIP-712](https://eips.ethereum.org/EIPS/eip-712) specification.

pub use parity_crypto::Keccak256;
pub use primitive_types::{H160, H256, U256};

pub mod domain;
pub mod encode;
pub mod s;
pub mod sol;

pub use domain::*;
pub use s::*;
