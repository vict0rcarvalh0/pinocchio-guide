//! Instruction to create a new account.

use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

/// Processes the `CreateAccount` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `lamports` - The number of lamports to transfer to the new account.
/// * `space` - The number of bytes to allocate for the new account.
/// * `owner` - The program that will own the new account.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✓      | The funding account. |
/// | 1     | ✓        | ✓      | The new account to be created. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 2 accounts are provided.
/// * [`ProgramError::MissingRequiredSignature`] - If either account is not a signer.
pub fn process_create_account<'a>(
    accounts: &'a [AccountView],
    lamports: u64,
    space: u64,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let funding_account = &accounts[0];
    let new_account = &accounts[1];

    if !funding_account.is_signer() || !new_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    CreateAccount {
        from: funding_account,
        to: new_account,
        lamports,
        space,
        owner,    }
    .invoke_signed(signers)
}
