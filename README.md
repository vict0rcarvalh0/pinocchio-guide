# Pinocchio Guide

## Table of contents

## Introduction

## System Instructions

These instructions handle the creation and management of basic accounts in the Solana system. Here's what each does:

### AdvanceNonceAccount
Increments the nonce value in a Nonce account. Useful for ensuring transaction uniqueness and replay protection.

```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::AdvanceNonceAccount;

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


- Allocate: Reserves space in an existing account to store data but doesn’t initialize any content.
```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use crate::Allocate;

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

- AllocateWithSeed: Similar to Allocate, but allows deriving the account using a specific seed.
```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::AllocateWithSeed;

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

- Assign: Changes the program owner of an existing account to a new program.
```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::Assign;

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

- AssignWithSeed: Changes the program owner of a seed-derived account.
```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::AssignWithSeed;

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

- AuthorizeNonceAccount: Sets or changes the authority of a Nonce account, determining who can authorize nonce increments.
```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::AuthorizeNonceAccount;

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

- CreateAccount: Creates a new account on the blockchain and assigns it an initial balance.
```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::CreateAccount;

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

- CreateAccountWithSeed: Creates a new seed-derived account and initializes it.
```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::CreateAccountWithSeed;

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

- InitializeNonceAccount: Sets up a Nonce account to enable the use of unique, reusable numbers in transactions.
```rust
use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use crate::InitializeNonceAccount;

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

- Transfer: Transfers lamports from one account to another.

- TransferWithSeed: Transfers lamports using a seed-derived account.

- UpdateNonceAccount: Updates metadata associated with a Nonce account.

- WithdrawNonceAccount: Allows withdrawing lamports from a Nonce account to a destination account.

## Token Instructions

These instructions manage SPL token accounts and operations, which follow Solana's token standard.

- Approve: Authorizes a spender to spend a specific amount of tokens.
/// Approves a delegate.
///
/// ### Accounts:
///   0. `[WRITE]` The token account.
///   1. `[]` The delegate.
///   2. `[SIGNER]` The source account owner.
pub struct Approve<'a> {
    /// Source Account.
    pub source: &'a AccountInfo,
    /// Delegate Account
    pub delegate: &'a AccountInfo,
    /// Source Owner Account
    pub authority: &'a AccountInfo,
    /// Amount
    pub amount: u64,
}

- ApproveChecked: A safer version of Approve, which verifies the number of decimals before granting authorization.
/// Approves a delegate.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[]` The token mint.
///   2. `[]` The delegate.
///   3. `[SIGNER]` The source account owner.
pub struct ApproveChecked<'a> {
    /// Source Account.
    pub source: &'a AccountInfo,
    /// Mint Account.
    pub mint: &'a AccountInfo,
    /// Delegate Account.
    pub delegate: &'a AccountInfo,
    /// Source Owner Account.
    pub authority: &'a AccountInfo,
    /// Amount.
    pub amount: u64,
    /// Decimals.
    pub decimals: u8,
}

- Burn: Removes a specified amount of tokens from circulation, reducing the total supply.
/// Burns tokens by removing them from an account.
///
/// ### Accounts:
///   0. `[WRITE]` The account to burn from.
///   1. `[WRITE]` The token mint.
///   2. `[SIGNER]` The account's owner/delegate.
pub struct Burn<'a> {
    /// Source of the Burn Account
    pub account: &'a AccountInfo,
    /// Mint Account
    pub mint: &'a AccountInfo,
    /// Owner of the Token Account
    pub authority: &'a AccountInfo,
    /// Amount
    pub amount: u64,
}

- BurnChecked: A variant of Burn that includes decimal verification before burning tokens.
/// Burns tokens by removing them from an account.
///
/// ### Accounts:
///   0. `[WRITE]` The account to burn from.
///   1. `[WRITE]` The token mint.
///   2. `[SIGNER]` The account's owner/delegate.
pub struct BurnChecked<'a> {
    /// Source of the Burn Account
    pub account: &'a AccountInfo,
    /// Mint Account
    pub mint: &'a AccountInfo,
    /// Owner of the Token Account
    pub authority: &'a AccountInfo,
    /// Amount
    pub amount: u64,
    /// Decimals
    pub decimals: u8,
}

