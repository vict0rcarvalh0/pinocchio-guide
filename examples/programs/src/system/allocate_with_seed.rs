use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    instruction::{Signer, Seed}, 
    program_error::ProgramError, 
    pubkey::{self, Pubkey}, 
    ProgramResult
};

use pinocchio_system::instructions::AllocateWithSeed;

const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");
entrypoint!(process_instruction);

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
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `seed`: The seed used to derive the account's address.
/// - `space`: The number of bytes to allocate.
/// - `owner`: The program that will own the allocated account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The allocated account.
/// 1. `[SIGNER]` The base account used to derive the allocated account.
pub fn process_allocate_with_seed<'a>(
    accounts: &'a [AccountInfo],
    seed: &str,            // String used along with the base public key to derive the allocated account's address.
    space: u64,            // The number of bytes to allocate for the account.
    owner: &Pubkey,        // The program that will own the allocated account.
    bump: [u8; 1],
) -> ProgramResult {
    // Extracting account information
    let [allocated_account, base_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the base account is a signer
    assert!(base_account.is_signer());

    // Validate the seed length
    assert!(seed.len() > pubkey::MAX_SEED_LEN);

    // Creating the instruction instance
    let allocate_with_seed_instruction = AllocateWithSeed {
        account: allocated_account,
        base: base_account,
        seed,
        space,
        owner,
    };

    let seeds = [Seed::from(b"base_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];
    // Invoking the instruction
    allocate_with_seed_instruction.invoke_signed(&signer)?;

    Ok(())
}