# Pinocchio Guide

## Introduction

The Pinocchio Guide serves as a comprehensive resource for understanding and implementing core account management functionalities on the Solana blockchain. It emphasizes foundational principles, code-level explanations, and best practices to help developers fully leverage Solana's unique transaction optimization system.

This documentation focuses on showing examples of instruction calls using **Pinocchio v0.10.1**, detailing its structure and usage to enhance your development experience.

## Key API Types (v0.10.x)

| Type | Description |
|------|-------------|
| `AccountView` | Zero-copy account representation (replaces `AccountInfo`) |
| `Address` | 32-byte public key type (replaces `Pubkey`) |
| `ProgramResult` | Result type for instruction processing |
| `error::ProgramError` | Program error types |
| `cpi::Signer` | Signer seeds for cross-program invocations |

### API Changes from Previous Versions

| Old API | New API |
|---------|---------|
| `account_info::AccountInfo` | `AccountView` |
| `pubkey::Pubkey` | `Address` |
| `program_error::ProgramError` | `error::ProgramError` |
| `instruction::Signer` | `cpi::Signer` |
| `account.is_signer` (field) | `account.is_signer()` (method) |
| `account.is_writable` (field) | `account.is_writable()` (method) |

## System Instructions

Pinocchio supports a range of system instructions that are essential for managing accounts and ensuring transaction security. These instructions operate at the core of Solana's blockchain to handle critical features like account creation, nonce advancement, and rent management.

### AdvanceNonceAccount

Increments the nonce value in a nonce account to ensure transaction uniqueness and replay protection. It is particularly useful for preventing double-spending or replaying transactions in Solana's deterministic environment.

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
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The Nonce account. |
/// | 1     | ✗        | ✗      | The recent blockhashes sysvar. |
/// | 2     | ✗        | ✓      | The Nonce authority. |
pub fn process_advance_nonce_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];
    let recent_blockhashes_sysvar = &accounts[1];
    let nonce_authority = &accounts[2];

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

Reserves space in an account for a specific size. Use this when an account needs to store custom data but does not require immediate funding.

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
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✓      | The account to allocate space for. |
pub fn process_allocate<'a>(
    accounts: &'a [AccountView],
    space: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let allocate_account = &accounts[0];

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

Similar to Allocate, but derives the account address using a seed and a base account. Use it to allocate accounts deterministically based on predictable inputs.

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
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The allocated account. |
/// | 1     | ✗        | ✓      | The base account used to derive the allocated account. |
pub fn process_allocate_with_seed<'a>(
    accounts: &'a [AccountView],
    seed: &str,
    space: u64,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let allocated_account = &accounts[0];
    let base_account = &accounts[1];

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

Changes the program ownership of an account. This is helpful when transferring ownership of an account to another program, for example, during a program upgrade.

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
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✓      | The account to be reassigned. |
pub fn process_assign<'a>(
    accounts: &'a [AccountView],
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let assigned_account = &accounts[0];

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

### AssignWithSeed

Like Assign, but the account address is derived using a seed and a base account. Use it when ownership needs to be transferred for deterministically derived accounts.

```rust
use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::AssignWithSeed;

/// Processes the `AssignWithSeed` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The account to be reassigned. |
/// | 1     | ✗        | ✓      | The base account used to derive the reassigned account. |
pub fn process_assign_with_seed<'a>(
    accounts: &'a [AccountView],
    base: &Address,
    seed: &str,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let assigned_account = &accounts[0];
    let base_account = &accounts[1];

    if !base_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    AssignWithSeed {
        account: assigned_account,
        base: base_account,
        seed,
        owner,
    }
    .invoke_signed(signers)
}
```

### AuthorizeNonceAccount

Updates the authority of a nonce account to a new signer. It's used when the current nonce authority needs to delegate or transfer control to another keypair.

