# Alkane Factory Project Brief

## Project Overview

The Alkane Factory is a specialized component of the ALKANES metaprotocol ecosystem, designed to provide templates for creating and managing "alkanes" - tokens that adhere to a specific token format for fully qualified tokens on Bitcoin. The project implements a factory pattern for instantiating tokens with attached witness envelopes for content.

### Purpose and Goals
- Provide reusable templates for different types of token implementations on Bitcoin
- Enable factory instantiation of tokens with witness envelopes for content
- Support various token types including owned tokens, free-mint tokens, and merkle-distributor tokens
- Implement standard interfaces for token functionality (name, symbol, data)
- Facilitate the creation of DeFi primitives on Bitcoin through the ALKANES metaprotocol

### Scope
The project focuses specifically on token factory functionality within the broader ALKANES ecosystem, which is a Rust implementation of a Bitcoin-based DeFi metaprotocol. The alkane-factory provides the templates and support code for creating different types of tokens that can be deployed on Bitcoin.

## Technical Context

### Programming Languages & Technologies
- **Primary Language**: Rust
- **Target Platform**: WebAssembly (WASM) for deployment on Bitcoin
- **Build System**: Cargo (Rust's package manager)
- **Compression**: gzip for WASM compression before deployment
- **Blockchain**: Bitcoin and compatible networks (testnet, dogecoin, luckycoin, bellscoin, fractal)

### Dependencies & Libraries
- **Core Dependencies**:
  - `alkanes-support`, `alkanes-runtime`: Core libraries for alkane functionality
  - `metashrew-support`: Support library for blockchain interaction
  - `protorune-support`: Support for the protorunes protocol
  - `bitcoin`: Bitcoin library with rand features
  - `anyhow`: Error handling library
  - `hex`: Hexadecimal encoding/decoding
  - `flate2`: Compression utilities for WASM files

### System Architecture

The project follows a modular architecture with the following key components:

1. **Core Factory Support**:
   - Provides the base functionality for creating and managing tokens
   - Implements the `MintableToken` trait with standard token operations
   - Defines storage patterns for token data

2. **Token Templates**:
   - Multiple token implementations in the `alkanes/` directory
   - Each implementation extends the base functionality with specific features:
     - **owned-token**: Standard token with ownership verification
     - **free-mint**: Token that can be freely minted up to a cap
     - **merkle-distributor**: Token distribution based on Merkle proofs

3. **Runtime Environment**:
   - Uses the alkanes runtime for execution context
   - Implements the `AlkaneResponder` trait for handling operations
   - Provides storage abstractions for persistent state

### Design Patterns
- **Factory Pattern**: Used for token instantiation with different templates
- **Trait-based Design**: Core functionality defined in traits (`MintableToken`, `AlkaneResponder`)
- **WebAssembly Target**: Compiled to WASM for blockchain deployment
- **Cellpack Structure**: Standard format for interacting with alkanes
- **Storage Pointers**: Key-value storage for persistent state

## Source Code Modules

### Core Modules

#### `crates/alkane-factory-support`
- **Purpose**: Provides the core functionality for the token factory
- **Key Files**:
  - `factory.rs`: Implements the `MintableToken` trait with standard token operations
  - `constants.rs`: Defines constants used throughout the project
  - `lib.rs`: Exports the module structure

#### `alkanes/`
- **Purpose**: Contains different token implementations
- **Sub-modules**:
  - `owned-token/`: Implementation of a standard owned token
    - Supports initialization, minting, and querying token properties
    - Includes ownership verification for certain operations
  - `free-mint/`: Implementation of a token that can be freely minted
    - Supports a configurable cap on total supply
    - Tracks minted tokens and value per mint
    - Allows anyone to mint tokens up to the cap
  - `merkle-distributor/`: Implementation for distributing tokens based on Merkle proofs
    - Uses Merkle trees for efficient verification of distribution rights
    - Locks tokens for distribution based on proofs
    - Verifies outputs against the Merkle root

### Token Implementation Details

#### `owned-token`
- Implements the `MintableToken` trait
- Supports operations like initialization, minting, and querying token properties
- Opcodes:
  - `0`: Initialize token with mint authorization and amount
  - `77`: Mint new tokens (owner only)
  - `99`: Get token name
  - `100`: Get token symbol
  - `101`: Get total supply
  - `1000`: Get token data

#### `free-mint`
- Implements the `MintableToken` trait
- Allows anyone to mint tokens up to a configurable cap
- Tracks the number of tokens minted and the value per mint
- Opcodes:
  - `0`: Initialize token with initial amount, value per mint, cap, name, and symbol
  - `77`: Mint tokens (available to anyone)
  - `99`: Get token name
  - `100`: Get token symbol
  - `101`: Get total supply
  - `102`: Get cap
  - `103`: Get minted amount
  - `104`: Get value per mint
  - `1000`: Get token data

#### `merkle-distributor`
- Implements the `AlkaneResponder` trait
- Uses Merkle proofs for efficient token distribution
- Locks tokens for distribution based on proofs
- Opcodes:
  - `0`: Initialize distributor with length, root, and locked tokens
  - `1`: Claim tokens by providing a valid Merkle proof

### Build System

The project uses Cargo for building, with a custom `build.rs` script that:
1. Builds each token implementation to WebAssembly
2. Compresses the WASM files with gzip
3. Generates test files with the compiled WASM bytecode
4. Creates a module structure for testing

The build process produces WASM files that are compressed with gzip before deployment to Bitcoin.

## Integration with ALKANES Metaprotocol

The alkane-factory is part of the broader ALKANES metaprotocol ecosystem, which provides a framework for creating and executing smart contracts on the Bitcoin blockchain. Key integration points include:

### Cellpack Structure
- Cellpacks are protomessages that interact with alkanes
- The first two varints in a cellpack specify the target alkane or a special deployment operation
- The remaining varints are inputs to the alkane, with the first typically being an opcode

### Deployment Process
1. Tokens are compiled to WebAssembly
2. The WASM is compressed with gzip
3. The compressed WASM is deployed to Bitcoin through a transaction with a witness envelope
4. The token is instantiated with a cellpack containing initialization parameters

### AlkaneId Addressing
- Alkanes are addressed by their AlkaneId (same structure as ProtoruneRuneId)
- Addresses are in the format `[2, n]` or `[3, n]`, where n is a u128 value
- Factory operations use special headers like `[1, 0]`, `[5, n]`, or `[6, n]` for deployment

## Additional Context

### Deployment
- WASM files are built to `target/wasm32-unknown-unknown/release/`
- Files are compressed with gzip level 9 before deployment to Bitcoin
- Deployment involves embedding the WASM in Bitcoin transactions with witness envelopes
- The factory pattern allows for efficient reuse of token templates

### Integration with Bitcoin
- Tokens adhere to the alkane token format for fully qualified tokens
- Standard interfaces include `name()`, `symbol()`, and `data()`
- Tokens can store arbitrary data in a compressed format
- The system leverages Bitcoin's transaction model and witness data for smart contract functionality

### Testing
- Test utilities are included as dev dependencies
- WebAssembly testing is supported through `wasm-bindgen-test`
- The build script generates test files with compiled WASM bytecode

### Project Status
- The project is part of the active ALKANES metaprotocol ecosystem
- It provides functional token templates for different use cases
- The system is designed to work across multiple Bitcoin-based networks

## Conclusion

The alkane-factory project provides a powerful framework for creating and managing tokens on Bitcoin through the ALKANES metaprotocol. By leveraging WebAssembly and the factory pattern, it enables the creation of various token types with standardized interfaces, facilitating the development of DeFi applications on Bitcoin. 