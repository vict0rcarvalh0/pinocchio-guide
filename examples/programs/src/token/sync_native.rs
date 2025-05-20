use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::instructions::SyncNative;

// A constant representing the program ID, decoded from a base58 string.
const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");

// Macro to define the program's entry point.
entrypoint!(process_instruction);

/// Entry point for the program. This function is called when the program is invoked.
///
/// ### Parameters:
/// - `_program_id`: The ID of the program being executed.
/// - `accounts`: The accounts passed to the program.
/// - `data`: Additional data passed to the program.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the program execution.
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Validate the length of the instruction data.
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Process the SyncNative instruction.
    process_sync_native(accounts)
}

/// Processes the `SyncNative` instruction.
///
/// This function handles the logic for synchronizing a native token account with the underlying
/// lamports. It validates the accounts, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
///
/// ### Accounts:
/// 0. `[WRITE]` The native token account to be synchronized with the underlying lamports.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_sync_native<'a>(accounts: &'a [AccountInfo]) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [native_token_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the native token account is writable.
    if !native_token_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Construct the `SyncNative` instruction.
    let sync_native_instruction = SyncNative {
        native_token: native_token_account,
    };

    // Invoke the instruction.
    sync_native_instruction.invoke()
}