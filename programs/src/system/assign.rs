use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
};

use pinocchio::instructions::Assign;

/// Processes the `Assign` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `owner`: The public key of the new program owner.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The account to be reassigned to a new program owner.
pub fn process_assign<'a>(
    accounts: &'a [AccountInfo<'a>],
    owner: &Pubkey,      // Public key of the program to assign as the new owner of the account.
    signers: &[Signer],
) -> ProgramResult {
    // Extracting account information
    let [assigned_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    
    // Ensure the assigned account is a signer
    assert!(assigned_account.is_signer(), ProgramError::MissingRequiredSignature);

    // Creating the instruction instance
    let assign_instruction = Assign {
        account: assigned_account,
        owner,
    };

    // Invoking the instruction
    assign_instruction.invoke_signed(signers)?;

    Ok(())
}