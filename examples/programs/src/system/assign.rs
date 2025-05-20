use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_system::instructions::Assign;

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
    // Ensure the data length is sufficient to extract the required fields.
    if data.len() < 33 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the owner public key from the data.
    let owner = unsafe { *(data.as_ptr() as *const Pubkey) };

    // Extract the bump seed from the data.
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(32) as *const [u8; 1]) };

    // Process the `Assign` instruction with the extracted parameters.
    process_assign(accounts, &owner, bump)
}

/// Processes the `Assign` instruction.
///
/// This function handles the logic for assigning a new program owner to an account. It validates
/// the accounts and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `owner`: The public key of the new program owner.
/// - `bump`: The bump seed used for generating the program-derived address.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The account to be reassigned to a new program owner.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_assign<'a>(
    accounts: &'a [AccountInfo],
    owner: &Pubkey,      // Public key of the program to assign as the new owner of the account.
    bump: [u8; 1],
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [assigned_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the assigned account is a signer.
    if !assigned_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `Assign` instruction.
    let assign_instruction = Assign {
        account: assigned_account,
        owner,
    };

    // Generate the seeds for the program-derived address.
    let seeds = [Seed::from(b"assigned_account"), Seed::from(&bump)];

    // Create the signer array using the seeds.
    let signer = [Signer::from(&seeds)];

    // Invoke the `Assign` instruction with the signer.
    assign_instruction.invoke_signed(&signer)?;

    Ok(())
}