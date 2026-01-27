# Pinocchio Guide

[![Tests](https://img.shields.io/badge/tests-29%20passing-brightgreen)](.)
[![Pinocchio](https://img.shields.io/badge/pinocchio-v0.10.1-blue)](https://github.com/anza-xyz/pinocchio)
[![License](https://img.shields.io/badge/license-MIT-lightgrey)](./LICENSE)

## Table of Contents

- [Introduction](#introduction)
- [Project Status](#project-status)
- [Dependencies](#dependencies)
- [Running Tests](#running-tests)
- [System Instructions](#system-instructions)
- [Token Instructions](#token-instructions)
- [Key API Changes](#key-api-changes-in-pinocchio-010x)
- [Examples] (#examples)
- [Resources](#resources)

## Introduction

Pinocchio is a zero-dependency library to create Solana programs in Rust. It takes advantage of the way SVM loaders serialize the program input parameters into a byte array that is then passed to the program's entrypoint to define zero-copy types to read the input. Since the communication between a program and SVM loader — either at the first time the program is called or when one program invokes the instructions of another program — is done via a byte array, a program can define its own types. This completely eliminates the dependency on the solana-program crate, which in turn mitigates dependency issues by having a crate specifically designed to create on-chain programs.

As a result, Pinocchio can be used as a replacement for solana-program to write on-chain programs, which are optimized in terms of both compute units consumption and binary size.

The Pinocchio Guide serves as a comprehensive resource for understanding and implementing core account management functionalities on the Solana blockchain. It emphasizes foundational principles, code-level explanations, and best practices to help developers fully leverage Solana's unique transaction optimization system.

## Project Status

**All 29 tests passing** - The project includes comprehensive test coverage using [Mollusk](https://github.com/anza-xyz/mollusk), Solana's lightweight test harness.

| Category | Instructions | Tests |
|----------|-------------|-------|
| System Instructions | 13 | All passing |
| Token Instructions | 16 | All passing |

All instructions demonstrate **real CPI (Cross-Program Invocation) calls** to the System Program and SPL Token Program.

## Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
pinocchio = { version = "0.10.1", features = ["cpi"] }
pinocchio-system = "0.5.0"
pinocchio-token = "0.5.0"
```

## Running Tests

The project uses [Mollusk](https://github.com/anza-xyz/mollusk), a lightweight test harness that provides a minified Solana Virtual Machine (SVM) environment.

```bash
# 1. Build the program for SBF (Solana BPF)
cargo build-sbf

# 2. Run tests with SBF_OUT_DIR pointing to the compiled program
SBF_OUT_DIR="$(pwd)/target/deploy" cargo test
```

**Note:** The `SBF_OUT_DIR` environment variable tells Mollusk where to find the compiled `.so` program file. Tests will fail without it.

Expected output:
```
running 29 tests
test system::assign::tests::test_assign ... ok
test system::transfer_lamports::tests::test_transfer ... ok
test token::approve::tests::test_approve ... ok
...
test result: ok. 29 passed; 0 failed; 0 ignored
```

Feel free to contribute to the repository and optimize the examples and documentation in any way!

## Documentation

- [Guide](GUIDE.md): This file provides a comprehensive explanation of Pinocchio's core functionalities, focusing on system-level operations and instruction calls. It includes examples and best practices for implementing account management functionalities on the Solana blockchain.
  
- [Tutorial](TUTORIAL.md): This file demonstrates how to transform an Anchor-based Vault into a Pinocchio-based Vault. It provides step-by-step instructions for building and optimizing a Vault program, comparing the high-level abstractions of Anchor with the low-level optimizations of Pinocchio.

Increments the nonce value in a nonce account to ensure transaction uniqueness and replay protection.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::AdvanceNonceAccount;

/// Processes the `AdvanceNonceAccount` instruction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[]` The recent blockhashes sysvar.
/// 2. `[SIGNER]` The Nonce authority.
pub fn process_advance_nonce_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    let [nonce_account, recent_blockhashes_sysvar, nonce_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    AdvanceNonceAccount {
        account: nonce_account,
        recent_blockhashes_sysvar,
        authority: nonce_authority,
    }
    .invoke_signed(signers)
}
```

### Allocate

Reserves space in an account for a specific size.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Allocate;

/// Processes the `Allocate` instruction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The account to allocate space for.
pub fn process_allocate<'a>(
    accounts: &'a [AccountView],
    space: u64,
    signers: &[Signer],
) -> ProgramResult {
    let [allocate_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !allocate_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Allocate {
        account: allocate_account,
        space,
    }
    .invoke_signed(signers)
}
```

### AllocateWithSeed

Similar to Allocate, but derives the account address using a seed and a base account.

```rust
use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::AllocateWithSeed;

/// Processes the `AllocateWithSeed` instruction.
///
/// ### Accounts:
/// 0. `[WRITE]` The allocated account.
/// 1. `[SIGNER]` The base account used to derive the allocated account.
pub fn process_allocate_with_seed<'a>(
    accounts: &'a [AccountView],
    seed: &str,
    space: u64,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    let [allocated_account, base_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !base_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    AllocateWithSeed {
        account: allocated_account,
        base: base_account,
        seed,
        space,
        owner,
    }
    .invoke_signed(signers)
}
```

### Assign

Changes the program ownership of an account.

```rust
use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Assign;

/// Processes the `Assign` instruction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The account to be reassigned.
pub fn process_assign<'a>(
    accounts: &'a [AccountView],
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    let [assigned_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !assigned_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Assign {
        account: assigned_account,
        owner,
    }
    .invoke_signed(signers)
}
```

### CreateAccount

Creates a new account on the blockchain, assigns ownership to a program, and funds the account.

```rust
use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

/// Processes the `CreateAccount` instruction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The funding account.
/// 1. `[WRITE, SIGNER]` The new account to be created.
pub fn process_create_account<'a>(
    accounts: &'a [AccountView],
    lamports: u64,
    space: u64,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    let [funding_account, new_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !funding_account.is_signer() || !new_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    CreateAccount {
        from: funding_account,
        to: new_account,
        lamports,
        space,
        owner,
    }
    .invoke_signed(signers)
}
```

### Transfer

Moves lamports (Solana's native token) between accounts.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

/// Processes the `Transfer` instruction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The source account.
/// 1. `[WRITE]` The destination account.
pub fn process_transfer<'a>(
    accounts: &'a [AccountView],
    lamports: u64,
    signers: &[Signer],
) -> ProgramResult {
    let [from_account, to_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !from_account.is_writable() || !from_account.is_signer() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !to_account.is_writable() {
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

## Token Instructions

Pinocchio provides an abstraction layer for interacting with Solana Program Library (SPL) tokens, enabling seamless management of fungible and non-fungible tokens.

### Approve

Grants another account the authority to transfer tokens on behalf of the current owner.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::Approve;

/// Processes the Approve instruction.
///
/// ### Accounts:
///   0. `[WRITE]` The token account.
///   1. `[]` The delegate account.
///   2. `[SIGNER]` The source account owner.
pub fn process_approve<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    signers: &[Signer],
) -> ProgramResult {
    let [source_account, delegate_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !source_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Approve {
        source: source_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,
    }
    .invoke_signed(signers)
}
```

### Burn

Permanently destroys a specified number of tokens from an account.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::Burn;

/// Processes the Burn instruction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to burn from.
///   1. `[WRITE]` The token mint.
///   2. `[SIGNER]` The account's owner/delegate.
pub fn process_burn<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    signers: &[Signer],
) -> ProgramResult {
    let [burn_account, mint_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !burn_account.is_writable() || !mint_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Burn {
        account: burn_account,
        mint: mint_account,
        authority: authority_account,
        amount,
    }
    .invoke_signed(signers)
}
```

### CloseAccount

Closes a token account, reclaiming its lamports.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::CloseAccount;

/// Processes the CloseAccount instruction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to close.
///   1. `[WRITE]` The destination account.
///   2. `[SIGNER]` The account's owner.
pub fn process_close_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    let [close_account, destination_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !close_account.is_writable() || !destination_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    CloseAccount {
        account: close_account,
        destination: destination_account,
        authority: authority_account,
    }
    .invoke_signed(signers)
}
```

### MintTo

Creates new tokens and deposits them into a token account.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::MintTo;

/// Processes the MintTo instruction.
///
/// ### Accounts:
///   0. `[WRITE]` The mint account.
///   1. `[WRITE]` The account to mint tokens to.
///   2. `[SIGNER]` The mint's minting authority.
pub fn process_mint_to<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    signers: &[Signer],
) -> ProgramResult {
    let [mint_account, token_account, mint_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !mint_account.is_writable() || !token_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !mint_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    MintTo {
        mint: mint_account,
        account: token_account,
        mint_authority,
        amount,
    }
    .invoke_signed(signers)
}
```

### Transfer (Token)

Moves tokens from one token account to another.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::Transfer;

/// Processes the Transfer instruction.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[WRITE]` The destination account.
///   2. `[SIGNER]` The source account's owner/delegate.
pub fn process_transfer<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    signers: &[Signer],
) -> ProgramResult {
    let [from_account, to_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !from_account.is_writable() || !to_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Transfer {
        from: from_account,
        to: to_account,
        authority: authority_account,
        amount,
    }
    .invoke_signed(signers)
}
```

### TransferChecked

Like Transfer, but with additional checks for mint and token amount validation.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::TransferChecked;

/// Processes the TransferChecked instruction.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[]` The token mint.
///   2. `[WRITE]` The destination account.
///   3. `[SIGNER]` The source account's owner/delegate.
pub fn process_transfer_checked<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    decimals: u8,
    signers: &[Signer],
) -> ProgramResult {
    let [from_account, mint_account, to_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !from_account.is_writable() || !to_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    TransferChecked {
        from: from_account,
        mint: mint_account,
        to: to_account,
        authority: authority_account,
        amount,
        decimals,
    }
    .invoke_signed(signers)
}
```

## Examples

The `examples` folder contains program examples that implement Pinocchio functions. These examples are still a work in progress and may contain errors or incomplete implementations. Contributions to improve these examples are welcome!

## Resources

- [Pinocchio GitHub](https://github.com/anza-xyz/pinocchio)
- [Solana Documentation](https://docs.solana.com/)
- [SPL Token Program](https://spl.solana.com/token)
