use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    ProgramResult
};

use pinocchio_system::instructions::UpdateNonceAccount;

/// Processes the `UpdateNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
pub fn process_update_nonce_account<'a>(
    accounts: &'a [AccountInfo],
    signers: &[Signer],  // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [nonce_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'nonce_account' is writable
    assert!(nonce_account.is_writable(), ProgramError::InvalidAccountData);

    // Creating the instruction instance
    let update_nonce_instruction = UpdateNonceAccount {
        account: nonce_account,
    };

    // Invoking the instruction
    update_nonce_instruction.invoke_signed(signers)?;

    Ok(())
}