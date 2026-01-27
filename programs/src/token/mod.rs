//! SPL Token instruction implementations.
//!
//! This module contains example implementations of SPL Token program
//! instructions using Pinocchio. These instructions handle token operations:
//!
//! - **Token Setup**: Initialize mints and token accounts
//! - **Token Operations**: Transfer, approve, revoke, burn
//! - **Account Management**: Freeze, thaw, close accounts
//! - **Checked Variants**: Operations with decimal validation
//!
//! # Note
//!
//! These implementations demonstrate Cross-Program Invocation (CPI) patterns
//! with the SPL Token program.
//!
//! # Example
//!
//! ```ignore
//! use pinocchio::{AccountView, ProgramResult};
//! use crate::token::transfer_tokens::process_transfer;
//!
//! fn transfer_spl_tokens(accounts: &[AccountView], amount: u64) -> ProgramResult {
//!     process_transfer(accounts, amount, &[])
//! }
//! ```

pub mod approve;
pub mod approve_checked;
pub mod burn;
pub mod burn_checked;
pub mod close_account;
pub mod freeze_account;
pub mod initialize_account;
pub mod initialize_mint;
pub mod mint_to;
pub mod mint_to_checked;
pub mod revoke;
pub mod set_authority;
pub mod sync_native;
pub mod thaw_account;
pub mod transfer_checked;
pub mod transfer_tokens;
