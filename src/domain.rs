use crate::{H160, H256, U256};

pub struct EIP712Domain {
    pub name: String,
    pub version: String,
    pub chain_id: U256,
    pub verifying_contract: H160,
    pub salt: H256,
}