```rust
use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::AuthorizeNonceAccount;

/// Processes the `AuthorizeNonceAccount` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The Nonce account. |
/// | 1     | ✗        | ✓      | The current Nonce authority. |
pub fn process_authorize_nonce_account<'a>(
    accounts: &'a [AccountView],
    new_authority: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];
    let nonce_authority = &accounts[1];

    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    AuthorizeNonceAccount {
        account: nonce_account,
        authority: nonce_authority,
        new_authority,
    }
    .invoke_signed(signers)
}
```

### CreateAccount

Creates a new account on the blockchain, assigns ownership to a program, and funds the account. This is a foundational instruction used whenever a new account needs to be initialized for use by a program.

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
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✓      | The funding account. |
/// | 1     | ✓        | ✓      | The new account to be created. |
pub fn process_create_account<'a>(
    accounts: &'a [AccountView],
    lamports: u64,
    space: u64,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let funding_account = &accounts[0];
    let new_account = &accounts[1];

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

### CreateAccountWithSeed

Creates a new account derived from a base account and a seed. Use this for predictable and reusable account addresses.

```rust
use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccountWithSeed;

/// Processes the `CreateAccountWithSeed` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✓      | The funding account. |
/// | 1     | ✓        | ✗      | The new account to be created. |
/// | 2     | ✗        | ✓      | The base account (optional). |
pub fn process_create_account_with_seed<'a>(
    accounts: &'a [AccountView],
    seed: &str,
    lamports: u64,
    space: u64,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let funding_account = &accounts[0];
    let new_account = &accounts[1];
    let base_account = if accounts.len() > 2 {
        Some(&accounts[2])
    } else {
        None
    };

    if !funding_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    CreateAccountWithSeed {
        from: funding_account,
        to: new_account,
        base: base_account,
        seed,
        lamports,
        space,
        owner,
    }
    .invoke_signed(signers)
}
```

### InitializeNonceAccount

Sets up an account as a nonce account, enabling it to manage nonces for transaction uniqueness. Use this when replay-protected transactions are critical.

```rust
use pinocchio::{
    AccountView,
    Address,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::InitializeNonceAccount;

/// Processes the `InitializeNonceAccount` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The Nonce account. |
/// | 1     | ✗        | ✗      | The recent blockhashes sysvar. |
/// | 2     | ✗        | ✗      | The rent sysvar. |
pub fn process_initialize_nonce_account<'a>(
    accounts: &'a [AccountView],
    authority: &Address,
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];
    let recent_blockhashes_sysvar = &accounts[1];
    let rent_sysvar = &accounts[2];

    if !nonce_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    InitializeNonceAccount {
        account: nonce_account,
        recent_blockhashes_sysvar,
        rent_sysvar,
        authority,
    }
    .invoke()
}
```

### Transfer

