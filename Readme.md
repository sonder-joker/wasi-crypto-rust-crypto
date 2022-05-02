# WasiCrypto-RustCrypto: RustCrypto's implementation for WASI Crypto Proposal

[RustCrypto](https://github.com/RustCrypto/traits) implementation for those support [WASI-Crypto Proposal](https://github.com/WebAssembly/wasi-crypto).

It's still in progress.

For benchmark, it need nightly support.

To start, add follow in `.cargo/config.toml`

```toml
[build]
target = "wasm32-wasi"

[target.wasm32-wasi]
runner = "the-vm-path-support-wasi-crypto"
```
