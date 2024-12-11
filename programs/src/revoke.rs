use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Signer,
    program_error::ProgramError,
};

use pinocchio_token::instructions::Revoke;

/// Processes the Revoke instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[SIGNER]` The source account owner.
pub fn process_revoke<'a>(
    accounts: &'a [AccountInfo<'a>],
    signers: &[Signer], // The signers array for authorization.
) -> ProgramResult {
    // Extracting account information
    let [source_account, owner_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the source account is writable
    assert!(source_account.is_writable(), ProgramError::InvalidAccountData);

    // Ensure the owner account is a signer
    assert!(owner_account.is_signer(), ProgramError::MissingRequiredSignature);

    // Creating the instruction instance
    let revoke_instruction = Revoke {
        source: source_account,
        authority: owner_account,
    };

    // Invoking the instruction
    revoke_instruction.invoke_signed(signers)?;

    Ok(())
}