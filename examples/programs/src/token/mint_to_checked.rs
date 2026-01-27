//! Instruction to mint tokens with decimal validation.

use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::MintToChecked;

/// Processes the `MintToChecked` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `amount` - The amount of tokens to mint.
/// * `decimals` - The number of decimal places for the tokens.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The mint account. |
/// | 1     | ✓        | ✗      | The account to mint tokens to. |
/// | 2     | ✗        | ✓      | The mint's minting authority. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 3 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If accounts are not writable.
/// * [`ProgramError::MissingRequiredSignature`] - If `mint_authority` is not a signer.
pub fn process_mint_to_checked<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    decimals: u8,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let mint_account = &accounts[0];
    let token_account = &accounts[1];
    let mint_authority = &accounts[2];

    if !mint_account.is_writable() || !token_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !mint_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    MintToChecked {
        mint: mint_account,
        account: token_account,
        mint_authority,
        amount,
        decimals,
    }
    .invoke_signed(signers)
}
