//! Instruction to create an account with a seed-derived address.

use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccountWithSeed;

/// Processes the `CreateAccountWithSeed` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `base` - The base address used to derive the new account.
/// * `seed` - The seed used to derive the account.
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
/// | 1     | ✓        | ✗      | The new account to be created. |
/// | 2     | ✗        | ✓      | The base account (optional, defaults to funding account). |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 2 accounts are provided.
/// * [`ProgramError::MissingRequiredSignature`] - If required accounts are not signers.
pub fn process_create_account_with_seed<'a>(
    accounts: &'a [AccountView],
    base: &Address,
    seed: &str,
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
    let base_account = if accounts.len() > 2 {
        Some(&accounts[2])
    } else {
        None
    };

    if !funding_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    CreateAccountWithSeed {
        from: funding_account,
        to: new_account,
        base: base_account,
        seed,
        lamports,
        space,
        owner,
    }
    .invoke_signed(signers)
}
