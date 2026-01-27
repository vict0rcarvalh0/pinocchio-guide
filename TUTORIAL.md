# Pinocchio Tutorial

## Introduction

This tutorial demonstrates how to transform an Anchor Vault into a Pinocchio Vault on Solana. A vault is an essential construct in decentralized applications (dApps), enabling the deposit and withdrawal of tokens (fungible or non-fungible). Vaults are often foundational for mechanisms like NFT staking, token staking, and escrow systems.

### Why Pinocchio Over Anchor?

Anchor and Pinocchio serve different purposes in Solana development:

- **Anchor:**
    - High-level framework
    - Simplifies program development with abstractions like PDA derivation and account handling
    - Suitable for projects prioritizing developer efficiency

- **Pinocchio:**
    - Zero-dependency library for optimizing Compute Units (CUs)
    - Ideal for performance-critical programs where reducing CU costs is paramount
    - Offers fine-grained control over program execution

This tutorial first builds a Vault using Anchor and then transforms it into a performance-optimized Pinocchio Vault.

---

## Part 1: Anchor Vault

### Step 1: Set Up Project Structure

Create the project by running:

```bash
anchor init vault-example
```

Your programs folder structure should look like this:

```
📦programs
 ┗ 📂vault-example
 ┃ ┣ 📂src
 ┃ ┃ ┗ 📜lib.rs
 ┃ ┣ 📜Cargo.toml
 ┃ ┗ 📜Xargo.toml
```

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
#[account]
pub struct VaultState {
    pub vault_bump: u8,  // PDA bump for the vault
    pub state_bump: u8,  // PDA bump for the state
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1; // 8 bytes discriminator + 2 u8 fields
}
```

Update `mod.rs`:

```rust
pub mod vault;
pub use vault::*;
```

### Step 3: Implement Initialize Instruction

Create the directory for instructions:

```bash
mkdir -p programs/vault_example/src/instructions
touch programs/vault_example/src/instructions/initialize.rs
```

In `initialize.rs`:

```rust
use anchor_lang::prelude::*;
use crate::VaultState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key.as_ref()],
        bump,
        space = VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        Ok(())
    }
}
```

### Step 4: Add Deposit and Withdraw Instructions

Create `operations.rs`:

```bash
touch programs/vault_example/src/instructions/operations.rs
```

```rust
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::VaultState;

#[derive(Accounts)]
pub struct Operations<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    
    #[account(
        seeds = [b"state", user.key.as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> Operations<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
```

### Step 5: Close the Vault

Create `close.rs`:

```bash
touch programs/vault_example/src/instructions/close.rs
```

```rust
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::VaultState;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"state", user.key.as_ref()],
        bump = vault_state.state_bump,
        close = user
    )]
    pub vault_state: Account<'info, VaultState>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, self.vault.lamports())?;
        Ok(())
    }
}
```

### Step 6: Finalize the Anchor Vault

Update `instructions/mod.rs`:

```rust
pub mod initialize;
pub mod operations;
pub mod close;

pub use initialize::*;
pub use operations::*;
pub use close::*;
```

Update `lib.rs`:

```rust
use anchor_lang::prelude::*;

mod state;
use state::*;

mod instructions;
use instructions::*;

declare_id!("YOUR_PROGRAM_ID_HERE");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}
```

Build with `anchor build`.

---

## Part 2: Pinocchio Vault

Unlike Anchor, Pinocchio focuses on lower-level optimizations for compute units and efficiency. Writing programs in Pinocchio requires more manual setup but provides finer control over the Solana runtime.

### Step 1: Set Up Project

```bash
cargo init --lib pinocchio-vault
cd pinocchio-vault
```

### Step 2: Configure Cargo.toml

```toml
[package]
name = "pinocchio-vault"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
pinocchio = { version = "0.10.1", features = ["cpi"] }
pinocchio-system = "0.5.0"
```

### Step 3: Project Structure

```
📦src
 ┣ 📂instructions
 ┃ ┣ 📜deposit.rs
 ┃ ┣ 📜withdraw.rs
 ┃ ┗ 📜mod.rs
 ┗ 📜lib.rs
