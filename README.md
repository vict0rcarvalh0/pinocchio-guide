# Pinocchio Guide

[![Pinocchio](https://img.shields.io/badge/pinocchio-v0.10.1-blue)](https://github.com/anza-xyz/pinocchio)

A comprehensive guide and examples for building Solana programs using [Pinocchio](https://github.com/anza-xyz/pinocchio) - a zero-dependency Rust library for efficient on-chain programs.

## What is Pinocchio?

Pinocchio is a zero-dependency library to create Solana programs in Rust. It takes advantage of the way SVM loaders serialize program input parameters into a byte array, defining zero-copy types to read the input directly. This completely eliminates the dependency on the `solana-program` crate, which mitigates dependency issues by having a crate specifically designed to create on-chain programs.

As a result, Pinocchio can be used as a replacement for `solana-program` to write on-chain programs that are optimized in terms of both **compute units consumption** and **binary size**.

## Documentation

| Document | Description |
|----------|-------------|
| [**GUIDE.md**](./GUIDE.md) | Comprehensive API reference with all system and token instruction examples |
| [**TUTORIAL.md**](./TUTORIAL.md) | Step-by-step tutorial: Transform an Anchor Vault into a Pinocchio Vault |

## Quick Start

Add Pinocchio to your `Cargo.toml`:

```toml
[dependencies]
pinocchio = { version = "0.10.1", features = ["cpi"] }
pinocchio-system = "0.5.0"
pinocchio-token = "0.5.0"
```

## Key API Types

| Type | Description |
|------|-------------|
| `AccountView` | Zero-copy account representation (replaces `AccountInfo`) |
| `Address` | 32-byte public key type (replaces `Pubkey`) |
| `ProgramResult` | Result type for instruction processing |
| `cpi::Signer` | Signer seeds for cross-program invocations |

## Example: Transfer Lamports

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

pub fn process_transfer<'a>(
    accounts: &'a [AccountView],
    lamports: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let from_account = &accounts[0];
    let to_account = &accounts[1];

    if !from_account.is_writable() || !from_account.is_signer() {
        return Err(ProgramError::InvalidAccountData);
    }

    Transfer {
        from: from_account,
        to: to_account,
        lamports,
    }
    .invoke_signed(signers)
}
```

## Examples

The `examples/` directory contains complete working implementations of all system and token instructions using Pinocchio v0.10.1.

```bash
cd examples

# Build
cargo build-sbf

# Test (if tests are present)
SBF_OUT_DIR="$(pwd)/target/deploy" cargo test
```

## Resources

- [Pinocchio GitHub](https://github.com/anza-xyz/pinocchio)
- [Solana Documentation](https://docs.solana.com/)
- [SPL Token Program](https://spl.solana.com/token)

## Contributing

Contributions are welcome. Feel free to submit issues or pull requests to improve the guide, examples, or documentation.
