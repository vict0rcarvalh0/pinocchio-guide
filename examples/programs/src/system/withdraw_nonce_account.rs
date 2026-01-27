//! Instruction to withdraw lamports from a nonce account.

use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::WithdrawNonceAccount;

/// Processes the `WithdrawNonceAccount` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `lamports` - The number of lamports to withdraw.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The Nonce account. |
/// | 1     | ✓        | ✗      | The recipient account. |
/// | 2     | ✗        | ✗      | The recent blockhashes sysvar. |
/// | 3     | ✗        | ✗      | The rent sysvar. |
/// | 4     | ✗        | ✓      | The Nonce authority. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 5 accounts are provided.
/// * [`ProgramError::MissingRequiredSignature`] - If `nonce_authority` is not a signer.
pub fn process_withdraw_nonce_account<'a>(
    accounts: &'a [AccountView],
    lamports: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 5 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];
    let recipient_account = &accounts[1];
    let recent_blockhashes_sysvar = &accounts[2];
    let rent_sysvar = &accounts[3];
    let nonce_authority = &accounts[4];

    if !nonce_account.is_writable() || !recipient_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    WithdrawNonceAccount {
        account: nonce_account,
        recipient: recipient_account,
        recent_blockhashes_sysvar,
        rent_sysvar,
        authority: nonce_authority,
        lamports,
    }
    .invoke_signed(signers)
}
