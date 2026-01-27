//! SPL Token program instruction handlers.
//!
//! This module contains implementations for all SPL Token program instructions
//! using the Pinocchio framework.

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

pub use approve::process_approve;
pub use approve_checked::process_approve_checked;
pub use burn::process_burn;
pub use burn_checked::process_burn_checked;
pub use close_account::process_close_account;
pub use freeze_account::process_freeze_account;
pub use initialize_account::process_initialize_account;
pub use initialize_mint::process_initialize_mint;
pub use mint_to::process_mint_to;
pub use mint_to_checked::process_mint_to_checked;
pub use revoke::process_revoke;
pub use set_authority::process_set_authority;
pub use sync_native::process_sync_native;
pub use thaw_account::process_thaw_account;
pub use transfer_checked::process_transfer_checked;
pub use transfer_tokens::process_transfer;
