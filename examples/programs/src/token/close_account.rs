use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::CloseAccount;

// A constant representing the program ID, decoded from a base58 string.
// const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");

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
    // Validate that the data length is sufficient.
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the bump seed from the data.
    let bump = unsafe { *(data.as_ptr().add(0) as *const [u8; 1]) };

    // Process the CloseAccount instruction.
    process_close_account(accounts, bump)
}

/// Processes the `CloseAccount` instruction.
///
/// This function handles the logic for closing an account. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `bump`: The bump seed for the signer account.
///
/// ### Accounts:
/// 0. `[WRITE]` The account to close.
/// 1. `[WRITE]` The destination account.
/// 2. `[SIGNER]` The account's owner.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_close_account<'a>(
    accounts: &'a [AccountInfo],
    bump: [u8; 1], // Bump seed for the signer account.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [close_account, destination_account, authority_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys) 
    };

    // Ensure that the 'close' account is writable.
    if !close_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'destination' account is writable.
    if !destination_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'authority' account is a signer.
    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `CloseAccount` instruction.
    let close_account_instruction = CloseAccount {
        account: close_account,
        destination: destination_account,
        authority: authority_account,
    };

    // Create the seeds and signer for the instruction.
    let seeds = [Seed::from(b"authority_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the signer.
    close_account_instruction.invoke_signed(&signer)?;

    Ok(())
}