# Pinocchio Guide

## Introduction

The Pinocchio Guide is your go-to resource for understanding and implementing core account management functionalities within the Solana blockchain. This guide emphasizes foundational principles, code-level explanations, and best practices to empower developers in leveraging Solana’s unique system of transaction optimization.

Pinocchio simplifies advanced blockchain programming by offering a robust and optimized framework for system-level operations.

This documentation focuses on showing examples of instruction calls, detailing its structure and usage to enhance your development experience.

## System Instructions

Pinocchio supports a range of system instructions that are essential for managing accounts and ensuring transaction security. These instructions operate at the core of Solana's blockchain to handle critical features like account creation, nonce advancement, and rent management.

### AdvanceNonceAccount

Increments the nonce value in a nonce account to ensure transaction uniqueness and replay protection. It is particularly useful for preventing double-spending or replaying transactions in Solana's deterministic environment.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::AdvanceNonceAccount;

/// Processes the `AdvanceNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[]` The recent blockhashes sysvar.
/// 2. `[SIGNER]` The Nonce authority.
pub fn process_advance_nonce_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    signers: &[Signer],
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let nonce_account = next_account_info(account_info_iter)?;              // Account that holds the nonce value to be advanced.
    let recent_blockhashes_sysvar = next_account_info(account_info_iter)?;  // Sysvar providing recent blockhashes for the network.
    let nonce_authority = next_account_info(account_info_iter)?;            //  Signer authorized to advance the nonce.

    // Ensure the nonce authority is a signer
    if !nonce_authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let advance_nonce_instruction = AdvanceNonceAccount {
        account: nonce_account,
        recent_blockhashes_sysvar,
        authority: nonce_authority,
    };

    // Invoking the instruction
    advance_nonce_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### Allocate

Reserves space in an account for a specific size. Use this when an account needs to store custom data but does not require immediate funding.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::Allocate;

/// Processes the `Allocate` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `space`: The number of bytes to allocate.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The account to allocate space for.
pub fn process_allocate<'a>(
    accounts: &'a [AccountInfo<'a>],
    space: u64,                       // Determines how many bytes of memory are allocated for the account.
    signers: &[Signer],
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter(); // Account for which space will be allocated.

    // The account to allocate space for
    let allocate_account = next_account_info(account_info_iter)?;

    // Ensure the allocate account is a signer
    if !allocate_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let allocate_instruction = Allocate {
        account: allocate_account,
        space,
    };

    // Invoking the instruction
    allocate_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### AllocateWithSeed

Similar to Allocate, but derives the account address using a seed and a base account. Use it to allocate accounts deterministically based on predictable inputs.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::AllocateWithSeed;

/// Processes the `AllocateWithSeed` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `seed`: The seed used to derive the account's address.
/// - `space`: The number of bytes to allocate.
/// - `owner`: The program that will own the allocated account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The allocated account.
/// 1. `[SIGNER]` The base account used to derive the allocated account.
pub fn process_allocate_with_seed<'a>(
    accounts: &'a [AccountInfo<'a>],
    seed: &str,            // String used along with the base public key to derive the allocated account's address.
    space: u64,            // The number of bytes to allocate for the account.
    owner: &Pubkey,        // The program that will own the allocated account.
    signers: &[Signer],
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let allocated_account = next_account_info(account_info_iter)?; // The account being allocated with the derived address.
    let base_account = next_account_info(account_info_iter)?;      // The base account used to derive the address with the seed.

    // Ensure the base account is a signer
    if !base_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate the seed length
    if seed.len() > Pubkey::MAX_SEED_LEN {
        return Err(ProgramError::InvalidSeeds);
    }

    // Creating the instruction instance
    let allocate_with_seed_instruction = AllocateWithSeed {
        account: allocated_account,
        base: base_account,
        seed,
        space,
        owner,
    };

    // Invoking the instruction
    allocate_with_seed_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### Assign

Changes the program ownership of an account. This is helpful when transferring ownership of an account to another program, for example, during a program upgrade.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::Assign;