Moves lamports (Solana's native token) between accounts. This is one of the most common operations for basic financial transactions.

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
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✓      | The source account. |
/// | 1     | ✓        | ✗      | The destination account. |
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

### TransferWithSeed

Similar to Transfer, but supports transferring funds from an account derived using a seed and a base key. Use it for sending lamports from deterministically derived accounts.

```rust
use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::TransferWithSeed;

/// Processes the `TransferWithSeed` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The source account. |
/// | 1     | ✗        | ✓      | The base account used to derive the source account. |
/// | 2     | ✓        | ✗      | The destination account. |
pub fn process_transfer_with_seed<'a>(
    accounts: &'a [AccountView],
    seed: &str,
    lamports: u64,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let from_account = &accounts[0];
    let base_account = &accounts[1];
    let to_account = &accounts[2];

    if !from_account.is_writable() || !to_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !base_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    TransferWithSeed {
        from: from_account,
        base: base_account,
        to: to_account,
        lamports,
        seed,
        owner,
    }
    .invoke_signed(signers)
}
```

### UpgradeNonceAccount

Modifies the state of a nonce account, typically to update its stored value. Use this to maintain nonce freshness and reliability.

```rust
use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::UpgradeNonceAccount;

/// Processes the `UpgradeNonceAccount` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The Nonce account to upgrade. |
pub fn process_update_nonce_account<'a>(
    accounts: &'a [AccountView],
) -> ProgramResult {
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];

    if !nonce_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    UpgradeNonceAccount {
        account: nonce_account,
    }
    .invoke()
}
```

### WithdrawNonceAccount

Withdraws lamports from a nonce account, reducing its balance. It is typically used to recover funds or adjust the available balance.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::WithdrawNonceAccount;

/// Processes the `WithdrawNonceAccount` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The Nonce account. |
/// | 1     | ✓        | ✗      | The recipient account. |
/// | 2     | ✗        | ✗      | The recent blockhashes sysvar. |
/// | 3     | ✗        | ✗      | The rent sysvar. |
/// | 4     | ✗        | ✓      | The Nonce authority. |
pub fn process_withdraw_nonce_account<'a>(
    accounts: &'a [AccountView],
    lamports: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 5 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];
    let recipient_account = &accounts[1];
    let recent_blockhashes_sysvar = &accounts[2];
    let rent_sysvar = &accounts[3];
    let nonce_authority = &accounts[4];

    if !nonce_account.is_writable() || !recipient_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    WithdrawNonceAccount {
        account: nonce_account,
        recipient: recipient_account,
        recent_blockhashes_sysvar,
        rent_sysvar,
        authority: nonce_authority,
        lamports,
    }
    .invoke_signed(signers)
}
```

## Token Instructions

Pinocchio provides an abstraction layer for interacting with Solana Program Library (SPL) tokens, enabling seamless management of fungible and non-fungible tokens. These instructions cover the entire lifecycle of token operations.

### Approve

Grants another account the authority to transfer tokens on behalf of the current owner. Use it for scenarios where a third-party program or user needs temporary token control.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::Approve;

/// Processes the `Approve` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The token account. |
/// | 1     | ✗        | ✗      | The delegate account. |
/// | 2     | ✗        | ✓      | The source account owner. |
pub fn process_approve<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let source_account = &accounts[0];
    let delegate_account = &accounts[1];
    let authority_account = &accounts[2];

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

### ApproveChecked

Functions like Approve but with additional checks to validate the mint and token amounts. Use it for increased security and integrity in token approval.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::ApproveChecked;

/// Processes the `ApproveChecked` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The source account. |
/// | 1     | ✗        | ✗      | The token mint. |
/// | 2     | ✗        | ✗      | The delegate account. |
/// | 3     | ✗        | ✓      | The source account owner. |
pub fn process_approve_checked<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    decimals: u8,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 4 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let source_account = &accounts[0];
    let mint_account = &accounts[1];
    let delegate_account = &accounts[2];
    let authority_account = &accounts[3];

    if !source_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    ApproveChecked {
        source: source_account,
        mint: mint_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,
        decimals,
    }
    .invoke_signed(signers)
}
```

### Burn

Permanently destroys a specified number of tokens from an account, reducing the total supply. This is often used for deflationary token models or error corrections.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::Burn;

/// Processes the `Burn` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The account to burn from. |
/// | 1     | ✓        | ✗      | The token mint. |
/// | 2     | ✗        | ✓      | The account's owner/delegate. |
pub fn process_burn<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let burn_account = &accounts[0];
    let mint_account = &accounts[1];
    let authority_account = &accounts[2];

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

### BurnChecked

Similar to Burn, but with extra validation checks for the mint and token amounts. Use it when security is a priority during token destruction.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::BurnChecked;

