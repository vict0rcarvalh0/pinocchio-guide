/// System Program Instructions
#[cfg(feature = "advance_nonce_account")]
mod advance_nonce_account;
#[cfg(feature = "advance_nonce_account")]
use advance_nonce_account::*;

/// SPL Token Instructions
#[cfg(feature = "approve")]
mod approve;
#[cfg(feature = "approve")]
use approve::*;

#[cfg(feature = "approve_checked")]
mod approve_checked;
#[cfg(feature = "approve_checked")]
use approve_checked::*;

#[cfg(feature = "burn")]
mod burn;
#[cfg(feature = "burn")]
use burn::*;

#[cfg(feature = "burn_checked")]
mod burn_checked;
#[cfg(feature = "burn_checked")]
use burn_checked::*;

#[cfg(feature = "transfer")]
mod transfer;
#[cfg(feature = "transfer")]
use transfer::*;