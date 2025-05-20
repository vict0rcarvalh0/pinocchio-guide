use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_system::instructions::CreateAccountWithSeed;

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
    if data.len() < 41 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the seed length from the instruction data.
    let seed_len = unsafe { *(data.as_ptr() as *const u8) } as usize;

    // Validate the total length of the instruction data.
    if data.len() < 1 + seed_len + 8 + 8 + 32 + 1 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the seed string from the instruction data.
    let seed = unsafe { std::str::from_utf8_unchecked(&data[1..1 + seed_len]) };

    // Extract the lamports value from the instruction data.
    let lamports_offset = 1 + seed_len;
    let lamports = unsafe { *(data.as_ptr().add(lamports_offset) as *const u64) };

    // Extract the space value from the instruction data.
    let space_offset = lamports_offset + 8;
    let space = unsafe { *(data.as_ptr().add(space_offset) as *const u64) };

    // Extract the owner public key from the instruction data.
    let owner_offset = space_offset + 8;
    let owner = unsafe { *(data.as_ptr().add(owner_offset) as *const Pubkey) };

    // Extract the bump seed from the instruction data.
    let bump_offset = owner_offset + 32;
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(bump_offset) as *const [u8; 1]) };

    // Process the `CreateAccountWithSeed` instruction.
    process_create_account_with_seed(accounts, seed, lamports, space, &owner, bump)
}

/// Processes the `CreateAccountWithSeed` instruction.
///
/// This function handles the logic for creating an account with a seed. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `seed`: The seed used to derive the account.
/// - `lamports`: The number of lamports to transfer to the new account.
/// - `space`: The number of bytes to allocate for the new account.
/// - `owner`: The program that will own the new account.
/// - `bump`: The bump seed used for address derivation.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The funding account.
/// 1. `[WRITE, SIGNER]` The new account to be created.
/// 2. `[OPTIONAL]` The base account used to derive the new account (if applicable).
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_create_account_with_seed<'a>(
    accounts: &'a [AccountInfo],
    seed: &'a str,      // The ASCII string that will be used as the seed to derive the address.
    lamports: u64,      // Number of lamports to transfer to the new account.
    space: u64,         // Number of bytes to allocate for the new account.
    owner: &Pubkey,     // Pubkey of the program that will own the new account.
    bump: [u8; 1],      // The bump seed used for address derivation.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [funding_account, new_account, base_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the funding account or the new account is a signer.
    assert!(funding_account.is_signer() || new_account.is_signer());

    // Construct the `CreateAccountWithSeed` instruction.
    let create_account_with_seed_instruction = CreateAccountWithSeed {
        from: funding_account,
        to: new_account,
        base: Some(base_account),
        seed,
        lamports,
        space,
        owner,
    };

    // Prepare the seeds and signer for the instruction.
    let seeds = [Seed::from(b"funding_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the provided signer.
    create_account_with_seed_instruction.invoke_signed(&signer)?;

    Ok(())
}