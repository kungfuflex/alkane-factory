# Alkane Factory Technical Context

## Technologies Used

### Programming Languages

- **Rust**: The primary language used throughout the project. Chosen for its performance, memory safety, and strong type system.
- **WebAssembly (WASM)**: The compilation target for token templates, enabling portable and sandboxed execution on the Bitcoin blockchain.

### Build Tools

- **Cargo**: Rust's package manager and build system, used for dependency management and compilation.
- **build.rs**: Custom build script for compiling token templates to WebAssembly and preparing them for deployment.
- **gzip**: Used for compressing WASM files before deployment to Bitcoin.

### Runtime Environment

- **ALKANES Runtime**: Provides the execution environment for token templates.
- **METASHREW Indexer**: Processes Bitcoin blocks and extracts ALKANES protocol messages.

### Storage

- **Key-Value Storage**: Used for persistent state management through the ALKANES runtime.
- **StoragePointer**: Abstraction for accessing and modifying state.

### Data Structures

- **TokenDetails**: Standardized structure for returning comprehensive token information:
  ```rust
  pub struct TokenDetails {
      pub name: String,
      pub symbol: String,
      pub total_supply: u128,
      pub cap: u128,
      pub minted: u128,
      pub value_per_mint: u128,
  }
  ```
- **Serialization Format**: Binary serialization format for token data:
  ```rust
  impl TokenDetails {
      pub fn try_to_vec(&self) -> Vec<u8> {
          let mut bytes = Vec::new();
          
          // Add the name
          let name_bytes = self.name.as_bytes();
          bytes.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
          bytes.extend_from_slice(name_bytes);
          
          // Add the symbol
          let symbol_bytes = self.symbol.as_bytes();
          bytes.extend_from_slice(&(symbol_bytes.len() as u32).to_le_bytes());
          bytes.extend_from_slice(symbol_bytes);
          
          // Add numeric values
          bytes.extend_from_slice(&self.total_supply.to_le_bytes());
          bytes.extend_from_slice(&self.cap.to_le_bytes());
          bytes.extend_from_slice(&self.minted.to_le_bytes());
          bytes.extend_from_slice(&self.value_per_mint.to_le_bytes());
          
          bytes
      }
  }
  ```

### Testing Frameworks

- **wasm-bindgen-test**: Used for testing WebAssembly code.
- **Rust's built-in testing framework**: Used for unit and integration testing.

## Development Setup

### Prerequisites

- **Rust Toolchain**: Required for building the project.
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **wasm32-unknown-unknown Target**: Required for compiling to WebAssembly.
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

- **wasm-bindgen-cli**: Required for testing WebAssembly code.
  ```bash
  cargo install -f wasm-bindgen-cli
  ```

### Building the Project

The project can be built with Cargo:

```bash
cargo build --release
```

This will:
1. Compile the Rust code
2. Run the build.rs script
3. Compile token templates to WebAssembly
4. Compress the WASM files
5. Generate test files

The build produces:
- WASM files in `target/wasm32-unknown-unknown/release/`
- Compressed WASM files with the `.wasm.gz` extension
- Test files in `src/tests/std/`

### Testing

The project includes several testing approaches:

1. **Unit Tests**: Testing individual components.
   ```bash
   cargo test
   ```

2. **WASM Tests**: Testing the compiled WebAssembly.
   ```bash
   cargo test --target wasm32-unknown-unknown
   ```

### Deployment

Token templates can be deployed to Bitcoin using the following process:

1. Build the token template:
   ```bash
   cargo build --release
   ```

2. Compress the WASM file:
   ```bash
   gzip -9 -c target/wasm32-unknown-unknown/release/owned_token.wasm > owned_token.wasm.gz
   ```

3. Create a transaction with the compressed WASM in a witness envelope.

4. Submit the transaction to the Bitcoin network.

5. Initialize the token with a cellpack containing parameters.

## Dependencies

### Core Dependencies