/// Processes the `BurnChecked` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The account to burn from. |
/// | 1     | ✓        | ✗      | The token mint. |
/// | 2     | ✗        | ✓      | The account's owner/delegate. |
pub fn process_burn_checked<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    decimals: u8,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let burn_account = &accounts[0];
    let mint_account = &accounts[1];
    let authority_account = &accounts[2];

    if !burn_account.is_writable() || !mint_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    BurnChecked {
        account: burn_account,
        mint: mint_account,
        authority: authority_account,
        amount,
        decimals,
    }
    .invoke_signed(signers)
}
```

### CloseAccount

Closes a token account, reclaiming its lamports and returning them to the owner. This is used to clean up unused accounts and reduce storage costs.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::CloseAccount;

/// Processes the `CloseAccount` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The account to close. |
/// | 1     | ✓        | ✗      | The destination account. |
/// | 2     | ✗        | ✓      | The account's owner. |
pub fn process_close_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let close_account = &accounts[0];
    let destination_account = &accounts[1];
    let authority_account = &accounts[2];

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

### FreezeAccount

Freezes a token account, preventing further token transfers. This is useful for regulatory or security enforcement in token ecosystems.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::FreezeAccount;

/// Processes the `FreezeAccount` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The account to freeze. |
/// | 1     | ✗        | ✗      | The token mint. |
/// | 2     | ✗        | ✓      | The mint freeze authority. |
pub fn process_freeze_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let account_to_freeze = &accounts[0];
    let mint_account = &accounts[1];
    let freeze_authority = &accounts[2];

    if !account_to_freeze.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !freeze_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    FreezeAccount {
        account: account_to_freeze,
        mint: mint_account,
        freeze_authority,
    }
    .invoke_signed(signers)
}
```

### InitializeAccount

Sets up a token account for use with a specific token mint. This is a prerequisite for any token interaction.

```rust
use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::InitializeAccount;

/// Processes the `InitializeAccount` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The account to initialize. |
/// | 1     | ✗        | ✗      | The mint this account will be associated with. |
/// | 2     | ✗        | ✗      | The new account's owner/multisignature. |
/// | 3     | ✗        | ✗      | Rent sysvar. |
pub fn process_initialize_account<'a>(
    accounts: &'a [AccountView],
) -> ProgramResult {
    if accounts.len() < 4 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let account_to_initialize = &accounts[0];
    let mint_account = &accounts[1];
    let owner_account = &accounts[2];
    let rent_sysvar = &accounts[3];

    if !account_to_initialize.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    InitializeAccount {
        account: account_to_initialize,
        mint: mint_account,
        owner: owner_account,
        rent_sysvar,
    }
    .invoke()
}
```

### InitializeMint

Sets up a mint account, defining the token's supply and decimal precision. This is the starting point for creating new tokens.

```rust
use pinocchio::{
    AccountView,
    Address,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::InitializeMint;

/// Processes the `InitializeMint` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | Mint account. |
/// | 1     | ✗        | ✗      | Rent sysvar. |
pub fn process_initialize_mint<'a>(
    accounts: &'a [AccountView],
    decimals: u8,
    mint_authority: &Address,
    freeze_authority: Option<&Address>,
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let mint_account = &accounts[0];
    let rent_sysvar = &accounts[1];

    if !mint_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    InitializeMint {
        mint: mint_account,
        rent_sysvar,
        decimals,
        mint_authority,
        freeze_authority,
    }
    .invoke()
}
```

### MintTo

Creates new tokens and deposits them into a token account. Use it to increase the circulating supply of a token.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::MintTo;

/// Processes the `MintTo` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The mint account. |
/// | 1     | ✓        | ✗      | The account to mint tokens to. |
/// | 2     | ✗        | ✓      | The mint's minting authority. |
pub fn process_mint_to<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let mint_account = &accounts[0];
    let token_account = &accounts[1];
    let mint_authority = &accounts[2];

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

### MintToChecked

Functions like MintTo but includes additional validation to ensure minting correctness. Use it for secure token issuance.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::MintToChecked;

