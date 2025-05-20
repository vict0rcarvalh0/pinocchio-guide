use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    instruction::{Signer, Seed}, 
    program_error::ProgramError, 
    pubkey::{self, Pubkey}, 
    ProgramResult
};

use pinocchio_system::instructions::AssignWithSeed;

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
    let seed_len = unsafe { *(data.as_ptr() as *const u8) } as usize;
    if data.len() < 1 + seed_len + 32 + 1 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let seed = unsafe {
        std::str::from_utf8_unchecked(&data[1..1 + seed_len])
    };
    let owner_offset = 1 + seed_len;
    let owner = unsafe { *(data.as_ptr().add(owner_offset) as *const Pubkey) };
    let bump_offset = owner_offset + 32;
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(bump_offset) as *const [u8; 1]) };
    process_assign_with_seed(accounts, seed, &owner, bump)
}

/// Processes the `AssignWithSeed` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `seed`: The seed used to derive the account.
/// - `owner`: The public key of the new program owner.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The account to be reassigned.
/// 1. `[SIGNER]` The base account used to derive the reassigned account.
pub fn process_assign_with_seed<'a>(
    accounts: &'a [AccountInfo],
    seed: &str,
    owner: &Pubkey,
    bump: [u8; 1],
) -> ProgramResult {
    // Extracting account information
    let [assigned_account, base_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the base account is a signer
    assert!(base_account.is_signer());

    // Validate the seed length
    assert!(seed.len() > pubkey::MAX_SEED_LEN);

    // Creating the instruction instance
    let assign_with_seed_instruction = AssignWithSeed {
        account: assigned_account,
        base: base_account,
        seed,
        owner,
    };

    let seeds = [Seed::from(b"base_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoking the instruction
    assign_with_seed_instruction.invoke_signed(&signer)?;

    Ok(())
}