use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    instruction::{Signer, Seed}, 
    program_error::ProgramError, 
    pubkey::{self, Pubkey}, 
    ProgramResult
};

use pinocchio_system::instructions::AllocateWithSeed;

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
    if data.len() < 10 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract `seed` length (u8) and the `seed` string
    let seed_len = unsafe { *(data.as_ptr() as *const u8) } as usize;
    if data.len() < 1 + seed_len + 8 + 32 + 1 {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    let seed = unsafe {
        std::str::from_utf8_unchecked(&data[1..1 + seed_len])
    };

    // Extract `space` (u64) from the next 8 bytes after the seed
    let space_offset = 1 + seed_len;
    let space = unsafe { *(data.as_ptr().add(space_offset) as *const u64) };

    // Extract `owner` (Pubkey) from the next 32 bytes after `space`
    let owner_offset = space_offset + 8;
    let owner = unsafe { *(data.as_ptr().add(owner_offset) as *const Pubkey) };

    // Extract `bump` ([u8; 1]) from the last byte
    let bump_offset = owner_offset + 32;
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(bump_offset) as *const [u8; 1]) };

    // Call `process_allocate_with_seed` with the new parameters
    process_allocate_with_seed(accounts, seed, space, &owner, bump)
}

/// Processes the `AllocateWithSeed` instruction.
///
/// This function handles the logic for allocating an account with a seed. It validates the accounts,
/// constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `seed`: The seed used to derive the account's address.
/// - `space`: The number of bytes to allocate.
/// - `owner`: The program that will own the allocated account.
/// - `bump`: The bump seed used for address derivation.
///
/// ### Accounts:
/// 0. `[WRITE]` The allocated account.
/// 1. `[SIGNER]` The base account used to derive the allocated account.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_allocate_with_seed<'a>(
    accounts: &'a [AccountInfo],
    seed: &str,            // String used along with the base public key to derive the allocated account's address.
    space: u64,            // The number of bytes to allocate for the account.
    owner: &Pubkey,        // The program that will own the allocated account.
    bump: [u8; 1],
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [allocated_account, base_account] = accounts else {
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

    // Construct the `AllocateWithSeed` instruction.
    let allocate_with_seed_instruction = AllocateWithSeed {
        account: allocated_account,
        base: base_account,
        seed,
        space,
        owner,
    };

    // Prepare the seeds and signer for the instruction.
    let seeds = [Seed::from(b"base_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction.
    allocate_with_seed_instruction.invoke_signed(&signer)?;

    Ok(())
}