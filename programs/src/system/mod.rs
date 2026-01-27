//! System program instruction implementations.
//!
//! This module contains example implementations of Solana's native system
//! program instructions using Pinocchio. These instructions handle fundamental
//! blockchain operations like:
//!
//! - **Account Management**: Creating, allocating, and assigning accounts
//! - **Transfers**: Moving SOL between accounts
//! - **Nonce Operations**: Durable transaction nonces for offline signing
//!
//! # Example
//!
//! ```ignore
//! use pinocchio::{AccountView, ProgramResult};
//! use crate::system::transfer_lamports::process_transfer;
//!
//! fn handle_transfer(accounts: &[AccountView], lamports: u64) -> ProgramResult {
//!     process_transfer(accounts, lamports, &[])
//! }
//! ```

pub mod advance_nonce_account;
pub mod allocate;
pub mod allocate_with_seed;
pub mod assign;
pub mod assign_with_seed;
pub mod authorize_nonce_account;
pub mod create_account;
pub mod create_account_with_seed;
pub mod initialize_nonce_account;
pub mod transfer_lamports;
pub mod transfer_with_seed;
pub mod update_nonce_account;
pub mod withdraw_nonce_account;
