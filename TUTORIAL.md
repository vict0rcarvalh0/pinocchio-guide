# Pinocchio Tutorial
## Introduction

This tutorial demonstrates how to transform an Anchor Vault into a Pinocchio Vault on Solana. A vault is an essential construct in decentralized applications (dApps), enabling the deposit and withdrawal of tokens (fungible or non-fungible). Vaults are often foundational for mechanisms like NFT staking, token staking, and escrow systems.

### Why Pinocchio Over Anchor?

Anchor and Pinocchio serve different purposes in Solana development:

- Anchor:
    - High-level framework.
    - Simplifies program development with abstractions like PDA derivation and account handling.
    - Suitable for projects prioritizing developer efficiency.

- Pinocchio:
    - Low-level library for optimizing Compute Units (CUs), a critical resource on Solana.
    - Ideal for performance-critical programs where reducing CU costs is paramount.
    - Offers fine-grained control over program execution.

This tutorial first builds a Vault using Anchor and then transforms it into a performance-optimized Pinocchio Vault.

## Anchor Vault
### Step 1: Set Up Project Structure

Create the project by running:
```bash
anchor new vault-example
```

**Step 2: Create the folder structure to the Project**

Your programs folder structure should look like this:
```
📦programs
 ┗ 📂vault-example
 ┃ ┣ 📂src
 ┃ ┃ ┗ 📜lib.rs
 ┃ ┣ 📜Cargo.toml
 ┃ ┗ 📜Xargo.toml
```

- programs/: Contains the core Solana program code.
- tests/: Contains JavaScript/TypeScript test files.
- Anchor.toml: Anchor-specific configurations like program IDs.
- Cargo.toml: Rust package manager configuration.

### Step 2: Define Vault State
Create the directory structure for managing your vault state:
```bash
mkdir -p programs/vault_example/src/state
touch programs/vault_example/src/state/vault.rs
touch programs/vault_example/src/state/mod.rs
```

Create the `VaultState` struct to store essential state data:
```rust
use anchor_lang::prelude::*;

/// The VaultState struct holds the persistent data for our vault program.
#[account]  // Marks this struct as a persistent account for Anchor.
pub struct VaultState {
    pub vault_bump: u8,  // PDA bump for the vault
    pub state_bump: u8,  // PDA bump for the state
}

// Allocates space for the VaultState account
impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1; // 8 bytes for discriminator + 2 u8 fields
}
```

By creating that, you can also add the in the mod.rs so you can...
```rust
pub mod vault;

pub use vault::*;
```

### Step 3: Implement Initialize Instruction
The `initialize` instruction sets up the vault and its associated state.

**File Setup**
Create the directory for instructions:

```bash 
mkdir -p programs/vault_example/src/instructions
touch programs/vault_example/src/instructions/initialize.rs
```

In `initialize.rs`, implement the Initialize struct and logic:
```rust
use anchor_lang::prelude::*;
use crate::VaultState;

/// The Initialize struct defines the accounts required for the initialize instruction.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)] // Mutable account for the user who pays for initialization(lamports will be subtracted).
    pub user: Signer<'info>,

    #[account(
        init,                                  // Creates a new account.
        payer = user,                          // The user funds account creation.
        seeds = [b"state", user.key.as_ref()], // Derives the PDA for state.
        bump,                                  // Stores the bump for future references.
        space = VaultState::INIT_SPACE,        // Specifies account space.
    )]
    pub vault_state: Account<'info, VaultState>, // Persistent state account.

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()], // Derives the PDA for vault.
        bump,                                           // Stores the bump for the vault PDA.
    )]
    pub vault: SystemAccount<'info>, // The vault account (PDA).

    pub system_program: Program<'info, System>, // Required for creating accounts.
}

/// Implementation of the `initialize` method for setting up the VaultState.
impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        // Assign PDA bumps to the VaultState struct.
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}
```

