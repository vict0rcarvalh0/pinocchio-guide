use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    ProgramResult
};

use pinocchio_token::instructions::FreezeAccount;

/// Processes the FreezeAccount instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to freeze.
///   1. `[]` The token mint.
///   2. `[SIGNER]` The mint freeze authority.
pub fn process_freeze_account<'a>(
    accounts: &'a [AccountInfo],
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [account_to_freeze, mint_account, freeze_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // Ensure that the account to freeze is writable
    assert!(account_to_freeze.is_writable(), ProgramError::InvalidAccountData);

    // Ensure that the freeze authority is a signer
    assert!(freeze_authority.is_signer(), ProgramError::MissingRequiredSignature);

    // Creating the instruction instance
    let freeze_account_instruction = FreezeAccount {
        account: account_to_freeze,
        mint: mint_account,
        freeze_authority,
    };

    // Invoking the instruction
    freeze_account_instruction.invoke_signed(signers)?;

    Ok(())
}