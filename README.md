# Web3-WebAssembly Rust EIP-1193 (Metamask) Example

This code demonstrates with fewest dependencies how to connect metamask and other eip-1193
compatible wallets in webassembly with rust (no javascript). Currently the web3 helper library
(which performs tasks such as call bytecode serialisation etc) is
[rust-web3](https://github.com/tomusdrw/rust-web3), as ethers-rs lacks support for an eip-1193
transport, but I have manually written a transport in other examples, so I could PR this
functionality if there is a demand.

## Usage

```
# enter a nix shell (or colect the deps yourself if you aren't using nix)
nix develop

# run trunk
trunk serve
```