/// Processes the `Assign` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `owner`: The public key of the new program owner.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The account to be reassigned to a new program owner.
pub fn process_assign<'a>(
    accounts: &'a [AccountInfo<'a>],
    owner: &Pubkey,      // Public key of the program to assign as the new owner of the account.
    signers: &[Signer],
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let assigned_account = next_account_info(account_info_iter)?; // The account to be reassigned to a new program owner.

    // Ensure the assigned account is a signer
    if !assigned_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let assign_instruction = Assign {
        account: assigned_account,
        owner,
    };

    // Invoking the instruction
    assign_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### AssignWithSeed

Like Assign, but the account address is derived using a seed and a base account. Use it when ownership needs to be transferred for deterministically derived accounts.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::AssignWithSeed;

/// Processes the `AssignWithSeed` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `seed`: The seed used to derive the account.
/// - `owner`: The public key of the new program owner.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The account to be reassigned.
/// 1. `[SIGNER]` The base account used to derive the reassigned account.
pub fn process_assign_with_seed<'a>(
    accounts: &'a [AccountInfo<'a>],
    seed: &str,
    owner: &Pubkey,
    signers: &[Signer],
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let assigned_account = next_account_info(account_info_iter)?; // Account being reassigned to a program owner based on the derived address.
    let base_account = next_account_info(account_info_iter)?;     // Base account used to derive the assigned account’s address with the seed.

    // Ensure the base account is a signer
    if !base_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate the seed length
    if seed.len() > Pubkey::MAX_SEED_LEN {
        return Err(ProgramError::InvalidSeeds);
    }

    // Creating the instruction instance
    let assign_with_seed_instruction = AssignWithSeed {
        account: assigned_account,
        base: base_account,
        seed,
        owner,
    };

    // Invoking the instruction
    assign_with_seed_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### AuthorizeNonceAccount

Updates the authority of a nonce account to a new signer. It’s used when the current nonce authority needs to delegate or transfer control to another keypair.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::AuthorizeNonceAccount;

/// Processes the `AuthorizeNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `new_authority`: The public key of the new authority.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[SIGNER]` The current Nonce authority.
pub fn process_authorize_nonce_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    new_authority: &Pubkey,  // Pubkey of the new entity to be authorized to execute nonce instructions on the account.
    signers: &[Signer],
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let nonce_account = next_account_info(account_info_iter)?;   // Nonce account whose authority will be changed.
    let nonce_authority = next_account_info(account_info_iter)?; // Current authority of the nonce account that will authorize the change.

    // Ensure the nonce authority is a signer
    if !nonce_authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let authorize_nonce_instruction = AuthorizeNonceAccount {
        account: nonce_account,
        authority: nonce_authority,
        new_authority,
    };

    // Invoking the instruction
    authorize_nonce_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### CreateAccount

Creates a new account on the blockchain, assigns ownership to a program, and funds the account. This is a foundational instruction used whenever a new account needs to be initialized for use by a program.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::CreateAccount;

/// Processes the `CreateAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer to the new account.
/// - `space`: The number of bytes to allocate for the new account.
/// - `owner`: The program that will own the new account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The funding account.
/// 1. `[WRITE, SIGNER]` The new account to be created.
pub fn process_create_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    lamports: u64,   // Number of lamports to transfer to the new account.
    space: u64,      // Number of bytes to allocate for the new account.
    owner: &Pubkey,  // Pubkey of the program that will own the new account.
    signers: &[Signer],
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let funding_account = next_account_info(account_info_iter)?; // The account that will fund the new account.
    let new_account = next_account_info(account_info_iter)?;     // The new account that will be created.

    // Ensure the funding account and new account are signers
    if !funding_account.is_signer || !new_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let create_account_instruction = CreateAccount {
        from: funding_account,
        to: new_account,
        lamports,
        space,
        owner,
    };

    // Invoking the instruction
    create_account_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### CreateAccountWithSeed

Creates a new account derived from a base account and a seed. Use this for predictable and reusable account addresses.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::CreateAccountWithSeed;