- CloseAccount: Closes a token account, transferring any remaining lamports to the account owner.
/// Close an account by transferring all its SOL to the destination account.
///
/// ### Accounts:
///   0. `[WRITE]` The account to close.
///   1. `[WRITE]` The destination account.
///   2. `[SIGNER]` The account's owner.
pub struct CloseAccount<'a> {
    /// Token Account.
    pub account: &'a AccountInfo,
    /// Destination Account
    pub destination: &'a AccountInfo,
    /// Owner Account
    pub authority: &'a AccountInfo,
}

- FreezeAccount: Freezes a token account, preventing any transfers until it is thawed.
/// Freeze an Initialized account using the Mint's freeze_authority
///
/// ### Accounts:
///   0. `[WRITE]` The account to freeze.
///   1. `[]` The token mint.
///   2. `[SIGNER]` The mint freeze authority.
pub struct FreezeAccount<'a> {
    /// Token Account to freeze.
    pub account: &'a AccountInfo,
    /// Mint Account.
    pub mint: &'a AccountInfo,
    /// Mint Freeze Authority Account
    pub freeze_authority: &'a AccountInfo,
}

- InitializeAccount: Initializes a token account associated with a specific wallet.
/// Initialize a new Token Account.
///
/// ### Accounts:
///   0. `[WRITE]`  The account to initialize.
///   1. `[]` The mint this account will be associated with.
///   2. `[]` The new account's owner/multisignature.
///   3. `[]` Rent sysvar
pub struct InitializeAccount<'a> {
    /// New Account.
    pub account: &'a AccountInfo,
    /// Mint Account.
    pub mint: &'a AccountInfo,
    /// Owner of the new Account.
    pub owner: &'a AccountInfo,
    /// Rent Sysvar Account
    pub rent_sysvar: &'a AccountInfo,
}

- InitializeAccount2: Similar to InitializeAccount, but directly links the account to a public key.
/// Initialize a new Token Account.
///
/// ### Accounts:
///   0. `[WRITE]`  The account to initialize.
///   1. `[]` The mint this account will be associated with.
///   3. `[]` Rent sysvar
pub struct InitializeAccount2<'a> {
    /// New Account.
    pub account: &'a AccountInfo,
    /// Mint Account.
    pub mint: &'a AccountInfo,
    /// Rent Sysvar Account
    pub rent_sysvar: &'a AccountInfo,
    /// Owner of the new Account.
    pub owner: &'a Pubkey,
}

- InitializeAccount3: An additional variant that simplifies the account initialization process further.
/// Initialize a new Token Account.
///
/// ### Accounts:
///   0. `[WRITE]`  The account to initialize.
///   1. `[]` The mint this account will be associated with.
pub struct InitializeAccount3<'a> {
    /// New Account.
    pub account: &'a AccountInfo,
    /// Mint Account.
    pub mint: &'a AccountInfo,
    /// Owner of the new Account.
    pub owner: &'a Pubkey,
}

- InitializeMint: Sets up a new Mint account for creating a new type of token.
/// Initialize a new mint.
///
/// ### Accounts:
///   0. `[WRITABLE]` Mint account
///   1. `[]` Rent sysvar
pub struct InitializeMint<'a> {
    /// Mint Account.
    pub mint: &'a AccountInfo,
    /// Rent sysvar Account.
    pub rent_sysvar: &'a AccountInfo,
    /// Decimals.
    pub decimals: u8,
    /// Mint Authority.
    pub mint_authority: &'a Pubkey,
    /// Freeze Authority.
    pub freeze_authority: Option<&'a Pubkey>,
}

- InitializeMint2: An alternative version of InitializeMint with compatibility tweaks.
/// Initialize a new mint.
///
/// ### Accounts:
///   0. `[WRITABLE]` Mint account
pub struct InitializeMint2<'a> {
    /// Mint Account.
    pub mint: &'a AccountInfo,
    /// Decimals.
    pub decimals: u8,
    /// Mint Authority.
    pub mint_authority: &'a Pubkey,
    /// Freeze Authority.
    pub freeze_authority: Option<&'a Pubkey>,
}

- MintTo: Mints new tokens and assigns them to a specific account.
/// Mints new tokens to an account.
///
/// ### Accounts:
///   0. `[WRITE]` The mint.
///   1. `[WRITE]` The account to mint tokens to.
///   2. `[SIGNER]` The mint's minting authority.
///
pub struct MintTo<'a> {
    /// Mint Account.
    pub mint: &'a AccountInfo,
    /// Token Account.
    pub account: &'a AccountInfo,
    /// Mint Authority
    pub mint_authority: &'a AccountInfo,
    /// Amount
    pub amount: u64,
}

