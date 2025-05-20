use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{Signer, Seed},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::Revoke;

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
    // Validate the instruction data length.
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the bump seed from the instruction data.
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(0) as *const [u8; 1]) };

    // Process the revoke instruction.
    process_revoke(accounts, bump)
}

/// Processes the `Revoke` instruction.
///
/// This function handles the logic for revoking a token. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `bump`: The bump seed used for signing.
///
/// ### Accounts:
/// 0. `[WRITE]` The source account.
/// 1. `[SIGNER]` The source account owner.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_revoke<'a>(
    accounts: &'a [AccountInfo],
    bump: [u8; 1],
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [source_account, owner_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the source account is writable.
    if !source_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the owner account is a signer.
    if !owner_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `Revoke` instruction.
    let revoke_instruction = Revoke {
        source: source_account,
        authority: owner_account,
    };

    // Create the seeds and signers for the instruction.
    let seeds = [Seed::from(b"owner_account"), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    // Invoke the instruction with the provided signers.
    revoke_instruction.invoke_signed(&signers)?;

    Ok(())
}