/// Processes the `CreateAccountWithSeed` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `seed`: The seed used to derive the account.
/// - `lamports`: The number of lamports to transfer to the new account.
/// - `space`: The number of bytes to allocate for the new account.
/// - `owner`: The program that will own the new account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The funding account.
/// 1. `[WRITE, SIGNER]` The new account to be created.
/// 2. `[OPTIONAL]` The base account used to derive the new account (if applicable).
pub fn process_create_account_with_seed<'a>(
    accounts: &'a [AccountInfo<'a>],
    seed: &'a str,      // The ASCII string that will be used as the seed to derive the address.
    lamports: u64,      // Number of lamports to transfer to the new account.
    space: u64,         // Number of bytes to allocate for the new account.
    owner: &Pubkey,     // Pubkey of the program that will own the new account.
    signers: &[Signer],
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let funding_account = next_account_info(account_info_iter)?;  // The account that will fund the new account.
    let new_account = next_account_info(account_info_iter)?;      // The new account that will be created.
    let base_account = next_account_info(account_info_iter).ok(); // The optional base account used to derive the address for the new account. If not provided, the funding_account will be used.

    // Ensure that funding account and new account are signers
    if !funding_account.is_signer || !new_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let create_account_with_seed_instruction = CreateAccountWithSeed {
        from: funding_account,
        to: new_account,
        base: base_account,
        seed,
        lamports,
        space,
        owner,
    };

    // Invoking the instruction
    create_account_with_seed_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### InitializeNonceAccount

Sets up an account as a nonce account, enabling it to manage nonces for transaction uniqueness. Use this when replay-protected transactions are critical.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::InitializeNonceAccount;

/// Processes the `InitializeNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `authority`: The public key of the entity authorized to manage the Nonce account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[]` The recent blockhashes sysvar.
/// 2. `[]` The rent sysvar.
pub fn process_initialize_nonce_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    authority: &'a Pubkey,   // Pubkey representing the entity authorized to interact with the nonce account.
    signers: &[Signer],      // Signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let nonce_account = next_account_info(account_info_iter)?;                // Nonce account to be initialized.
    let recent_blockhashes_sysvar = next_account_info(account_info_iter)?;    // System variable containing recent blockhashes.
    let rent_sysvar = next_account_info(account_info_iter)?;                  // System variable providing rent information.

    // Ensure that nonce account is writable
    if !nonce_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Creating the instruction instance
    let initialize_nonce_account_instruction = InitializeNonceAccount {
        account: nonce_account,
        recent_blockhashes_sysvar,
        rent_sysvar,
        authority,
    };

    // Invoking the instruction
    initialize_nonce_account_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### Transfer

Moves lamports (Solana's native token) between accounts. This is one of the most common operations for basic financial transactions.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::Transfer;

/// Processes the `Transfer` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The source account.
/// 1. `[WRITE]` The destination account.
pub fn process_transfer<'a>(
    accounts: &'a [AccountInfo<'a>],
    lamports: u64,        // The amount of lamports to transfer.
    signers: &[Signer],   // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let from_account = next_account_info(account_info_iter)?; // The funding account from which lamports will be transferred.
    let to_account = next_account_info(account_info_iter)?;   // The recipient account that will receive the lamports.

    // Ensure that the 'from' account is writable and a signer
    if !from_account.is_writable || !from_account.is_signer {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'to' account is writable
    if !to_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Creating the instruction instance
    let transfer_instruction = Transfer {
        from: from_account,
        to: to_account,
        lamports,
    };

    // Invoking the instruction
    transfer_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### TransferWithSeed

Similar to Transfer, but supports transferring funds from an account derived using a seed and a base key. Use it for sending lamports from deterministically derived accounts.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::TransferWithSeed;

/// Processes the `TransferWithSeed` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer.
/// - `seed`: The seed used to derive the source account.
/// - `owner`: The program that owns the source account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The source account.
/// 1. `[SIGNER]` The base account used to derive the source account.
/// 2. `[WRITE]` The destination account.
pub fn process_transfer_with_seed<'a>(
    accounts: &'a [AccountInfo<'a>],
    lamports: u64,        //  The amount of lamports to transfer.
    seed: &'a str,        // The seed used to derive the address of the funding account.
    owner: &'a Pubkey,    // The address of the program that will own the new account.
    signers: &[Signer],   // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let from_account = next_account_info(account_info_iter)?; // The funding account from which lamports will be transferred.
    let base_account = next_account_info(account_info_iter)?; // The base account used to derive the funding account's address. This must be a signer.
    let to_account = next_account_info(account_info_iter)?;   // The recipient account that will receive the lamports.

    // Ensure that the 'from' account is writable
    if !from_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'base' account is a signer
    if !base_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure that the 'to' account is writable
    if !to_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Creating the instruction instance
    let transfer_instruction = TransferWithSeed {
        from: from_account,
        base: base_account,
        to: to_account,
        lamports,
        seed,
        owner,
    };

    // Invoking the instruction
    transfer_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### UpdateNonceAccount

