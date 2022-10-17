[EIP-712](https://eips.ethereum.org/EIPS/eip-712): Typed Structured Data Hashing and Signing

EIP-712 is a standard for structuring typed data for Ethereum smart contracts. It is a standard for how to encode typed data according to EIP-191 so that the encoded data may be used to produce a cryptographic hash and a signature.

## Motivation

There are many use cases where it is useful to allow a smart contract to verify that a message was signed by a known account. This is useful for cases such as signed approvals, price oracles, and authenticated off-chain protocols.

The Ethereum ABI specification does not provide encoding for structs, making it difficult to encode arbitrary data for signing. This EIP provides a basic specification for encoding typed data.