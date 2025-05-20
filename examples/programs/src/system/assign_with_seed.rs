use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    instruction::{Signer, Seed}, 
    program_error::ProgramError, 
    pubkey::{self, Pubkey}, 
    ProgramResult
};

use pinocchio_system::instructions::AssignWithSeed;

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
    if data.len() < 10 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the seed length from the instruction data.
    let seed_len = unsafe { *(data.as_ptr() as *const u8) } as usize;

    // Ensure the data length is sufficient for the seed, owner, and bump.
    if data.len() < 1 + seed_len + 32 + 1 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the seed from the instruction data.
    let seed = unsafe {
        std::str::from_utf8_unchecked(&data[1..1 + seed_len])
    };

    // Extract the owner public key from the instruction data.
    let owner_offset = 1 + seed_len;
    let owner = unsafe { *(data.as_ptr().add(owner_offset) as *const Pubkey) };

    // Extract the bump value from the instruction data.
    let bump_offset = owner_offset + 32;
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(bump_offset) as *const [u8; 1]) };

    // Process the `AssignWithSeed` instruction.
    process_assign_with_seed(accounts, seed, &owner, bump)
}

/// Processes the `AssignWithSeed` instruction.
///
/// This function handles the logic for assigning an account with a seed. It validates the accounts,
/// constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `seed`: The seed used to derive the account.
/// - `owner`: The public key of the new program owner.
/// - `bump`: The bump seed used for the derived address.
///
/// ### Accounts:
/// 0. `[WRITE]` The account to be reassigned.
/// 1. `[SIGNER]` The base account used to derive the reassigned account.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_assign_with_seed<'a>(
    accounts: &'a [AccountInfo],
    seed: &str,
    owner: &Pubkey,
    bump: [u8; 1],
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [assigned_account, base_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the base account is a signer.
    if !base_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate the seed length.
    if seed.len() > pubkey::MAX_SEED_LEN {
        return Err(ProgramError::InvalidSeeds);
    }

    // Construct the `AssignWithSeed` instruction.
    let assign_with_seed_instruction = AssignWithSeed {
        account: assigned_account,
        base: base_account,
        seed,
        owner,
    };

    // Create the seeds and signer for the instruction.
    let seeds = [Seed::from(b"base_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the signer.
    assign_with_seed_instruction.invoke_signed(&signer)?;

    Ok(())
}