Modifies the state of a nonce account, typically to update its stored value. Use this to maintain nonce freshness and reliability.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::UpdateNonceAccount;

/// Processes the `UpdateNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
pub fn process_update_nonce_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    signers: &[Signer],  // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // The nonce account passed to the instruction
    let nonce_account = next_account_info(account_info_iter)?; // The account that needs to be upgraded

    // Ensure that the 'nonce_account' is writable
    if !nonce_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Creating the instruction instance
    let update_nonce_instruction = UpdateNonceAccount {
        account: nonce_account,
    };

    // Invoking the instruction
    update_nonce_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### WithdrawNonceAccount

Withdraws lamports from a nonce account, reducing its balance. It is typically used to recover funds or adjust the available balance.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::WithdrawNonceAccount;

/// Processes the `WithdrawNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports_to_withdraw`: The number of lamports to withdraw.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[WRITE]` The recipient account.
/// 2. `[]` The recent blockhashes sysvar.
/// 3. `[]` The rent sysvar.
/// 4. `[SIGNER]` The Nonce authority.
pub fn process_withdraw_nonce_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    signers: &[Signer],          // The signers array required to authorize the transaction.
    lamports_to_withdraw: u64,   // The amount of lamports to withdraw.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Nonce account
    let nonce_account = next_account_info(account_info_iter)?;               // The account from which lamports will be withdrawn.

    // Recipient account
    let recipient_account = next_account_info(account_info_iter)?;           // The account where the withdrawn lamports will be sent.

    // RecentBlockhashes sysvar
    let recent_blockhashes_sysvar = next_account_info(account_info_iter)?;   // A sysvar account providing recent blockhashes.

    // Rent sysvar
    let rent_sysvar = next_account_info(account_info_iter)?;                 // A sysvar account providing rent information.

    // Nonce authority
    let nonce_authority = next_account_info(account_info_iter)?;             // The account that is authorized to execute the withdrawal.

    // Ensure the necessary accounts are writable or readonly as required
    if !nonce_account.is_writable || !recipient_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the nonce authority is a signer
    if !nonce_authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let withdraw_nonce_instruction = WithdrawNonceAccount {
        account: nonce_account,
        recipient: recipient_account,
        recent_blockhashes_sysvar,
        rent_sysvar,
        authority: nonce_authority,
        lamports: lamports_to_withdraw,
    };

    // Invoking the instruction
    withdraw_nonce_instruction.invoke_signed(signers)?;

    Ok(())
}
```

## Token Instructions

Pinocchio provides an abstraction layer for interacting with Solana Program Library(SPL) tokens, enabling seamless management of fungible and non-fungible tokens. These instructions cover the entire lifecycle of token operations.

### Approve

Grants another account the authority to transfer tokens on behalf of the current owner. Use it for scenarios where a third-party program or user needs temporary token control.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::Approve;

/// Processes the Approve instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to approve.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The token account.
///   1. `[]` The delegate account.
///   2. `[SIGNER]` The source account owner.
pub fn process_approve<'a>(
    accounts: &'a [AccountInfo<'a>],
    amount: u64,         // Amount of tokens to approve.
    signers: &[Signer],  // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let source_account = next_account_info(account_info_iter)?; // The token account.
    let delegate_account = next_account_info(account_info_iter)?; // The delegate account.
    let authority_account = next_account_info(account_info_iter)?; // The source account owner.

    // Ensure that the 'source' account is writable
    if !source_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'authority' account is a signer
    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let approve_instruction = Approve {
        source: source_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,
    };

    // Invoking the instruction
    approve_instruction.invoke_signed(signers)?;

    Ok(())
}

```

### ApproveChecked

Functions like Approve but with additional checks to validate the mint and token amounts. Use it for increased security and integrity in token approval.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::ApproveChecked;

