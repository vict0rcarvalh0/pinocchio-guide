# Pinocchio Tutorial

## Introduction
This is a simple pinocchio tutorial that involves creating transforming a Anchor Vault into a Pinocchio Vault.
A vault is basically a account where users can deposit and withdraw tokens, fungible or not.
This is the base of a lot of applications, a vault is needed for NFT or tokens staking or Escrows for example.

## Anchor Vault
So to start the vault creation in anchor, create the following structure in your directory after running the command `anchor new vault_example`:
```
📦programs
 ┗ 📂anchor-vault
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

In our example, the vault state contains just two properties, the vault and state bumps, so the content in the `vault.rs` would be:
```rust
use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}
```

By creating that, you can also add the in the mod.rs so you can...
```rust
pub mod vault;

pub use vault::*;
```

After that, in the initialize file, create the initialize struct and its implementation
```rust
use anchor_lang::prelude::*;

use crate::VaultState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account{
        init,
        payer = user, 
        seeds = [b"state", user.key.as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    }]
    pub vault_state: Account<'info, VaultState>, 

    #[account{
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
    }]
    pub vault: SystemAccount<'info>, 

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> { // 
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
} 
```

Now it comes the important part, to create the vault operations in the operations file
```rust
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::VaultState;

#[derive(Accounts)]
pub struct Operations<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account{
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    }]
    pub vault: SystemAccount<'info>, 
    #[account{
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    }]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
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

And now the last instruction file, to close the vault, that is useful to be called after some operations....
```rust
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::VaultState;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account{
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    }]
    pub vault: SystemAccount<'info>,
    #[account{
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user
    }]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

impl<'info> Close<'info> {
    pub fn close (&mut self) -> Result<()> {
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

To call the instructions in lib.rs you need to put this in `instructions/mod.rs`
```rust
pub mod initialize;
pub mod operations;
pub mod close;

pub use initialize::*;
pub use operations::*;
pub use close::*;
```

Now to finally finish the anchor vault, call all the instructions created in the `lib.rs`
```rust
use anchor_lang::prelude::*;
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

mod state;
use state::*;

mod instructions;
use instructions::*;

declare_id!("your default program ID generated by the anchor new command");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }

    pub fn deposit(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;

        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;

        Ok(())
    }
}
```
By running the `anchor build` command you should build your Anchor Vault and start the testing steps.

## Pinocchio Vault

step 1
cargo init --lib native-vault

cargo add pinocchio

step 2
adjust the crate type to cdylib(why) in cargo toml
[lib]
# Should be cdylib to build correctly with cargo build sbf
crate-type = ["cdylib", "lib"]

[dependencies]
# Typically native programs need the solana program (cargo add solana-program)
solana-program = "2.1.4"


step 3
create structure
```
📦src
 ┣ 📂instructions
 ┃ ┣ 📜deposit.rs
 ┃ ┣ 📜mod.rs
 ┃ ┗ 📜withdraw.rs
 ┗ 📜lib.rs
```

step 4 - lib.rs
```rust
use instructions::VaultInstructions;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, pubkey};

mod instructions;
use instructions::*;

// always use pubkey macro because creating it in runtime is gonna be more expensive
const ID: Pubkey = pubkey!("22222222222222222222222222222222"); //  32 lenght string

entrypoint!(process_instruction); // is going to .....

pub fn process_instruction(
    program_id: &Pubkey,          // Reference to program ID
    accounts: &[AccountInfo],     // Reference to array of accounts
    data: &[u8],                  // Reference to array of 1 bytes
) -> ProgramResult{               // ProgramResult is .....
    // crate id check so cannot pick the program and deploy with other adddress
    if program_id.ne(&crate::ID) {
        return Err(ProgramError::IncorrectProgramId);
    }

    // it will have an option and contain the discriminator and thedata to match the discriminator 
    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

}
```

step 5 - mod.rs
```rust
pub mod deposit;
pub mod withdraw;
use solana_program::program_error::ProgramError;

pub enum VaultInstructions {
    Deposit,
    Withdraw
}

impl TryFrom<&u8> for VaultInstructions {
    type Error = ProgramError;

    fn try_from(discriminator: &u8) -> Result<Self, Self::Error> {
        match discriminator { 
            0 => Ok(VaultInstructions::Deposit),
            1 => Ok(VaultInstructions::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}
```

add that match in lib.rs
```rust
    let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);

    match VaultInstructions::try_from(discriminator)? {
        VaultInstructions::Deposit => deposit::process(accounts, amount),
        VaultInstructions::Withdraw => withdraw::process(accounts, amount),
    }
```

step 6 - instructions(deposit)
```rust
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, program_error::ProgramError, pubkey::Pubkey, system_instruction::transfer};

pub fn process(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    let [signer, vault, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData)
    };

    // pda check
    let (pda, bump) = Pubkey::try_find_program_address(
        &[signer.key.as_ref()], 
        &crate::ID
    ).ok_or(ProgramError::InvalidSeeds)?;

    assert_eq!(&pda, vault.key);

    // invoke(instruction, account_infos)
    invoke(
        &transfer(
            signer.key, 
            vault.key, 
            lamports
        ), 
        accounts
    )
}
```

step 7 - instructions(withdraw)
```rust
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, system_instruction::transfer};

pub fn process(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    let [vault, signer, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData)
    };

    // pda check
    let (pda, bump) = Pubkey::try_find_program_address(
        &[signer.key.as_ref()], 
        &crate::ID
    ).ok_or(ProgramError::InvalidSeeds)?;

    assert_eq!(&pda, vault.key);

    invoke_signed(
        &transfer(
            signer.key, 
            vault.key, 
            lamports
        ), 
        accounts,
        &[&[signer.key.as_ref()]]
    )
}
```

step 8 - change match statement in lib.rs
```rust
match VaultInstructions::try_from(discriminator)? {
    VaultInstructions::Deposit => deposit::process(accounts, amount),
    VaultInstructions::Withdraw => withdraw::process(accounts, amount),
}
```

CU comparison
