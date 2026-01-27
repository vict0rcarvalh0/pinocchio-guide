//! Instruction to burn tokens with decimal validation.

use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::BurnChecked;

/// Processes the `BurnChecked` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `amount` - The amount of tokens to burn.
/// * `decimals` - The number of decimals for the token.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The account to burn from. |
/// | 1     | ✓        | ✗      | The token mint. |
/// | 2     | ✗        | ✓      | The account's owner/delegate. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 3 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If accounts are not writable.
/// * [`ProgramError::MissingRequiredSignature`] - If `authority_account` is not a signer.
pub fn process_burn_checked<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    decimals: u8,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let burn_account = &accounts[0];
    let mint_account = &accounts[1];
    let authority_account = &accounts[2];

    if !burn_account.is_writable() || !mint_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    BurnChecked {
        account: burn_account,
        mint: mint_account,
        authority: authority_account,
        amount,
        decimals,
    }
    .invoke_signed(signers)
}
