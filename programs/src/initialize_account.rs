use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
};

use pinocchio::instructions::InitializeAccount;

/// Processes the InitializeAccount instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]`  The account to initialize.
///   1. `[]` The mint this account will be associated with.
///   2. `[]` The new account's owner/multisignature.
///   3. `[]` Rent sysvar.
pub fn process_initialize_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [account_to_initialize, mint_account, owner_account, rent_sysvar] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the account to initialize is writable
    assert!(account_to_initialize.is_writable(), ProgramError::InvalidAccountData);

    // Ensure the rent sysvar is valid (you might need additional checks here)
    assert_eq!(rent_sysvar.key(), &solana_program::sysvar::rent::ID, ProgramError::InvalidAccountData);

    // Creating the instruction instance
    let initialize_account_instruction = InitializeAccount {
        account: account_to_initialize,
        mint: mint_account,
        owner: owner_account,
        rent_sysvar,
    };

    // Invoking the instruction
    initialize_account_instruction.invoke_signed(signers)?;

    Ok(())
}