use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use pinocchio_token::instructions::SyncNative;

/// Process the SyncNative instruction.
///
/// ### Parameters:
/// - `accounts`: List of the accounts involved in the instruction..
///
/// ### Accounts:
///   0. `[WRITE]` The native token account to be syncronized with the subjacent lamports.
pub fn process_sync_native<'a>(
    accounts: &'a [AccountInfo<'a>],
    program_id: &Pubkey,
) -> ProgramResult {
    // Iterate over the provided accounts
    let [native_token_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate if the account is writable
    assert!(native_token_account.is_writable(), ProgramError::InvalidAccountData);

    // Validate if the account is owned by the program
    assert_eq!(native_token_account.owner, program_id, ProgramError::InvalidProgramId);

    // Construct the SyncNative instruction
    let sync_native_instruction = SyncNative {
        native_token: native_token_account,
    };

    // Invoke the instruction
    sync_native_instruction.invoke()
}