use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use pinocchio_token::instructions::CloseAccount;

/// Processes the CloseAccount instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to close.
///   1. `[WRITE]` The destination account.
///   2. `[SIGNER]` The account's owner.
pub fn process_close_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [close_account, destination_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys) 
    };

    // Ensure that the 'close' account is writable
    assert!(close_account.is_writable(), ProgramError::InvalidAccountData);

    // Ensure that the 'destination' account is writable
    assert!(destination_account.is_writable(), ProgramError::InvalidAccountData);

    // Ensure that the 'authority' account is a signer
    assert!(authority_account.is_signer(), ProgramError::MissingRequiredSignature);

    // Creating the instruction instance
    let close_account_instruction = CloseAccount {
        account: close_account,
        destination: destination_account,
        authority: authority_account,
    };

    // Invoking the instruction
    close_account_instruction.invoke_signed(signers)?;

    Ok(())
}