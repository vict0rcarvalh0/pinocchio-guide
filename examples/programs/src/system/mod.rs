//! System program instruction handlers.
//!
//! This module contains implementations for all system program instructions
//! using the Pinocchio framework.

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

pub use advance_nonce_account::process_advance_nonce_account;
pub use allocate::process_allocate;
pub use allocate_with_seed::process_allocate_with_seed;
pub use assign::process_assign;
pub use assign_with_seed::process_assign_with_seed;
pub use authorize_nonce_account::process_authorize_nonce_account;
pub use create_account::process_create_account;
pub use create_account_with_seed::process_create_account_with_seed;
pub use initialize_nonce_account::process_initialize_nonce_account;
pub use transfer_lamports::process_transfer;
pub use transfer_with_seed::process_transfer_with_seed;
pub use update_nonce_account::process_update_nonce_account;
pub use withdraw_nonce_account::process_withdraw_nonce_account;
