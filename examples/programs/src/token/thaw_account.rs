use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::instructions::ThawAccount;

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
    // Validate that the instruction data is at least 8 bytes long.
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Delegate processing to the `process_thaw_account` function.
    process_thaw_account(accounts)
}

/// Processes the `ThawAccount` instruction.
///
/// This function handles the logic for thawing a frozen token account. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
///
/// ### Accounts:
/// 0. `[WRITE]` The token account to be thawed.
/// 1. `[]` The token mint associated with the account.
/// 2. `[SIGNER]` The freeze authority for the mint.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_thaw_account<'a>(
    accounts: &'a [AccountInfo],
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [token_account, mint_account, freeze_authority_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the token account is writable.
    if !token_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the freeze authority is a signer.
    if !freeze_authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `ThawAccount` instruction.
    let thaw_account_instruction = ThawAccount {
        account: token_account,
        mint: mint_account,
        freeze_authority: freeze_authority_account,
    };

    // Invoke the instruction.
    thaw_account_instruction.invoke()?;

    Ok(())
}