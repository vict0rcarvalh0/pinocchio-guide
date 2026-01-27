//! Instruction to thaw a frozen token account.

use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::ThawAccount;

/// Processes the `ThawAccount` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The account to thaw. |
/// | 1     | ✗        | ✗      | The token mint. |
/// | 2     | ✗        | ✓      | The mint freeze authority. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 3 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If `account_to_thaw` is not writable.
/// * [`ProgramError::MissingRequiredSignature`] - If `freeze_authority` is not a signer.
pub fn process_thaw_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let account_to_thaw = &accounts[0];
    let mint_account = &accounts[1];
    let freeze_authority = &accounts[2];

    if !account_to_thaw.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !freeze_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    ThawAccount {
        account: account_to_thaw,
        mint: mint_account,
        freeze_authority,
    }
    .invoke_signed(signers)
}
