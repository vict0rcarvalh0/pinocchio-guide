use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_system::instructions::Transfer;

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
    // Ensure the data length is sufficient for the instruction.
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the bump seed from the data.
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(0) as *const [u8; 1]) };

    // Extract the lamports amount from the data.
    let lamports = unsafe { *(data.as_ptr().add(1) as *const u64) };

    // Process the transfer instruction.
    process_transfer(accounts, lamports, bump)
}

/// Processes the `Transfer` instruction.
///
/// This function handles the logic for transferring lamports between accounts. It validates
/// the accounts and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer.
/// - `bump`: The bump seed used for signing.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The source account.
/// 1. `[WRITE]` The destination account.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_transfer<'a>(
    accounts: &'a [AccountInfo],
    lamports: u64,        // The amount of lamports to transfer.
    bump: [u8; 1],        // The bump seed used for signing.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [from_account, to_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'from' account is writable and a signer.
    if !from_account.is_writable() || !from_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure that the 'to' account is writable.
    if !to_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Construct the `Transfer` instruction.
    let transfer_instruction = Transfer {
        from: from_account,
        to: to_account,
        lamports,
    };

    // Create the seeds and signer for the instruction.
    let seeds = [Seed::from(b"from_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the signer.
    transfer_instruction.invoke_signed(&signer)?;

    Ok(())
}