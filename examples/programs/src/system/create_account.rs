use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_system::instructions::CreateAccount;

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
    // Validate the length of the instruction data.
    if data.len() < 42 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract parameters from the instruction data.
    let lamports = unsafe { *(data.as_ptr() as *const u64) };
    let space = unsafe { *(data.as_ptr().add(8) as *const u64) };
    let owner = unsafe { *(data.as_ptr().add(16) as *const Pubkey) };
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(48) as *const [u8; 1]) };

    // Process the `CreateAccount` instruction.
    process_create_account(accounts, lamports, space, &owner, bump)
}

/// Processes the `CreateAccount` instruction.
///
/// This function handles the logic for creating a new account. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer to the new account.
/// - `space`: The number of bytes to allocate for the new account.
/// - `owner`: The program that will own the new account.
/// - `bump`: The bump seed used for the program-derived address.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The funding account.
/// 1. `[WRITE, SIGNER]` The new account to be created.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_create_account<'a>(
    accounts: &'a [AccountInfo],
    lamports: u64,   // Number of lamports to transfer to the new account.
    space: u64,      // Number of bytes to allocate for the new account.
    owner: &Pubkey,  // Pubkey of the program that will own the new account.
    bump: [u8; 1],
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [funding_account, new_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the funding account and new account are signers.
    if !funding_account.is_signer() || !new_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `CreateAccount` instruction.
    let create_account_instruction = CreateAccount {
        from: funding_account,
        to: new_account,
        lamports,
        space,
        owner,
    };

    // Define the seeds and signer for the instruction.
    let seeds = [Seed::from(b"funding_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the provided signer.
    create_account_instruction.invoke_signed(&signer)?;

    Ok(())
}