### Step 4: Add Deposit and Withdraw Instructions
To handle deposits and withdrawals of lamports, implement the respective instructions in `operations.rs`:
```bash
touch programs/vault_example/src/instructions/operations.rs
```

In `operations.rs`, implement the following logic:
```rust
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::VaultState;

#[derive(Accounts)]
pub struct Operations<'info> {
    #[account(mut)] // The user who is performing the operation, marked as mutable for balance modification.
    pub user: Signer<'info>,
    #[account(
        mut,                                            // The vault account is mutable because lamports will be added to it.
        seeds = [b"vault", vault_state.key().as_ref()], // Derive the PDA using the "vault" seed and the vault_state account's key.
        bump = vault_state.vault_bump,                  // Match the bump stored in the vault state.
    )]
    pub vault: SystemAccount<'info>, // The vault PDA that holds the funds.
    #[account(
        seeds = [b"state", user.key.as_ref()], // Derive the PDA using the "state" seed and the user's key.
        bump = vault_state.state_bump,         // Match the bump stored in the vault state.
    )]
    pub vault_state: Account<'info, VaultState>, // The state account that stores metadata about the vault.
    pub system_program: Program<'info, System>, // Required for transferring lamports.
}

impl<'info> Operations<'info> {
    /// Transfers lamports from the user's account to the vault.
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // Create a reference to the System Program for executing transfers.
        let cpi_program = self.system_program.to_account_info();

        // Define the transfer operation, specifying the source (user) and destination (vault).
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        // Context for the cross-program invocation (CPI).
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Perform the lamports transfer.
        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    /// Transfers lamports from the vault back to the user's account.
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        // Reference to the System Program for executing transfers.
        let cpi_program = self.system_program.to_account_info();

        // Define the transfer operation, specifying the source (vault) and destination (user).
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        // Create the seeds for signing the transfer. The vault PDA requires its bump seed.
        let seeds = &[
            b"vault",                                          // The same "vault" seed used during initialization.
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]                     // The bump stored in the vault state.
        ];

        // Signer seeds are required for transfers involving a PDA.
        let signer_seeds = &[&seeds[..]];

        // Context for the cross-program invocation, including the signer seeds.
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Perform the lamports transfer.
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}
```

### Step 5: Close the Vault
The close function transfers all remaining lamports in the vault back to the user and closes the associated accounts. This is useful for cleanup after the vault is no longer needed.

**File Setup**
Create the close.rs file:
```bash
touch programs/vault_example/src/instructions/close.rs
```

**Close Function Implementation**
```rust
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::VaultState;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)] // The user who will receive the remaining lamports.
    pub user: Signer<'info>,
    #[account(
        mut,                                            // The vault account is mutable because lamports will be withdrawn.
        seeds = [b"vault", vault_state.key().as_ref()], // Derive the PDA using the "vault" seed and the vault_state key.
        bump = vault_state.vault_bump,                  // Match the bump stored in the vault state.
    )]
    pub vault: SystemAccount<'info>, // The vault PDA to be closed.
    #[account(
        mut,                                   // The vault_state account is mutable for closure.
        seeds = [b"state", user.key.as_ref()], // Derive the PDA using the "state" seed and the user's key.
        bump = vault_state.state_bump,         // Match the bump stored in the vault state.
        close = user                           // Send remaining lamports to the user when closing the account.
    )]
    pub vault_state: Account<'info, VaultState>, // The state account that will be closed.
    pub system_program: Program<'info, System>,  // Required for transferring lamports.
}

impl<'info> Close<'info> {
    /// Transfers all remaining lamports from the vault to the user and closes the account.
    pub fn close(&mut self) -> Result<()> {
        // Reference to the System Program for executing transfers.
        let cpi_program = self.system_program.to_account_info();

        // Define the transfer operation, moving all lamports from the vault to the user.
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        // Create the seeds for signing the transfer. The vault PDA requires its bump seed.
        let seeds = &[
            b"vault",                                          // The same "vault" seed used during initialization.
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]                     // The bump stored in the vault state.
        ];

        // Signer seeds are required for transfers involving a PDA.
        let signer_seeds = &[&seeds[..]];

        // Context for the cross-program invocation, including the signer seeds.
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Transfer all remaining lamports to the user.
        transfer(cpi_ctx, self.vault.lamports())?;

        Ok(())
    }
}   
```