/// Processes the ApproveChecked instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to approve.
/// - `decimals`: The number of decimals for the token.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[]` The token mint.
///   2. `[]` The delegate account.
///   3. `[SIGNER]` The source account owner.
pub fn process_approve_checked<'a>(
    accounts: &'a [AccountInfo<'a>],
    amount: u64,         // Amount of tokens to approve.
    decimals: u8,        // Token decimals for validation.
    signers: &[Signer],  // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let source_account = next_account_info(account_info_iter)?; // The source account.
    let mint_account = next_account_info(account_info_iter)?;   // The token mint account.
    let delegate_account = next_account_info(account_info_iter)?; // The delegate account.
    let authority_account = next_account_info(account_info_iter)?; // The source account owner.

    // Ensure that the 'source' account is writable
    if !source_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'authority' account is a signer
    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let approve_checked_instruction = ApproveChecked {
        source: source_account,
        mint: mint_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,
        decimals,
    };

    // Invoking the instruction
    approve_checked_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### Burn

Permanently destroys a specified number of tokens from an account, reducing the total supply. This is often used for deflationary token models or error corrections.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::Burn;

/// Processes the Burn instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to burn.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to burn from.
///   1. `[WRITE]` The token mint.
///   2. `[SIGNER]` The account's owner/delegate.
pub fn process_burn<'a>(
    accounts: &'a [AccountInfo<'a>],
    amount: u64,         // Amount of tokens to burn.
    signers: &[Signer],  // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let burn_account = next_account_info(account_info_iter)?;     // The account to burn from.
    let mint_account = next_account_info(account_info_iter)?;     // The token mint account.
    let authority_account = next_account_info(account_info_iter)?; // The account owner or delegate.

    // Ensure that the 'burn' account is writable
    if !burn_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'mint' account is writable
    if !mint_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'authority' account is a signer
    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let burn_instruction = Burn {
        account: burn_account,
        mint: mint_account,
        authority: authority_account,
        amount,
    };

    // Invoking the instruction
    burn_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### BurnChecked

Similar to Burn, but with extra validation checks for the mint and token amounts. Use it when security is a priority during token destruction.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::BurnChecked;

/// Processes the BurnChecked instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to burn.
/// - `decimals`: The decimals for the token being burned.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to burn from.
///   1. `[WRITE]` The token mint.
///   2. `[SIGNER]` The account's owner/delegate.
pub fn process_burn_checked<'a>(
    accounts: &'a [AccountInfo<'a>],
    amount: u64,         // Amount of tokens to burn.
    decimals: u8,        // Number of decimals for the token.
    signers: &[Signer],  // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let burn_account = next_account_info(account_info_iter)?;     // The account to burn from.
    let mint_account = next_account_info(account_info_iter)?;     // The token mint account.
    let authority_account = next_account_info(account_info_iter)?; // The account owner or delegate.

    // Ensure that the 'burn' account is writable
    if !burn_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'mint' account is writable
    if !mint_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'authority' account is a signer
    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let burn_checked_instruction = BurnChecked {
        account: burn_account,
        mint: mint_account,
        authority: authority_account,
        amount,
        decimals,
    };

    // Invoking the instruction
    burn_checked_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### CloseAccount

Closes a token account, reclaiming its lamports and returning them to the owner. This is used to clean up unused accounts and reduce storage costs.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::CloseAccount;

/// Processes the CloseAccount instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to close.
///   1. `[WRITE]` The destination account.
///   2. `[SIGNER]` The account's owner.
pub fn process_close_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let close_account = next_account_info(account_info_iter)?;      // The account to close.
    let destination_account = next_account_info(account_info_iter)?; // The destination account.
    let authority_account = next_account_info(account_info_iter)?;  // The owner of the account to close.

    // Ensure that the 'close' account is writable
    if !close_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'destination' account is writable
    if !destination_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'authority' account is a signer
    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let close_account_instruction = CloseAccount {
        account: close_account,
        destination: destination_account,
        authority: authority_account,
    };

    // Invoking the instruction
    close_account_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### FreezeAccount

Freezes a token account, preventing further token transfers. This is useful for regulatory or security enforcement in token ecosystems.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::FreezeAccount;

