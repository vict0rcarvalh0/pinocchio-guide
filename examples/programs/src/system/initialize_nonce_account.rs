use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_system::instructions::InitializeNonceAccount;

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
    // Check if the data length is valid.
    if data.len() < 33 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the authority public key from the data.
    let authority = unsafe { *(data.as_ptr() as *const Pubkey) };

    // Extract the bump seed from the data.
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(32) as *const [u8; 1]) };

    // Process the `InitializeNonceAccount` instruction.
    process_initialize_nonce_account(accounts, &authority, bump)
}

/// Processes the `InitializeNonceAccount` instruction.
///
/// This function handles the logic for initializing a nonce account. It validates the accounts,
/// constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `authority`: The public key of the entity authorized to manage the Nonce account.
/// - `bump`: The bump seed used for the nonce account.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[]` The recent blockhashes sysvar.
/// 2. `[]` The rent sysvar.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_initialize_nonce_account<'a>(
    accounts: &'a [AccountInfo],
    authority: &'a Pubkey,   // Pubkey representing the entity authorized to interact with the nonce account.
    bump: [u8; 1],
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [nonce_account, recent_blockhashes_sysvar, rent_sysvar] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the nonce account is writable.
    assert!(nonce_account.is_writable());

    // Construct the `InitializeNonceAccount` instruction.
    let initialize_nonce_account_instruction = InitializeNonceAccount {
        account: nonce_account,
        recent_blockhashes_sysvar,
        rent_sysvar,
        authority,
    };

    // Create the seeds for the signer.
    let seeds = [Seed::from(b"nonce_account"), Seed::from(&bump)];

    // Create the signer using the seeds.
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the signer.
    initialize_nonce_account_instruction.invoke_signed(&signer)?;

    Ok(())
}