use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_system::instructions::CreateAccountWithSeed;

const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 41 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let seed_len = unsafe { *(data.as_ptr() as *const u8) } as usize;
    if data.len() < 1 + seed_len + 8 + 8 + 32 + 1 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let seed = unsafe {
        std::str::from_utf8_unchecked(&data[1..1 + seed_len])
    };
    let lamports_offset = 1 + seed_len;
    let lamports = unsafe { *(data.as_ptr().add(lamports_offset) as *const u64) };
    let space_offset = lamports_offset + 8;
    let space = unsafe { *(data.as_ptr().add(space_offset) as *const u64) };
    let owner_offset = space_offset + 8;
    let owner = unsafe { *(data.as_ptr().add(owner_offset) as *const Pubkey) };
    let bump_offset = owner_offset + 32;
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(bump_offset) as *const [u8; 1]) };
    process_create_account_with_seed(accounts, seed, lamports, space, &owner, bump)
}

/// Processes the `CreateAccountWithSeed` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `seed`: The seed used to derive the account.
/// - `lamports`: The number of lamports to transfer to the new account.
/// - `space`: The number of bytes to allocate for the new account.
/// - `owner`: The program that will own the new account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The funding account.
/// 1. `[WRITE, SIGNER]` The new account to be created.
/// 2. `[OPTIONAL]` The base account used to derive the new account (if applicable).
pub fn process_create_account_with_seed<'a>(
    accounts: &'a [AccountInfo],
    seed: &'a str,      // The ASCII string that will be used as the seed to derive the address.
    lamports: u64,      // Number of lamports to transfer to the new account.
    space: u64,         // Number of bytes to allocate for the new account.
    owner: &Pubkey,     // Pubkey of the program that will own the new account.
    bump: [u8; 1],
) -> ProgramResult {
    // Extracting account information
    let [funding_account, new_account, base_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that funding account and new account are signers
    assert!(funding_account.is_signer() || new_account.is_signer());

    // Creating the instruction instance
    let create_account_with_seed_instruction = CreateAccountWithSeed {
        from: funding_account,
        to: new_account,
        base: Some(base_account),
        seed,
        lamports,
        space,
        owner,
    };

    let seeds = [Seed::from(b"funding_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];
    // Invoking the instruction
    create_account_with_seed_instruction.invoke_signed(&signer)?;

    Ok(())
}