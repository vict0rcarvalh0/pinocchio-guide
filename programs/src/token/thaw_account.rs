use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::ThawAccount;

/// Processes the ThawAccount instruction.
///
/// ### Parameters:
/// - `accounts`: List of accounts involved in the instruction.
///
/// ### Accounts:
///   0. `[WRITE]` The token account to be thawed.
///   1. `[]` The token mint associated with the account.
///   2. `[SIGNER]` The freeze authority for the mint.
pub fn process_thaw_account<'a>(
    accounts: &'a [AccountInfo],
    program_id: &Pubkey,
) -> ProgramResult {
    // Iterate over the provided accounts
    let [token_account, mint_account, freeze_authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate that the token account is writable
    assert!(token_account.is_writable(), ProgramError::InvalidAccountData);

    // Validate the token account is owned by the current program
    assert!(token_account.owner() != program_id, ProgramError::IncorrectProgramId);

    // Validate the mint account
    assert!(mint_account.owner() != program_id, ProgramError::IncorrectProgramId);

    // Validate the freeze authority is a signer
    assert!(freeze_authority_account.is_signer(), ProgramError::MissingRequiredSignature);

    // Construct the ThawAccount instruction
    let thaw_account_instruction = ThawAccount {
        account: token_account,
        mint: mint_account,
        freeze_authority: freeze_authority_account,
    };

    // Invoke the instruction
    thaw_account_instruction.invoke()
}