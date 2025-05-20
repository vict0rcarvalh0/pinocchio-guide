use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_system::instructions::TransferWithSeed;

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
    // Ensure the data length is sufficient for parsing.
    if data.len() < 41 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Parse the lamports to transfer from the data.
    let lamports = unsafe { *(data.as_ptr() as *const u64) };

    // Parse the seed length from the data.
    let seed_len = unsafe { *(data.as_ptr().add(8) as *const u8) } as usize;

    // Ensure the data length is sufficient for the seed, owner, and bump.
    if data.len() < 9 + seed_len + 32 + 1 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the seed from the data.
    let seed = unsafe {
        std::str::from_utf8_unchecked(&data[9..9 + seed_len])
    };

    // Extract the owner public key from the data.
    let owner_offset = 9 + seed_len;
    let owner = unsafe { *(data.as_ptr().add(owner_offset) as *const Pubkey) };

    // Extract the bump seed from the data.
    let bump_offset = owner_offset + 32;
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(bump_offset) as *const [u8; 1]) };

    // Process the transfer with seed instruction.
    process_transfer_with_seed(accounts, lamports, seed, &owner, bump)
}

/// Processes the `TransferWithSeed` instruction.
///
/// This function handles the logic for transferring lamports using a derived account.
/// It validates the accounts and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer.
/// - `seed`: The seed used to derive the source account.
/// - `owner`: The program that owns the source account.
/// - `bump`: The bump seed used to derive the source account.
///
/// ### Accounts:
/// 0. `[WRITE]` The source account.
/// 1. `[SIGNER]` The base account used to derive the source account.
/// 2. `[WRITE]` The destination account.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_transfer_with_seed<'a>(
    accounts: &'a [AccountInfo],
    lamports: u64,        // The amount of lamports to transfer.
    seed: &'a str,        // The seed used to derive the address of the funding account.
    owner: &'a Pubkey,    // The address of the program that will own the new account.
    bump: [u8; 1],        // The bump seed used to derive the source account.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [from_account, base_account, to_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'from' account is writable.
    if !from_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'base' account is a signer.
    if !base_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure that the 'to' account is writable.
    if !to_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Construct the `TransferWithSeed` instruction.
    let transfer_instruction = TransferWithSeed {
        from: from_account,
        base: base_account,
        to: to_account,
        lamports,
        seed,
        owner,
    };

    // Create the seeds and signers for the instruction.
    let seeds = [Seed::from(b"seed"), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    // Invoke the instruction with the provided signers.
    transfer_instruction.invoke_signed(&signers)?;

    Ok(())
}