- **alkanes-support**: Core utilities for the ALKANES protocol.
  ```toml
  alkanes-support = { git = "https:/github.com/kungfuflex/alkanes-rs" }
  ```

- **alkanes-runtime**: Smart contract runtime environment.
  ```toml
  alkanes-runtime = { git = "https://github.com/kungfuflex/alkanes-rs" }
  ```

- **metashrew-support**: Support library for blockchain interaction.
  ```toml
  metashrew-support = { git = "https://github.com/kungfuflex/alkanes-rs" }
  ```

- **protorune-support**: Support for the protorunes protocol.
  ```toml
  protorune-support = { git = "https://github.com/kungfuflex/alkanes-rs" }
  ```

- **bitcoin**: Bitcoin data structures and utilities.
  ```toml
  bitcoin = { version = "0.32.4", features = ["rand"] }
  ```

- **anyhow**: Error handling library.
  ```toml
  anyhow = "1.0.94"
  ```

- **hex**: Hexadecimal encoding/decoding.
  ```toml
  hex = "0.4.3"
  ```

### Development Dependencies

- **alkanes**: ALKANES metaprotocol implementation with test utilities.
  ```toml
  alkanes = { git = "https://github.com/kungfuflex/alkanes-rs", features = ["test-utils"] }
  ```

- **metashrew**: Blockchain indexer with test utilities.
  ```toml
  metashrew = { git = "https://github.com/kungfuflex/alkanes-rs", features = ["test-utils"] }
  ```

- **protorune**: Protorunes protocol implementation with test utilities.
  ```toml
  protorune = { git = "https://github.com/kungfuflex/alkanes-rs", features = ["test-utils"] }
  ```

- **wasm-bindgen**: WebAssembly bindings for Rust.
  ```toml
  wasm-bindgen = "0.2.99"
  ```

- **wasm-bindgen-test**: Testing framework for WebAssembly.
  ```toml
  wasm-bindgen-test = "0.3.49"
  ```

- **hex_lit**: Hexadecimal literals for Rust.
  ```toml
  hex_lit = "0.1.1"
  ```

### Build Dependencies

- **anyhow**: Error handling utilities.
  ```toml
  anyhow = "1.0.90"
  ```

- **flate2**: Compression utilities for WASM files.
  ```toml
  flate2 = "1.0.34"
  ```

- **hex**: Hexadecimal encoding/decoding.
  ```toml
  hex = "0.4.3"
  ```

## Technical Constraints

### Bitcoin Compatibility

Token templates must operate within the constraints of the Bitcoin protocol:

- Limited transaction size
- Limited script capabilities
- No native smart contract support
- Immutable transaction history

These constraints are addressed by:
- Compiling token templates to WebAssembly
- Using witness data for contract code and data
- Leveraging the ALKANES metaprotocol for execution

### WebAssembly Limitations

Token templates must operate within WebAssembly constraints:

- Limited memory model
- No direct system access
- Deterministic execution
- Limited floating-point precision

These constraints are addressed by:
- Careful memory management
- Using the ALKANES runtime for system access
- Ensuring deterministic behavior
- Avoiding floating-point operations

### Storage Limitations

Token state must be stored efficiently:

- Key-value storage model
- Limited storage space
- No complex queries

These constraints are addressed by:
- Using efficient storage patterns
- Minimizing state size
- Using simple key-value lookups

### Deployment Size

Token templates must be small enough for efficient deployment:

- WASM files should be as small as possible
- Compression is used to reduce size further

These constraints are addressed by:
- Minimizing code size
- Using gzip compression
- Avoiding unnecessary dependencies

## Feature Flags

The project uses feature flags to control compilation:

- **test**: Enables test-specific functionality
- **testnet**: Configures for Bitcoin testnet
- **dogecoin**: Configures for Dogecoin network
- **luckycoin**: Configures for Luckycoin network
- **bellscoin**: Configures for Bellscoin network
- **fractal**: Configures for Fractal network
- **mainnet**: Configures for Bitcoin mainnet

