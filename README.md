# alkane-factory

Templates for alkanes that are meant to be factory instantiated with an attached witness envelope for content.

Tokens adhere to the alkane token format for fully qualified tokens, with name(), symbol(), and data().


## Build

```sh
cargo build --release
```

WASM will be built to `target/wasm32-unknown-unknown/release/owned_token.wasm`

gzip compression level 9 is recommended to compress the wasm to a `*.wasm.gz` file before deploying to Bitcoin.

## Usage

This alkane implements the following opcodes:

- 0: `initialize(mint_auth_token_amount: u128, mint_amount: u128)`
- 99: `name(): String`
- 100: `symbol(): String`
- 10000: `data(): Vec<u8>`


## Author

flex

## License

MIT