### Step 6: Finalizing the Anchor Vault (lib.rs)
Now that all instruction files (initialize.rs, operations.rs, and close.rs) are implemented, the final step is to integrate these instructions into the lib.rs file. This is the entry point for the Anchor program and ties everything together.

**Update `instructions/mod.rs`**
In `instructions/mod.rs`, ensure all instruction modules are imported and made accessible for the lib.rs file.
```rust
pub mod initialize;
pub mod operations;
pub mod close;

pub use initialize::*;
pub use operations::*;
pub use close::*;
```


**Update `lib.rs`**
```rust
use anchor_lang::prelude::*;

mod state; // Import the state module containing the VaultState structure.
use state::*;

mod instructions;    // Import the instructions module.
use instructions::*; // Use all the submodules defined within `instructions/mod.rs`.

declare_id!("your default program ID generated by the anchor new command"); // Replace with your program ID.

#[program]
pub mod anchor_vault {
    use super::*;

    /// Initialize a vault for a user.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?; // Call the initialize method from the `Initialize` struct.

        Ok(())
    }

    /// Deposit lamports into the vault.
    pub fn deposit(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?; // Call the deposit method from the `Operations` struct.

        Ok(())
    }

    /// Withdraw lamports from the vault.
    pub fn withdraw(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?; // Call the withdraw method from the `Operations` struct.

        Ok(())
    }

    /// Close the vault and return remaining lamports to the user.
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?; // Call the close method from the `Close` struct.

        Ok(())
    }
}
```

### Final Project Structure
```
📦programs
 ┗ 📂vault-example
 ┃ ┣ 📂src
 ┃ ┃ ┣ 📂instructions
 ┃ ┃ ┃ ┣ 📜close.rs
 ┃ ┃ ┃ ┣ 📜initialize.rs
 ┃ ┃ ┃ ┣ 📜mod.rs
 ┃ ┃ ┃ ┗ 📜operations.rs
 ┃ ┃ ┣ 📂state
 ┃ ┃ ┃ ┣ 📜mod.rs
 ┃ ┃ ┃ ┗ 📜vault.rs
 ┃ ┃ ┗ 📜lib.rs
 ┃ ┣ 📜Cargo.toml
 ┃ ┗ 📜Xargo.toml
```

By running the `anchor build` command you should build your Anchor Vault and jump into the tests creation.

## Pinocchio Vault
Unlike Anchor, which simplifies Solana development through macros and abstractions, Pinocchio focuses on lower-level optimizations for compute units and efficiency. Writing programs in Pinocchio requires more manual setup but provides finer control over the Solana runtime.

In this tutorial, we'll implement the same vault functionality as the Anchor example, but with Pinocchio's methodology.

### Step 1: Set Up Project
First, initialize the project and add pinocchio:
```bash
cargo init --lib native-vault

cargo add pinocchio
```

### Step 2: Adjust Cargo.toml
Set the crate type type to cdylib to ensure the output is compatible with Solana's SBF loader

```toml
[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
pinocchio = "0.6.0"
```


### Step 3: Project Structure
Create the following directory structure:
```
📦src
 ┣ 📂instructions
 ┃ ┣ 📜deposit.rs
 ┃ ┣ 📜mod.rs
 ┃ ┗ 📜withdraw.rs
 ┗ 📜lib.rs
```

### Step 4: Implement process_instruction
The process_instruction function serves as the program's entry point and this process name is a common convention in native programs. It verifies the program ID, parses instruction data, and delegates the operation to the correct handler.