These feature flags can be enabled during compilation:

```bash
cargo build --release --features mainnet
```

## Build System

The project uses a custom build script (`build.rs`) that:

1. Compiles each token template to WebAssembly:
   ```rust
   Command::new("cargo")
       .env("CARGO_TARGET_DIR", wasm_str)
       .arg("build")
       .arg("--release")
       .spawn()?
       .wait()?;
   ```

2. Compresses the WASM files with gzip:
   ```rust
   let mut writer = GzEncoder::new(Vec::<u8>::with_capacity(binary.len()), Compression::best());
   writer.write_all(&binary)?;
   Ok(writer.finish()?)
   ```

3. Generates test files with the compiled WASM bytecode:
   ```rust
   fs::write(
       &write_dir.join("std").join(subbed.clone() + "_build.rs"),
       String::from("use hex_lit::hex;\n#[allow(long_running_const_eval)]\npub fn get_bytes() -> Vec<u8> { (&hex!(\"")
           + data.as_str()
           + "\")).to_vec() }",
   )?;
   ```

4. Creates a module structure for testing:
   ```rust
   fs::write(
       &write_dir.join("std").join("mod.rs"),
       mods.into_iter()
           .map(|v| v.replace("-", "_"))
           .fold(String::default(), |r, v| {
               r + "pub mod " + v.as_str() + "_build;\n"
           }),
   )
   ```

This build system ensures that token templates are properly compiled, compressed, and prepared for testing and deployment.

## Deployment Procedure

The deployment procedure for token templates involves several steps:

1. **Build**: Compile the token template to WebAssembly.
   ```bash
   cargo build --release
   ```

2. **Compress**: Compress the WASM file with gzip.
   ```bash
   gzip -9 -c target/wasm32-unknown-unknown/release/owned_token.wasm > owned_token.wasm.gz
   ```

3. **Create Transaction**: Create a Bitcoin transaction with the compressed WASM in a witness envelope.

4. **Submit Transaction**: Submit the transaction to the Bitcoin network.

5. **Initialize**: Initialize the token with a cellpack containing parameters.

This procedure enables efficient deployment of token templates to Bitcoin while leveraging the witness data for additional functionality.

## Documentation

The project includes several forms of documentation:

- **README.md**: Provides basic information about the project and usage instructions.
- **Code Comments**: Includes documentation for key functions and components.
- **Memory Bank**: Comprehensive documentation of the project's architecture, design patterns, and technical context.

Additional documentation for the ALKANES metaprotocol is available at:
- [https://github.com/kungfuflex/alkanes-rs/wiki](https://github.com/kungfuflex/alkanes-rs/wiki)

## Integration with ALKANES Ecosystem

The Alkane Factory integrates with the broader ALKANES ecosystem:

- **ALKANES-RS**: Provides the core runtime and support libraries.
- **METASHREW**: Provides the indexer infrastructure for processing Bitcoin blocks.
- **Protorune**: Provides the base token protocol.

This integration enables the Alkane Factory to leverage the existing infrastructure while providing specialized token functionality.

## Security Considerations

The project includes several security considerations:

- **Ownership Verification**: The owned token implementation includes ownership verification for privileged operations.
- **Input Validation**: All inputs are validated before processing.
- **Error Handling**: Comprehensive error handling prevents unexpected behavior.
- **Initialization Checks**: Tokens can only be initialized once.

These security considerations ensure that token templates operate safely and securely within the ALKANES metaprotocol.

## Performance Considerations

The project includes several performance considerations:

- **Efficient Storage**: Storage patterns are designed for efficient access.
- **Minimal State**: State is kept as minimal as possible.
- **Optimized WASM**: WASM files are optimized for size and performance.
- **Compression**: WASM files are compressed to reduce deployment size.

These performance considerations ensure that token templates operate efficiently within the constraints of the Bitcoin blockchain.