/// Processes the FreezeAccount instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to freeze.
///   1. `[]` The token mint.
///   2. `[SIGNER]` The mint freeze authority.
pub fn process_freeze_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let account_to_freeze = next_account_info(account_info_iter)?; // The account to freeze.
    let mint_account = next_account_info(account_info_iter)?;      // The token mint account.
    let freeze_authority = next_account_info(account_info_iter)?;  // The mint freeze authority account.

    // Ensure that the account to freeze is writable
    if !account_to_freeze.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the freeze authority is a signer
    if !freeze_authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let freeze_account_instruction = FreezeAccount {
        account: account_to_freeze,
        mint: mint_account,
        freeze_authority,
    };

    // Invoking the instruction
    freeze_account_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### InitializeAccount

Sets up a token account for use with a specific token mint. This is a prerequisite for any token interaction.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::InitializeAccount;

/// Processes the InitializeAccount instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]`  The account to initialize.
///   1. `[]` The mint this account will be associated with.
///   2. `[]` The new account's owner/multisignature.
///   3. `[]` Rent sysvar.
pub fn process_initialize_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let account_to_initialize = next_account_info(account_info_iter)?; // The account to initialize.
    let mint_account = next_account_info(account_info_iter)?;          // The mint associated with the account.
    let owner_account = next_account_info(account_info_iter)?;         // The new account's owner/multisignature.
    let rent_sysvar = next_account_info(account_info_iter)?;           // Rent sysvar account.

    // Ensure that the account to initialize is writable
    if !account_to_initialize.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the rent sysvar is valid (you might need additional checks here)
    if rent_sysvar.key != &solana_program::sysvar::rent::ID {
        return Err(ProgramError::InvalidAccountData);
    }

    // Creating the instruction instance
    let initialize_account_instruction = InitializeAccount {
        account: account_to_initialize,
        mint: mint_account,
        owner: owner_account,
        rent_sysvar,
    };

    // Invoking the instruction
    initialize_account_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### InitializeMint

Sets up a mint account, defining the token's supply and decimal precision. This is the starting point for creating new tokens.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::InitializeMint;

/// Processes the InitializeMint instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `decimals`: Number of decimals for the token.
/// - `mint_authority`: The public key of the mint authority.
/// - `freeze_authority`: An optional public key for the freeze authority.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITABLE]` Mint account.
///   1. `[]` Rent sysvar.
pub fn process_initialize_mint<'a>(
    accounts: &'a [AccountInfo<'a>],
    decimals: u8,                   // Decimals for the mint.
    mint_authority: &Pubkey,        // Public key of the mint authority.
    freeze_authority: Option<&Pubkey>, // Optional public key of the freeze authority.
    signers: &[Signer],             // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let mint_account = next_account_info(account_info_iter)?; // The mint account.
    let rent_sysvar = next_account_info(account_info_iter)?;  // Rent sysvar account.

    // Ensure the mint account is writable
    if !mint_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the rent sysvar is valid (you might need additional checks here)
    if rent_sysvar.key != &solana_program::sysvar::rent::ID {
        return Err(ProgramError::InvalidAccountData);
    }

    // Creating the instruction instance
    let initialize_mint_instruction = InitializeMint {
        mint: mint_account,
        rent_sysvar,
        decimals,
        mint_authority,
        freeze_authority,
    };

    // Invoking the instruction
    initialize_mint_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### MintTo

Creates new tokens and deposits them into a token account. Use it to increase the circulating supply of a token.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::Signer,
    pubkey::Pubkey,
    program_error::ProgramError,
};

use crate::MintTo;

/// Processes the MintTo instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to mint.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The mint account.
///   1. `[WRITE]` The account to mint tokens to.
///   2. `[SIGNER]` The mint's minting authority.
pub fn process_mint_to<'a>(
    accounts: &'a [AccountInfo<'a>],
    amount: u64,            // Amount of tokens to mint.
    signers: &[Signer],     // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let mint_account = next_account_info(account_info_iter)?; // The mint account.
    let token_account = next_account_info(account_info_iter)?; // The recipient token account.
    let mint_authority = next_account_info(account_info_iter)?; // The mint authority account.

    // Ensure the mint account is writable
    if !mint_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the token account is writable
    if !token_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the mint authority is a signer
    if !mint_authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let mint_to_instruction = MintTo {
        mint: mint_account,
        account: token_account,
        mint_authority,
        amount,
    };

    // Invoking the instruction
    mint_to_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### MintToChecked