```

### Step 4: Implement Entry Point (lib.rs)

```rust
use pinocchio::{
    entrypoint,
    error::ProgramError,
    AccountView,
    Address,
    ProgramResult,
};

mod instructions;
use instructions::VaultInstruction;

// Replace with your actual program ID
const PROGRAM_ID: [u8; 32] = [0u8; 32];

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    data: &[u8],
) -> ProgramResult {
    // Verify program ID
    if program_id.as_ref() != &PROGRAM_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Parse discriminator
    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    // Parse amount (8 bytes, little-endian)
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = u64::from_le_bytes(data[..8].try_into().unwrap());

    match VaultInstruction::try_from(*discriminator)? {
        VaultInstruction::Deposit => instructions::deposit::process(accounts, amount),
        VaultInstruction::Withdraw => instructions::withdraw::process(accounts, amount),
    }
}
```

### Step 5: Define Instruction Module (instructions/mod.rs)

```rust
pub mod deposit;
pub mod withdraw;

use pinocchio::error::ProgramError;

pub enum VaultInstruction {
    Deposit,
    Withdraw,
}

impl TryFrom<u8> for VaultInstruction {
    type Error = ProgramError;

    fn try_from(discriminator: u8) -> Result<Self, Self::Error> {
        match discriminator {
            0 => Ok(VaultInstruction::Deposit),
            1 => Ok(VaultInstruction::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
```

### Step 6: Implement Deposit (instructions/deposit.rs)

```rust
use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

/// Processes the deposit instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✓      | The signer/depositor account |
/// | 1     | ✓        | ✗      | The vault PDA |
pub fn process(accounts: &[AccountView], lamports: u64) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    
    let signer = &accounts[0];
    let vault = &accounts[1];

    // Validate signer
    if !signer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate accounts are writable
    if !signer.is_writable() || !vault.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Execute transfer from signer to vault
    Transfer {
        from: signer,
        to: vault,
        lamports,
    }
    .invoke()
}
```

### Step 7: Implement Withdraw (instructions/withdraw.rs)

```rust
use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

/// Processes the withdraw instruction.
///
/// # Accounts
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The vault PDA |
/// | 1     | ✓        | ✓      | The signer/recipient account |
pub fn process(accounts: &[AccountView], lamports: u64) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    
    let vault = &accounts[0];
    let signer = &accounts[1];

    // Validate signer
    if !signer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate accounts are writable
    if !vault.is_writable() || !signer.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Create PDA signer seeds
    // In production, derive these from the actual PDA seeds
    let bump = [255u8]; // Replace with actual bump
    let seeds: &[&[u8]] = &[signer.address().as_ref(), &bump];
    let pda_signer = Signer::from(seeds);

    // Execute transfer from vault to signer
    Transfer {
        from: vault,
        to: signer,
        lamports,
    }
    .invoke_signed(&[pda_signer])
}
```

### Step 8: Build

```bash
cargo build-sbf
```

---

## Key Differences: Anchor vs Pinocchio

| Feature | Anchor | Pinocchio |
|---------|--------|-----------|
| **Dependencies** | Many (anchor-lang, etc.) | Zero (pinocchio only) |
| **Account Type** | `AccountInfo` | `AccountView` |
| **Public Key Type** | `Pubkey` | `Address` |
| **Account Access** | Automatic deserialization | Zero-copy, manual parsing |
| **CPI Signers** | `CpiContext::new_with_signer` | `Signer::from(&seeds)` |
| **Compute Units** | Higher | Lower (optimized) |
| **Binary Size** | Larger | Smaller |
| **Development Speed** | Faster | Slower (more manual work) |

---

## Compute Unit Comparison

When optimizing for production, Pinocchio typically saves 20-50% on compute units compared to Anchor for equivalent operations. The exact savings depend on:

- Instruction complexity
- Number of accounts
- Data serialization requirements

---

## Resources

- [Pinocchio GitHub](https://github.com/anza-xyz/pinocchio)
- [Anchor Framework](https://www.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
