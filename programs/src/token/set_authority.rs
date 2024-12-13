use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::Signer,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::{AuthorityType, SetAuthority};

/// Processes the SetAuthority instruction.
///
/// ### Accounts:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The mint or account to change the authority of.
///   1. `[SIGNER]` The current authority of the mint or account.
pub fn process_set_authority<'a>(
    accounts: &'a [AccountInfo],
    authority_type: AuthorityType,
    new_authority: Option<&Pubkey>, // Optional new authority
    signers: &[Signer],
) -> ProgramResult {
    // Extract account information
    let [account_to_update, current_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // Ensure the account to update is writable
    assert!(account_to_update.is_writable(), ProgramError::InvalidAccountData);

    // Ensure the current authority account is a signer
    assert!(current_authority.is_signer(), ProgramError::MissingRequiredSignature);

    // Create the instruction instance
    let set_authority_instruction = SetAuthority {
        account: account_to_update,
        authority: current_authority,
        authority_type,
        new_authority,
    };

    // Invoke the instruction
    set_authority_instruction.invoke_signed(signers)?;

    Ok(())
}