/// Processes the `MintToChecked` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The mint account. |
/// | 1     | ✓        | ✗      | The account to mint tokens to. |
/// | 2     | ✗        | ✓      | The mint's minting authority. |
pub fn process_mint_to_checked<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    decimals: u8,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let mint_account = &accounts[0];
    let token_account = &accounts[1];
    let mint_authority = &accounts[2];

    if !mint_account.is_writable() || !token_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !mint_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    MintToChecked {
        mint: mint_account,
        account: token_account,
        mint_authority,
        amount,
        decimals,
    }
    .invoke_signed(signers)
}
```

### Revoke

Revokes previously granted transfer authority from another account. This is used to restore exclusive control over tokens.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::Revoke;

/// Processes the `Revoke` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The source account. |
/// | 1     | ✗        | ✓      | The source account owner. |
pub fn process_revoke<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let source_account = &accounts[0];
    let owner_account = &accounts[1];

    if !source_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !owner_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Revoke {
        source: source_account,
        authority: owner_account,
    }
    .invoke_signed(signers)
}
```

### SetAuthority

Updates the authority of a mint or token account. Use this to transfer or delegate control securely.

```rust
use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::{SetAuthority, AuthorityType};

/// Processes the `SetAuthority` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The mint or account to change the authority of. |
/// | 1     | ✗        | ✓      | The current authority of the mint or account. |
pub fn process_set_authority<'a>(
    accounts: &'a [AccountView],
    authority_type: AuthorityType,
    new_authority: Option<&Address>,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let account_to_update = &accounts[0];
    let current_authority = &accounts[1];

    if !account_to_update.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !current_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    SetAuthority {
        account: account_to_update,
        authority: current_authority,
        authority_type,
        new_authority,
    }
    .invoke_signed(signers)
}
```

### SyncNative

Synchronizes a native token account's balance with its lamports. This is crucial for ensuring accuracy when working with Solana-native tokens.

```rust
use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::SyncNative;

/// Processes the `SyncNative` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The native token account to sync. |
pub fn process_sync_native<'a>(
    accounts: &'a [AccountView],
) -> ProgramResult {
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let native_token_account = &accounts[0];

    if !native_token_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    SyncNative {
        native_token: native_token_account,
    }
    .invoke()
}
```

### ThawAccount

Reverses the effects of FreezeAccount, re-enabling transfers. Use it when the conditions for freezing an account are no longer applicable.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::ThawAccount;

/// Processes the `ThawAccount` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The account to thaw. |
/// | 1     | ✗        | ✗      | The token mint. |
/// | 2     | ✗        | ✓      | The mint freeze authority. |
pub fn process_thaw_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let account_to_thaw = &accounts[0];
    let mint_account = &accounts[1];
    let freeze_authority = &accounts[2];

    if !account_to_thaw.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !freeze_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    ThawAccount {
        account: account_to_thaw,
        mint: mint_account,
        freeze_authority,
    }
    .invoke_signed(signers)
}
```

### Transfer (Token)

Moves tokens from one token account to another. This is the primary method for token transactions.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::Transfer;

/// Processes the `Transfer` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The source account. |
/// | 1     | ✓        | ✗      | The destination account. |
/// | 2     | ✗        | ✓      | The source account's owner/delegate. |
pub fn process_transfer<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let from_account = &accounts[0];
    let to_account = &accounts[1];
    let authority_account = &accounts[2];

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

Like Transfer, but with additional checks for mint and token amount validation. Use it to enforce stricter security in token transfers.

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::TransferChecked;

/// Processes the `TransferChecked` instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The source account. |
/// | 1     | ✗        | ✗      | The token mint. |
/// | 2     | ✓        | ✗      | The destination account. |
/// | 3     | ✗        | ✓      | The source account's owner/delegate. |
pub fn process_transfer_checked<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    decimals: u8,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 4 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let from_account = &accounts[0];
    let mint_account = &accounts[1];
    let to_account = &accounts[2];
    let authority_account = &accounts[3];

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

## Resources

- [Pinocchio GitHub](https://github.com/anza-xyz/pinocchio)
- [Solana Documentation](https://docs.solana.com/)
- [SPL Token Program](https://spl.solana.com/token)
