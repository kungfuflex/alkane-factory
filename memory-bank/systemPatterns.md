# Alkane Factory System Patterns

## System Architecture

The Alkane Factory follows a modular architecture designed to provide reusable token templates for the ALKANES metaprotocol. The architecture consists of several key layers:

### 1. Core Support Layer

The core support layer provides the fundamental functionality for token creation and management:

- **Location**: `crates/alkane-factory-support/`
- **Key Components**:
  - `MintableToken` trait: Defines standard token operations
  - Storage patterns for token state
  - Utility functions for token operations

This layer serves as the foundation for all token implementations, ensuring consistency across different token types.

### 2. Token Template Layer

The token template layer contains concrete implementations of different token types:

- **Location**: `alkanes/`
- **Key Components**:
  - `owned-token`: Standard token with ownership verification
  - `free-mint`: Token that can be freely minted up to a cap
  - `merkle-distributor`: Token distribution based on Merkle proofs

Each template extends the base functionality with specialized features while maintaining a consistent interface.

### 3. Build System Layer

The build system layer handles the compilation and preparation of token templates for deployment:

- **Location**: `build.rs`
- **Key Components**:
  - WebAssembly compilation
  - WASM compression
  - Test file generation

This layer ensures that token templates are properly compiled, compressed, and ready for deployment to Bitcoin.

### 4. Testing Layer

The testing layer provides utilities for testing token templates:

- **Location**: `src/tests/`
- **Key Components**:
  - Test fixtures
  - Helper functions
  - Test cases for different token types

This layer ensures that token templates function correctly and meet their specifications.

## Key Design Patterns

### 1. Factory Pattern

The factory pattern is central to the Alkane Factory project, enabling the creation of tokens with consistent interfaces but specialized behaviors:

```
┌─────────────────┐     ┌─────────────────┐
│                 │     │                 │
│  Factory        │────▶│   Token         │
│  Contract       │     │   Template      │
│                 │     │                 │
└─────────────────┘     └─────────────────┘
         │                       │
         │                       │ instantiates
         │                       ▼
         │              ┌─────────────────┐
         │              │                 │
         └─────────────▶│   Token         │
                        │   Instance      │
                        │                 │
                        └─────────────────┘
```

**Implementation**:
- Token templates are deployed to specific addresses
- Factory operations use special headers like `[1, 0]`, `[5, n]`, or `[6, n]` for deployment
- New token instances are created by cloning templates

### 2. Trait-based Design

The project uses Rust's trait system to define interfaces and behaviors:

```rust
pub trait MintableToken {
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn set_name_and_symbol(&self, name: u128, symbol: u128);
    fn total_supply(&self) -> u128;
    fn set_total_supply(&self, v: u128);
    fn increase_total_supply(&self, v: u128) -> Result<()>;
    fn mint(&self, context: &Context, value: u128) -> Result<AlkaneTransfer>;
    fn data(&self) -> Vec<u8>;
    fn set_data(&self) -> Result<()>;
    fn observe_initialization(&self) -> Result<()>;
    // Default implementations provided...
}
```

This pattern enables:
- Polymorphism across different token types
- Code reuse through default implementations
- Clear interface definitions

### 3. Command Pattern

The command pattern is used for handling different operations through opcodes:

```rust
impl AlkaneResponder for OwnedToken {
    fn execute(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut inputs = context.inputs.clone();
        match shift_or_err(&mut inputs)? {
            0 => {
                // Initialize token
                // ...
            }
            77 => {
                // Mint new tokens
                // ...
            }
            99 => {
                // Get token name
                // ...
            }
            // Other opcodes...
            999 => {
                // Get all token details
                // ...
            }
            _ => Err(anyhow!("unrecognized opcode")),
        }
    }
}
```

This pattern enables:
- Clear separation of different operations
- Consistent interface across different token types
- Easy extension with new operations
- Standardization of opcodes across different token implementations

#### Standardized Opcode 999

The project has standardized on opcode 999 across different token types to return all token details in a single call:

```rust
pub struct TokenDetails {
    pub name: String,
    pub symbol: String,
    pub total_supply: u128,
    pub cap: u128,
    pub minted: u128,
    pub value_per_mint: u128,
}

impl MintableAlkane {
    pub fn token_details(&self) -> Result<CallResponse> {
        let details = TokenDetails {
            name: self.name(),
            symbol: self.symbol(),
            total_supply: self.total_supply(),
            cap: self.cap(),
            minted: self.minted(),
            value_per_mint: self.value_per_mint(),
        };
        
        let mut response = CallResponse::default();
        response.data = details.try_to_vec();
        
        Ok(response)
    }
}
```

This standardization provides:
- Consolidated data retrieval with a single call
- Consistent interface across different token types
- Improved client interaction efficiency
- Standardized serialization format for token data

### 4. Storage Pattern

The project uses a key-value storage pattern for persistent state:

```rust
fn name_pointer() -> StoragePointer {
    StoragePointer::from_keyword("/name")
}

fn symbol_pointer() -> StoragePointer {
    StoragePointer::from_keyword("/symbol")
}

fn total_supply_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/totalsupply")
}
```

This pattern enables:
- Consistent storage access across different token types
- Clear organization of token state
- Efficient state management

### 5. Authentication Pattern

The owned token implementation uses an authentication pattern for ownership verification:

```rust
impl AuthenticatedResponder for OwnedToken {}

// In the execute method:
self.only_owner()?;  // Verifies that the caller is the owner
```

This pattern enables:
- Secure access control for privileged operations
- Clear separation of authentication logic
- Consistent ownership verification

## Component Relationships

