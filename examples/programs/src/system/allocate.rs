use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    instruction::{Seed, Signer},
    ProgramResult
};

use pinocchio_system::instructions::Allocate;

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
    // Validate the length of the data buffer.
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the `space` and `bump` values from the data buffer.
    let space = unsafe { *(data.as_ptr() as *const u64) };
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(8) as *const [u8; 1]) };

    // Process the allocate instruction with the extracted parameters.
    process_allocate(accounts, space, bump)
}

/// Processes the `Allocate` instruction.
///
/// This function handles the logic for allocating space for an account. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `space`: The number of bytes to allocate.
/// - `bump`: The bump seed used for signing.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The account to allocate space for.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_allocate<'a>(
    accounts: &'a [AccountInfo],
    space: u64,
    bump: [u8; 1],
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [allocate_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the allocate account is a signer.
    if !allocate_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `Allocate` instruction.
    let allocate_instruction = Allocate {
        account: allocate_account,
        space,
    };

    // Create the seeds and signers for the instruction.
    let seeds = [Seed::from(b"seeds"), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    // Invoke the instruction with the provided signers.
    allocate_instruction.invoke_signed(&signers)?;

    Ok(())
}