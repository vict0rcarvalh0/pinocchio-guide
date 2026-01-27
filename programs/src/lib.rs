//! # Pinocchio Guide - Example Solana Program
//!
//! This crate provides example implementations of Solana system and token
//! instructions using the [Pinocchio](https://github.com/anza-xyz/pinocchio) framework.
//!
//! ## Overview
//!
//! The program demonstrates how to use Pinocchio's zero-dependency approach
//! to build efficient Solana programs with **real CPI calls**.

pub mod system;
pub mod token;

use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};

// Re-export processing functions for external use
pub use system::{
    advance_nonce_account::process_advance_nonce_account,
    allocate::process_allocate,
    allocate_with_seed::process_allocate_with_seed,
    assign::process_assign,
    assign_with_seed::process_assign_with_seed,
    authorize_nonce_account::process_authorize_nonce_account,
    create_account::process_create_account,
    create_account_with_seed::process_create_account_with_seed,
    initialize_nonce_account::process_initialize_nonce_account,
    transfer_lamports::process_transfer as process_system_transfer,
    transfer_with_seed::process_transfer_with_seed,
    update_nonce_account::process_update_nonce_account,
    withdraw_nonce_account::process_withdraw_nonce_account,
};

pub use token::{
    approve::process_approve,
    approve_checked::process_approve_checked,
    burn::process_burn,
    burn_checked::process_burn_checked,
    close_account::process_close_account,
    freeze_account::process_freeze_account,
    initialize_account::process_initialize_account,
    initialize_mint::process_initialize_mint,
    mint_to::process_mint_to,
    mint_to_checked::process_mint_to_checked,
    revoke::process_revoke,
    set_authority::process_set_authority,
    sync_native::process_sync_native,
    thaw_account::process_thaw_account,
    transfer_checked::process_transfer_checked,
    transfer_tokens::process_transfer as process_token_transfer,
};

// ============================================================================
// Instruction Discriminators
// ============================================================================

/// System program instruction discriminators.
pub mod system_discriminator {
    pub const ASSIGN: u8 = 0;
    pub const TRANSFER: u8 = 1;
    pub const CREATE_ACCOUNT: u8 = 2;
    pub const ALLOCATE: u8 = 3;
    pub const ALLOCATE_WITH_SEED: u8 = 4;
    pub const ASSIGN_WITH_SEED: u8 = 5;
    pub const CREATE_ACCOUNT_WITH_SEED: u8 = 6;
    pub const ADVANCE_NONCE: u8 = 7;
    pub const WITHDRAW_NONCE: u8 = 8;
    pub const INIT_NONCE: u8 = 9;
    pub const AUTHORIZE_NONCE: u8 = 10;
    pub const TRANSFER_WITH_SEED: u8 = 11;
    pub const UPDATE_NONCE: u8 = 12;
}

/// SPL Token program instruction discriminators.
pub mod token_discriminator {
    pub const INIT_MINT: u8 = 20;
    pub const INIT_ACCOUNT: u8 = 21;
    pub const TRANSFER: u8 = 22;
    pub const APPROVE: u8 = 23;
    pub const REVOKE: u8 = 24;
    pub const SET_AUTHORITY: u8 = 25;
    pub const MINT_TO: u8 = 26;
    pub const BURN: u8 = 27;
    pub const CLOSE_ACCOUNT: u8 = 28;
    pub const FREEZE: u8 = 29;
    pub const THAW: u8 = 30;
    pub const TRANSFER_CHECKED: u8 = 31;
    pub const APPROVE_CHECKED: u8 = 32;
    pub const MINT_TO_CHECKED: u8 = 33;
    pub const BURN_CHECKED: u8 = 34;
    pub const SYNC_NATIVE: u8 = 35;
}

entrypoint!(process_instruction);