```rust
use instructions::VaultInstructions;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, pubkey};

mod instructions;
use instructions::*;

// Always define program IDs as constants using `pubkey!`.
// This avoids runtime costs of deriving keys dynamically.
const ID: Pubkey = pubkey!("22222222222222222222222222222222"); // Replace with your program's ID.

entrypoint!(process_instruction); // Macro that declares `process_instruction` as the program's entry point.

/// Main function to process instructions.
pub fn process_instruction(
    program_id: &Pubkey,       // Reference to the program ID.
    accounts: &[AccountInfo], // List of accounts involved in the transaction.
    data: &[u8],              // Serialized instruction data (byte array).
) -> ProgramResult {
    // Ensure the program ID matches the expected value. This prevents hijacking by another program.
    if program_id != &crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Parse the instruction discriminator and its associated data.
    // `split_first` separates the first byte (discriminator) from the rest (payload).
    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    
    let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);

    match VaultInstructions::try_from(discriminator)? {
        VaultInstructions::Deposit => todo!(),
        VaultInstructions::Withdraw => todo!(),
    }
}
```

### Step 5: Define Instruction Module
Create the instructions module in `instructions/mod.rs` to manage the vault operations and implement instruction dispatching.

```rust
pub mod deposit;  // Deposit handler.
pub mod withdraw; // Withdraw handler.

use solana_program::program_error::ProgramError;

/// Enum representing possible vault instructions.
pub enum VaultInstructions {
    Deposit,  // Instruction to deposit lamports into the vault.
    Withdraw, // Instruction to withdraw lamports from the vault.
}

/// Convert a discriminator byte into a `VaultInstructions` enum variant.
impl TryFrom<&u8> for VaultInstructions {
    type Error = ProgramError;

    fn try_from(discriminator: &u8) -> Result<Self, Self::Error> {
        match discriminator {
            0 => Ok(VaultInstructions::Deposit),
            1 => Ok(VaultInstructions::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData), // Invalid discriminator.
        }
    }
}
```

### Step 6: Implement the deposit Handler
The deposit instruction transfers lamports from the user's account to the vault.

```rust
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, program_error::ProgramError,
    pubkey::Pubkey, system_instruction::transfer,
};

/// Processes the deposit instruction.
pub fn process(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    // Validate the account array structure.
    let [signer, vault, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    // Derive the Program Derived Address (PDA) for the vault and validate it.
    let (pda, _bump) = Pubkey::try_find_program_address(&[signer.key.as_ref()], &crate::ID)
        .ok_or(ProgramError::InvalidSeeds)?;
    assert_eq!(&pda, vault.key); // Ensure the PDA matches the vault's public key.

    // Perform the transfer of lamports from the signer to the vault account.
    invoke(
        &transfer(signer.key, vault.key, lamports),
        accounts, // Pass account references required for the transfer.
    )
}
```

### Step 7: Implement the withdraw Handler
The withdraw instruction transfers lamports from the vault back to the user's account.

```rust
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, system_instruction::transfer};

/// Processes the withdraw instruction.
pub fn process(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    // Validate the account array structure.
    let [vault, signer, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    // Derive the Program Derived Address (PDA) for the vault and validate it.
    let (pda, bump) = Pubkey::try_find_program_address(&[signer.key.as_ref()], &crate::ID)
        .ok_or(ProgramError::InvalidSeeds)?;
    assert_eq!(&pda, vault.key); // Ensure the PDA matches the vault's public key.

    // Perform the transfer of lamports from the vault back to the signer account.
    invoke_signed(
        &transfer(vault.key, signer.key, lamports),
        accounts, // Pass account references required for the transfer.
        &[&[signer.key.as_ref(), &[bump]]], // Include PDA seeds for signing.
    )
}

```

### Step 8: Finalize the process_instruction Logic
Update the match statement in `lib.rs` to dispatch the correct instruction handler:
```rust
match VaultInstructions::try_from(discriminator)? {
    VaultInstructions::Deposit => deposit::process(accounts, amount),
    VaultInstructions::Withdraw => withdraw::process(accounts, amount),
}
```

build
CU comparison(todo)