- MintToChecked: A safer version of MintTo that verifies decimals before minting tokens.

/// Mints new tokens to an account.
///
/// ### Accounts:
///   0. `[WRITE]` The mint.
///   1. `[WRITE]` The account to mint tokens to.
///   2. `[SIGNER]` The mint's minting authority.
///
pub struct MintToChecked<'a> {
    /// Mint Account.
    pub mint: &'a AccountInfo,
    /// Token Account.
    pub account: &'a AccountInfo,
    /// Mint Authority
    pub mint_authority: &'a AccountInfo,
    /// Amount
    pub amount: u64,
    /// Decimals
    pub decimals: u8,
}

- Revoke: Revokes permissions previously granted via Approve.
/// Revokes the delegate's authority.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[SIGNER]` The source account owner.
pub struct Revoke<'a> {
    /// Source Account.
    pub source: &'a AccountInfo,
    ///  Source Owner Account.
    pub authority: &'a AccountInfo,
}

- SetAuthority: Transfers authority over a token or an account to another address.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum AuthorityType {
    MintTokens = 0,
    FreezeAccount = 1,
    AccountOwner = 2,
    CloseAccount = 3,
}

/// Sets a new authority of a mint or account.
///
/// ### Accounts:
///   0. `[WRITE]` The mint or account to change the authority of.
///   1. `[SIGNER]` The current authority of the mint or account.
pub struct SetAuthority<'a> {
    /// Account (Mint or Token)
    pub account: &'a AccountInfo,

    /// Authority of the Account.
    pub authority: &'a AccountInfo,

    /// The type of authority to update.
    pub authority_type: AuthorityType,

    /// The new authority
    pub new_authority: Option<&'a Pubkey>,
}

- SyncNative: Synchronizes the lamports balance of a wrapped SOL account with its stored value.
/// Given a native token account updates its amount field based
/// on the account's underlying `lamports`.
///
/// ### Accounts:
///   0. `[WRITE]`  The native token account to sync with its underlying
///      lamports.
pub struct SyncNative<'a> {
    /// Native Token Account
    pub native_token: &'a AccountInfo,
}

- ThawAccount: Unfreezes a previously frozen account.
/// Thaw a Frozen account using the Mint's freeze_authority
///
/// ### Accounts:
///   0. `[WRITE]` The account to thaw.
///   1. `[]` The token mint.
///   2. `[SIGNER]` The mint freeze authority.
pub struct ThawAccount<'a> {
    /// Token Account to thaw.
    pub account: &'a AccountInfo,
    /// Mint Account.
    pub mint: &'a AccountInfo,
    /// Mint Freeze Authority Account
    pub freeze_authority: &'a AccountInfo,
}

- Transfer: Transfers tokens from one account to another.
/// Transfer Tokens from one Token Account to another.
///
/// ### Accounts:
///   0. `[WRITE]` Sender account
///   1. `[WRITE]` Recipient account
///   2. `[SIGNER]` Authority account
pub struct Transfer<'a> {
    /// Sender account.
    pub from: &'a AccountInfo,
    /// Recipient account.
    pub to: &'a AccountInfo,
    /// Authority account.
    pub authority: &'a AccountInfo,
    /// Amount of microtokens to transfer.
    pub amount: u64,
}

- TransferChecked: A variant of Transfer that performs additional decimal verification.
/// Transfer Tokens from one Token Account to another.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[]` The token mint.
///   2. `[WRITE]` The destination account.
///   3. `[SIGNER]` The source account's owner/delegate.
pub struct TransferChecked<'a> {
    /// Sender account.
    pub from: &'a AccountInfo,
    /// Mint Account
    pub mint: &'a AccountInfo,
    /// Recipient account.
    pub to: &'a AccountInfo,
    /// Authority account.
    pub authority: &'a AccountInfo,
    /// Amount of microtokens to transfer.
    pub amount: u64,
    /// Decimal for the Token
    pub decimals: u8,
}

Token States

States represent persistent data associated with token accounts in the SPL token system.

- AccountState: Defines the current state of a token account, such as Initialized, Frozen, or Uninitialized.

- Mint: Represents the data of a Mint account, including decimals, total supply, and authorities.

- Token: Abstractly represents the connection between a set of Mint Accounts and Token Accounts.