Functions like MintTo but includes additional validation to ensure minting correctness. Use it for secure token issuance.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::Signer,
    pubkey::Pubkey,
    program_error::ProgramError,
};

use crate::MintToChecked;

/// Processes the MintToChecked instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to mint.
/// - `decimals`: The number of decimal places for the tokens.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The mint account.
///   1. `[WRITE]` The account to mint tokens to.
///   2. `[SIGNER]` The mint's minting authority.
pub fn process_mint_to_checked<'a>(
    accounts: &'a [AccountInfo<'a>],
    amount: u64,            // Amount of tokens to mint.
    decimals: u8,           // Number of decimal places.
    signers: &[Signer],     // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let mint_account = next_account_info(account_info_iter)?; // The mint account.
    let token_account = next_account_info(account_info_iter)?; // The recipient token account.
    let mint_authority = next_account_info(account_info_iter)?; // The mint authority account.

    // Ensure the mint account is writable
    if !mint_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the token account is writable
    if !token_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the mint authority is a signer
    if !mint_authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let mint_to_checked_instruction = MintToChecked {
        mint: mint_account,
        account: token_account,
        mint_authority,
        amount,
        decimals,
    };

    // Invoking the instruction
    mint_to_checked_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### Revoke

Revokes previously granted transfer authority from another account. This is used to restore exclusive control over tokens.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::Signer,
    program_error::ProgramError,
};

use crate::Revoke;

/// Processes the Revoke instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[SIGNER]` The source account owner.
pub fn process_revoke<'a>(
    accounts: &'a [AccountInfo<'a>],
    signers: &[Signer], // The signers array for authorization.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let source_account = next_account_info(account_info_iter)?; // Source account to revoke delegate.
    let owner_account = next_account_info(account_info_iter)?; // Owner of the source account.

    // Ensure the source account is writable
    if !source_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the owner account is a signer
    if !owner_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let revoke_instruction = Revoke {
        source: source_account,
        authority: owner_account,
    };

    // Invoking the instruction
    revoke_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### SetAuthority

Updates the authority of a mint or token account. Use this to transfer or delegate control securely.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::Signer,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{SetAuthority, AuthorityType};

/// Processes the SetAuthority instruction.
///
/// ### Accounts:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The mint or account to change the authority of.
///   1. `[SIGNER]` The current authority of the mint or account.
pub fn process_set_authority<'a>(
    accounts: &'a [AccountInfo<'a>],
    authority_type: AuthorityType,
    new_authority: Option<&Pubkey>, // Optional new authority
    signers: &[Signer],
) -> ProgramResult {
    // Extract account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let account_to_update = next_account_info(account_info_iter)?; // The account to update.
    let current_authority = next_account_info(account_info_iter)?; // Current authority of the account.

    // Ensure the account to update is writable
    if !account_to_update.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the current authority account is a signer
    if !current_authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Create the instruction instance
    let set_authority_instruction = SetAuthority {
        account: account_to_update,
        authority: current_authority,
        authority_type,
        new_authority,
    };

    // Invoke the instruction
    set_authority_instruction.invoke_signed(signers)?;

    Ok(())
}
```

### SyncNative

Synchronizes a native token account’s balance with its lamports. This is crucial for ensuring accuracy when working with Solana-native tokens.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::SyncNative;

/// Process the SyncNative instruction.
///
/// ### Parameters:
/// - `accounts`: List of the accounts involved in the instruction..
///
/// ### Accounts:
///   0. `[WRITE]` The native token account to be syncronized with the subjacent lamports.
pub fn process_sync_native<'a>(
    accounts: &'a [AccountInfo<'a>],
    program_id: &Pubkey,
) -> ProgramResult {
    // Iterate over the provided accounts
    let account_info_iter = &mut accounts.iter();

    // The account to be syncronized
    let native_token_account = next_account_info(account_info_iter)?;

    // Validate if the account is writable
    if !native_token_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate if the account is owned by the program
    if native_token_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Construct the SyncNative instruction
    let sync_native_instruction = SyncNative {
        native_token: native_token_account,
    };

    // Invoke the instruction
    sync_native_instruction.invoke()
}
```

