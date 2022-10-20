🗝️ [EIP-712](https://eips.ethereum.org/EIPS/eip-712): Typed Structured Data Hashing and Signing

EIP-712 is a standard for structuring typed data for Ethereum smart contracts. It is a standard for how to encode typed data according to EIP-191 so that the encoded data may be used to produce a cryptographic hash and a signature.

## Motivation

There are many use cases where it is useful to allow a smart contract to verify that a message was signed by a known account. This is useful for cases such as signed approvals, price oracles, and authenticated off-chain protocols.

The Ethereum ABI specification does not provide encoding for structs, making it difficult to encode arbitrary data for signing. This EIP provides a basic specification for encoding typed data.

## Specification

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

### Definitions

- `domain` is a set of information about the context in which the message is being signed. It is encoded as a struct with the following members:
  - `name` is a string that describes the domain to which the message is being sent. This string SHOULD be the human-readable name of the domain.
  - `version` is a string that represents the version of the domain's message signing format. This string SHOULD be a unique identifier for the version of the domain's message signing format.
  - `chainId` is a 256-bit integer that represents the chain ID of the chain where the message is being sent.
  - `verifyingContract` is a 160-bit integer that represents the address of the contract that will be used to verify the message.
  - `salt` is a 256-bit integer that represents a random value used to ensure that signatures are unique to this message.
- `message` is a struct that contains the information to be signed. It is encoded as a struct.
