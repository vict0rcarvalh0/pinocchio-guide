//! Instruction to transfer lamports from a seed-derived address.

use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::TransferWithSeed;

/// Processes the `TransferWithSeed` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `base` - The base address used to derive the source account.
/// * `seed` - The seed used to derive the source account.
/// * `lamports` - The number of lamports to transfer.
/// * `owner` - The program that owns the source account.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The source account. |
/// | 1     | ✗        | ✓      | The base account used to derive the source account. |
/// | 2     | ✓        | ✗      | The destination account. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 3 accounts are provided.
/// * [`ProgramError::MissingRequiredSignature`] - If `base_account` is not a signer.
pub fn process_transfer_with_seed<'a>(
    accounts: &'a [AccountView],
    base: &Address,
    seed: &str,
    lamports: u64,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let from_account = &accounts[0];
    let base_account = &accounts[1];
    let to_account = &accounts[2];

    if !from_account.is_writable() || !to_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !base_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    TransferWithSeed {
        from: from_account,
        base: base_account,
        to: to_account,
        lamports,
        seed,
        owner,
    }
    .invoke_signed(signers)
}