### Token Template and MintableToken Trait

Token templates implement the `MintableToken` trait to provide standard token functionality:

```
┌─────────────────┐
│                 │
│  MintableToken  │
│  Trait          │
│                 │
└─────────────────┘
         ▲
         │ implements
         │
┌─────────────────┐
│                 │
│  Token          │
│  Template       │
│                 │
└─────────────────┘
```

This relationship ensures that all token templates provide a consistent interface while allowing for specialized behavior.

### AlkaneResponder and Token Templates

Token templates implement the `AlkaneResponder` trait to handle operations:

```
┌─────────────────┐
│                 │
│  AlkaneResponder│
│  Trait          │
│                 │
└─────────────────┘
         ▲
         │ implements
         │
┌─────────────────┐
│                 │
│  Token          │
│  Template       │
│                 │
└─────────────────┘
```

This relationship enables token templates to be executed within the ALKANES runtime environment.

### Build System and Token Templates

The build system compiles token templates to WebAssembly:

```
┌─────────────────┐     ┌─────────────────┐
│                 │     │                 │
│  Build System   │────▶│   Token         │
│                 │     │   Template      │
│                 │     │                 │
└─────────────────┘     └─────────────────┘
                                │
                                │ compiles to
                                ▼
                        ┌─────────────────┐
                        │                 │
                        │   WASM Binary   │
                        │                 │
                        └─────────────────┘
```

This relationship ensures that token templates are properly compiled and prepared for deployment.

## Data Flow Patterns

### 1. Token Initialization Flow

```
1. User creates a transaction with the token template WASM in a witness envelope
2. User includes initialization parameters in the cellpack
3. The transaction is submitted to the Bitcoin network
4. The ALKANES indexer processes the transaction
5. The token template is instantiated with the provided parameters
6. The token state is initialized
```

### 2. Token Minting Flow

```
1. User creates a transaction with a cellpack targeting the token
2. User includes the mint opcode and parameters in the cellpack
3. The transaction is submitted to the Bitcoin network
4. The ALKANES indexer processes the transaction
5. The token's mint function is called with the provided parameters
6. The token's total supply is increased
7. The minted tokens are transferred to the specified recipient
```

### 3. Token Query Flow

#### Individual Property Query
```
1. User creates a transaction with a cellpack targeting the token
2. User includes the query opcode in the cellpack (e.g., 99 for name, 100 for symbol)
3. The transaction is submitted to the Bitcoin network
4. The ALKANES indexer processes the transaction
5. The token's query function is called
6. The requested data is returned in the response
```

#### Consolidated Token Details Query (Opcode 999)
```
1. User creates a transaction with a cellpack targeting the token
2. User includes the opcode 999 in the cellpack
3. The transaction is submitted to the Bitcoin network
4. The ALKANES indexer processes the transaction
5. The token's token_details function is called
6. All token details are serialized into a single response
7. The consolidated data is returned in the response
```

This consolidated query pattern improves efficiency by:
- Reducing the number of transactions needed to retrieve token information
- Providing a consistent interface across different token types
- Enabling clients to retrieve all token data with a single call

## Error Handling Strategy

The project uses Rust's `anyhow` for error handling:

```rust
fn observe_initialization(&self) -> Result<()> {
    let mut pointer = StoragePointer::from_keyword("/initialized");
    if pointer.get().len() == 0 {
        pointer.set_value::<u8>(0x01);
        Ok(())
    } else {
        Err(anyhow!("already initialized"))
    }
}
```

This approach provides:
- Rich error context
- Clear error messages
- Consistent error handling across the codebase

## Testing Approach

The project includes several testing approaches:

1. **Unit Tests**: Testing individual components
   ```rust
   #[test]
   fn test_token_initialization() {
       // Test implementation...
   }
   ```

2. **Integration Tests**: Testing interactions between components
   ```rust
   #[test]
   fn test_token_minting() {
       // Test implementation...
   }
   ```

3. **WASM Tests**: Testing the compiled WebAssembly
   ```rust
   #[wasm_bindgen_test]
   fn test_wasm_token() {
       // Test implementation...
   }
   ```

This comprehensive testing approach ensures that token templates function correctly in different contexts.

## Deployment Pattern

The project uses a specific pattern for deploying tokens to Bitcoin:

1. **Compilation**: Token templates are compiled to WebAssembly
2. **Compression**: WASM files are compressed with gzip
3. **Witness Envelope**: The compressed WASM is included in a transaction witness envelope
4. **Initialization**: The token is initialized with a cellpack containing parameters

This pattern enables efficient deployment of token templates to Bitcoin while leveraging the witness data for additional functionality.

## AlkaneId Addressing System

The ALKANES addressing system is a key part of the protocol:

- Alkanes are addressed by their AlkaneId (block and tx fields)
- Addresses are in the format `[2, n]` or `[3, n]`, where n is a u128 value
- Factory operations use special headers like `[1, 0]`, `[5, n]`, or `[6, n]` for deployment

This addressing system enables efficient token instantiation and interaction within the ALKANES metaprotocol.

## Cellpack Structure

Cellpacks are the standard format for interacting with alkanes:

- The first two varints specify the target alkane or a special deployment operation
- The remaining varints are inputs to the alkane, with the first typically being an opcode

This structure provides a consistent interface for token operations while enabling flexible parameter passing.

## Conclusion

The Alkane Factory project employs a variety of design patterns and architectural approaches to provide a flexible and powerful framework for token creation on Bitcoin. By leveraging the ALKANES metaprotocol and WebAssembly, it enables complex token functionality within the constraints of Bitcoin's transaction model.