use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::SyncNative;

const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    process_sync_native(accounts)
}

/// Process the SyncNative instruction.
///
/// ### Parameters:
/// - `accounts`: List of the accounts involved in the instruction..
///
/// ### Accounts:
///   0. `[WRITE]` The native token account to be syncronized with the subjacent lamports.
pub fn process_sync_native<'a>(
    accounts: &'a [AccountInfo],
    program_id: &Pubkey,
) -> ProgramResult {
    // Iterate over the provided accounts
    let [native_token_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate if the account is writable
    assert!(native_token_account.is_writable());

    // Validate if the account is owned by the program
    assert_eq!(native_token_account.owner(), program_id);

    // Construct the SyncNative instruction
    let sync_native_instruction = SyncNative {
        native_token: native_token_account,
    };

    // Invoke the instruction
    sync_native_instruction.invoke()
}