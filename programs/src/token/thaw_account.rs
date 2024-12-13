use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
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
    accounts: &'a [AccountInfo<'a>],
    program_id: &Pubkey,
) -> ProgramResult {
    // Iterate over the provided accounts
    let [token_account, mint_account, freeze_authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate that the token account is writable
    if !token_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }
    assert!(token_account.is_writable(), ProgramError::InvalidAccountData);

    // Validate the token account is owned by the current program
    if token_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate the mint account
    if mint_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate the freeze authority is a signer
    if !freeze_authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the ThawAccount instruction
    let thaw_account_instruction = ThawAccount {
        account: token_account,
        mint: mint_account,
        freeze_authority: freeze_authority_account,
    };

    // Invoke the instruction
    thaw_account_instruction.invoke()
}