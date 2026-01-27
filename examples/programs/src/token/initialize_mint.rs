//! Instruction to initialize a mint account.

use pinocchio::{
    AccountView,
    Address,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::InitializeMint;

/// Processes the `InitializeMint` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `decimals` - Number of decimals for the token.
/// * `mint_authority` - The public key of the mint authority.
/// * `freeze_authority` - An optional public key for the freeze authority.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | Mint account. |
/// | 1     | ✗        | ✗      | Rent sysvar. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 2 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If `mint_account` is not writable.
pub fn process_initialize_mint<'a>(
    accounts: &'a [AccountView],
    decimals: u8,
    mint_authority: &Address,
    freeze_authority: Option<&Address>,
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let mint_account = &accounts[0];
    let rent_sysvar = &accounts[1];

    if !mint_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    InitializeMint {
        mint: mint_account,
        rent_sysvar,
        decimals,
        mint_authority,
        freeze_authority,
    }
    .invoke()
}