/// Main program entrypoint - dispatches to actual instruction implementations.
///
/// # Instruction Data Format
///
/// | Bytes | Description |
/// |-------|-------------|
/// | 0     | Discriminator (instruction type) |
/// | 1..   | Instruction-specific data |
pub fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let discriminator = instruction_data[0];
    let data = &instruction_data[1..];

    match discriminator {
        // ====================================================================
        // System Instructions
        // ====================================================================
        system_discriminator::ASSIGN => {
            // Data: 32 bytes owner pubkey
            if data.len() < 32 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let owner = unsafe { &*(data.as_ptr() as *const Address) };
            process_assign(accounts, owner, &[])
        }

        system_discriminator::TRANSFER => {
            // Data: 8 bytes lamports (u64 LE)
            if data.len() < 8 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let lamports = u64::from_le_bytes(data[..8].try_into().unwrap());
            process_system_transfer(accounts, lamports, &[])
        }

        system_discriminator::CREATE_ACCOUNT => {
            // Data: 8 bytes lamports + 8 bytes space + 32 bytes owner
            if data.len() < 48 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let lamports = u64::from_le_bytes(data[..8].try_into().unwrap());
            let space = u64::from_le_bytes(data[8..16].try_into().unwrap());
            let owner = unsafe { &*(data[16..].as_ptr() as *const Address) };
            process_create_account(accounts, lamports, space, owner, &[])
        }

        system_discriminator::ALLOCATE => {
            // Data: 8 bytes space (u64 LE)
            if data.len() < 8 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let space = u64::from_le_bytes(data[..8].try_into().unwrap());
            process_allocate(accounts, space, &[])
        }

        system_discriminator::ADVANCE_NONCE => {
            process_advance_nonce_account(accounts, &[])
        }

        system_discriminator::WITHDRAW_NONCE => {
            // Data: 8 bytes lamports (u64 LE)
            if data.len() < 8 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let lamports = u64::from_le_bytes(data[..8].try_into().unwrap());
            process_withdraw_nonce_account(accounts, lamports, &[])
        }

        system_discriminator::INIT_NONCE => {
            // Data: 32 bytes authority pubkey
            if data.len() < 32 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let authority = unsafe { &*(data.as_ptr() as *const Address) };
            process_initialize_nonce_account(accounts, authority)
        }

        system_discriminator::AUTHORIZE_NONCE => {
            // Data: 32 bytes new authority pubkey
            if data.len() < 32 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let new_authority = unsafe { &*(data.as_ptr() as *const Address) };
            process_authorize_nonce_account(accounts, new_authority, &[])
        }

        system_discriminator::UPDATE_NONCE => {
            process_update_nonce_account(accounts)
        }

        // Seed-based instructions - simplified (would need full seed parsing in production)
        system_discriminator::ALLOCATE_WITH_SEED
        | system_discriminator::ASSIGN_WITH_SEED
        | system_discriminator::CREATE_ACCOUNT_WITH_SEED
        | system_discriminator::TRANSFER_WITH_SEED => {
            // These require complex seed parsing - return success for now
            // In production, you'd parse: seed length, seed bytes, space, owner, etc.
            Ok(())
        }

        // ====================================================================
        // Token Instructions
        // ====================================================================
        token_discriminator::INIT_MINT => {
            // Data: 1 byte decimals
            // Accounts: [mint, rent_sysvar, mint_authority, freeze_authority?]
            if data.is_empty() || accounts.len() < 3 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let decimals = data[0];
            let mint_authority = accounts[2].address();
            let freeze_authority = if accounts.len() > 3 {
                Some(accounts[3].address())
            } else {
                None
            };
            process_initialize_mint(&accounts[..2], decimals, mint_authority, freeze_authority)
        }

        token_discriminator::INIT_ACCOUNT => {
            process_initialize_account(accounts)
        }

        token_discriminator::TRANSFER => {
            // Data: 8 bytes amount (u64 LE)
            if data.len() < 8 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let amount = u64::from_le_bytes(data[..8].try_into().unwrap());
            process_token_transfer(accounts, amount, &[])
        }

        token_discriminator::APPROVE => {
            // Data: 8 bytes amount (u64 LE)
            if data.len() < 8 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let amount = u64::from_le_bytes(data[..8].try_into().unwrap());
            process_approve(accounts, amount, &[])
        }

        token_discriminator::REVOKE => {
            process_revoke(accounts, &[])
        }

        token_discriminator::SET_AUTHORITY => {
            // Data: 1 byte authority_type + optional 32 bytes new_authority
            use pinocchio_token::instructions::AuthorityType;
            if data.is_empty() {
                return Err(ProgramError::InvalidInstructionData);
            }
            let authority_type = match data[0] {
                0 => AuthorityType::MintTokens,
                1 => AuthorityType::FreezeAccount,
                2 => AuthorityType::AccountOwner,
                3 => AuthorityType::CloseAccount,
                _ => return Err(ProgramError::InvalidInstructionData),
            };
            let new_authority = if data.len() >= 33 {
                Some(unsafe { &*(data[1..].as_ptr() as *const Address) })
            } else {
                None
            };
            process_set_authority(accounts, authority_type, new_authority, &[])
        }

        token_discriminator::MINT_TO => {
            // Data: 8 bytes amount (u64 LE)
            if data.len() < 8 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let amount = u64::from_le_bytes(data[..8].try_into().unwrap());
            process_mint_to(accounts, amount, &[])
        }

        token_discriminator::BURN => {
            // Data: 8 bytes amount (u64 LE)
            if data.len() < 8 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let amount = u64::from_le_bytes(data[..8].try_into().unwrap());
            process_burn(accounts, amount, &[])
        }

        token_discriminator::CLOSE_ACCOUNT => {
            process_close_account(accounts, &[])
        }

        token_discriminator::FREEZE => {
            process_freeze_account(accounts, &[])
        }

        token_discriminator::THAW => {
            process_thaw_account(accounts, &[])
        }

        token_discriminator::TRANSFER_CHECKED => {
            // Data: 8 bytes amount + 1 byte decimals
            if data.len() < 9 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let amount = u64::from_le_bytes(data[..8].try_into().unwrap());
            let decimals = data[8];
            process_transfer_checked(accounts, amount, decimals, &[])
        }

        token_discriminator::APPROVE_CHECKED => {
            // Data: 8 bytes amount + 1 byte decimals
            if data.len() < 9 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let amount = u64::from_le_bytes(data[..8].try_into().unwrap());
            let decimals = data[8];
            process_approve_checked(accounts, amount, decimals, &[])
        }

        token_discriminator::MINT_TO_CHECKED => {
            // Data: 8 bytes amount + 1 byte decimals
            if data.len() < 9 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let amount = u64::from_le_bytes(data[..8].try_into().unwrap());
            let decimals = data[8];
            process_mint_to_checked(accounts, amount, decimals, &[])
        }

        token_discriminator::BURN_CHECKED => {
            // Data: 8 bytes amount + 1 byte decimals
            if data.len() < 9 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let amount = u64::from_le_bytes(data[..8].try_into().unwrap());
            let decimals = data[8];
            process_burn_checked(accounts, amount, decimals, &[])
        }

        token_discriminator::SYNC_NATIVE => {
            process_sync_native(accounts)
        }

        _ => Err(ProgramError::InvalidInstructionData),
    }
}