### ThawAccount

Reverses the effects of FreezeAccount, re-enabling transfers. Use it when the conditions for freezing an account are no longer applicable.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::ThawAccount;

/// Processes the ThawAccount instruction.
///
/// ### Parameters:
/// - `accounts`: List of accounts involved in the instruction.
///
/// ### Accounts:
///   0. `[WRITE]` The token account to be thawed.
///   1. `[]` The token mint associated with the account.
///   2. `[SIGNER]` The freeze authority for the mint.
pub fn process_thaw_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    program_id: &Pubkey,
) -> ProgramResult {
    // Iterate over the provided accounts
    let account_info_iter = &mut accounts.iter();

    // The account to thaw
    let token_account = next_account_info(account_info_iter)?;

    // The associated mint account
    let mint_account = next_account_info(account_info_iter)?;

    // The freeze authority account
    let freeze_authority_account = next_account_info(account_info_iter)?;

    // Validate that the token account is writable
    if !token_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate the token account is owned by the current program
    if token_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate the mint account
    if mint_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate the freeze authority is a signer
    if !freeze_authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the ThawAccount instruction
    let thaw_account_instruction = ThawAccount {
        account: token_account,
        mint: mint_account,
        freeze_authority: freeze_authority_account,
    };

    // Invoke the instruction
    thaw_account_instruction.invoke()
}
```

### Transfer

Moves tokens from one token account to another. This is the primary method for token transactions.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::Transfer;

/// Processes the Transfer instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts involved in the transfer.
/// - `amount`: The amount of tokens to transfer.
/// - `program_id`: The ID of the current program.
///
/// ### Accounts:
///   0. `[WRITE]` The sender account.
///   1. `[WRITE]` The recipient account.
///   2. `[SIGNER]` The authority that approves the transfer.
pub fn process_transfer<'a>(
    accounts: &'a [AccountInfo<'a>],
    amount: u64,
    program_id: &Pubkey,
) -> ProgramResult {
    // Iterate over the provided accounts
    let account_info_iter = &mut accounts.iter();

    // The sender account
    let sender_account = next_account_info(account_info_iter)?;

    // The recipient account
    let recipient_account = next_account_info(account_info_iter)?;

    // The authority account
    let authority_account = next_account_info(account_info_iter)?;

    // Validate that the sender account is writable
    if !sender_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate that the recipient account is writable
    if !recipient_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate the sender and recipient accounts are owned by the program
    if sender_account.owner != program_id || recipient_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate the authority is a signer
    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the Transfer instruction
    let transfer_instruction = Transfer {
        from: sender_account,
        to: recipient_account,
        authority: authority_account,
        amount,
    };

    // Invoke the instruction
    transfer_instruction.invoke()
```

### TransferChecked

Like Transfer, but with additional checks for mint and token amount validation. Use it to enforce stricter security in token transfers.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::TransferChecked;

/// Processes the TransferChecked instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to transfer (in microtokens).
/// - `decimals`: The number of decimal places for the token.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[]` The token mint.
///   2. `[WRITE]` The destination account.
///   3. `[SIGNER]` The source account's owner/delegate.
pub fn process_transfer_checked<'a>(
    accounts: &'a [AccountInfo<'a>],
    amount: u64,        // The amount of tokens to transfer.
    decimals: u8,       // The number of decimals for the token.
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let account_info_iter = &mut accounts.iter();

    // Accounts passed to the instruction
    let from_account = next_account_info(account_info_iter)?; // The source account.
    let mint_account = next_account_info(account_info_iter)?; // The token mint account.
    let to_account = next_account_info(account_info_iter)?;   // The destination account.
    let authority_account = next_account_info(account_info_iter)?; // The source account's owner/delegate.

    // Ensure the 'from' account is writable
    if !from_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the 'to' account is writable
    if !to_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the authority account is a signer
    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let transfer_checked_instruction = TransferChecked {
        from: from_account,
        mint: mint_account,
        to: to_account,
        authority: authority_account,
        amount,
        decimals,
    };

    // Invoking the instruction
    transfer_checked_instruction.invoke_signed(signers)?;

    Ok(())
}
```
