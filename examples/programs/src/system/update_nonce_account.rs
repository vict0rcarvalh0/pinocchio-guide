use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_system::instructions::UpdateNonceAccount;

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
    // Ensure the data length is sufficient for extracting the bump value.
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the bump value from the data.
    let bump = unsafe { *(data.as_ptr().add(0) as *const [u8; 1]) };

    // Delegate processing to the `process_update_nonce_account` function.
    process_update_nonce_account(accounts, bump)
}

/// Processes the `UpdateNonceAccount` instruction.
///
/// This function handles the logic for updating a nonce account. It validates the accounts,
/// constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `bump`: The bump seed used for generating the signer.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_update_nonce_account<'a>(
    accounts: &'a [AccountInfo],
    bump: [u8; 1],  
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [nonce_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'nonce_account' is writable.
    assert!(nonce_account.is_writable());

    // Construct the `UpdateNonceAccount` instruction.
    let update_nonce_instruction = UpdateNonceAccount {
        account: nonce_account,
    };

    // Create the seeds and signer for the instruction.
    let seeds = [Seed::from(b"seeds"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the signer.
    update_nonce_instruction.invoke_signed(&signer)?;

    Ok(())
}