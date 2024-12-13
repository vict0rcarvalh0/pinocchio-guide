// /// System Program Instructions
// #[cfg(feature = "advance_nonce_account")]
// mod advance_nonce_account;
// #[cfg(feature = "advance_nonce_account")]
// use advance_nonce_account::*;

// /// SPL Token Instructions
// #[cfg(feature = "approve")]
// mod approve;
// #[cfg(feature = "approve")]
// use approve::*;

// #[cfg(feature = "approve_checked")]
// mod approve_checked;
// #[cfg(feature = "approve_checked")]
// use approve_checked::*;

// #[cfg(feature = "burn")]
// mod burn;
// #[cfg(feature = "burn")]
// use burn::*;

// #[cfg(feature = "burn_checked")]
// mod burn_checked;
// #[cfg(feature = "burn_checked")]
// use burn_checked::*;

// #[cfg(feature = "close_account")]
// mod close_account;
// #[cfg(feature = "close_account")]
// use close_account::*;

// #[cfg(feature = "freeze_account")]
// mod freeze_account;
// #[cfg(feature = "freeze_account")]
// use freeze_account::*;

// #[cfg(feature = "initialize_account")]
// mod initialize_account;
// #[cfg(feature = "initialize_account")]
// use initialize_account::*;

// #[cfg(feature = "mint_to")]
//mod mint_to;
// #[cfg(feature = "mint_to")]
//use mint_to::*;

// #[cfg(feature = "mint_to_checked")]
// mod mint_to_checked;
// #[cfg(feature = "mint_to_checked")]
// use mint_to_checked::*;

// #[cfg(feature = "revoke")]
// mod revoke;
// #[cfg(feature = "revoke")]
// use revoke::*;

#[cfg(feature = "set_authority")]
mod set_authority;
#[cfg(feature = "set_authority")]
use set_authority::*;

#[cfg(feature = "sync_native")]
mod sync_native;
#[cfg(feature = "sync_native")]
use sync_native::*;

#[cfg(feature = "thaw_account")]
mod thaw_account;
#[cfg(feature = "thaw_account")]
use thaw_account::*;

#[cfg(feature = "transfer")]
mod transfer;
#[cfg(feature = "transfer")]
use transfer::*;

#[cfg(feature = "transfer_checked")]
mod transfer_checked;
#[cfg(feature = "transfer_checked")]
use transfer_checked::*;
