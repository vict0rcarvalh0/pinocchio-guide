use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_system::instructions::AuthorizeNonceAccount;

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
    // Ensure the data length is sufficient for the instruction.
    if data.len() < 33 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the new authority's public key from the instruction data.
    let new_authority = unsafe { *(data.as_ptr() as *const Pubkey) };

    // Extract the bump seed from the instruction data.
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(32) as *const [u8; 1]) };

    // Process the `AuthorizeNonceAccount` instruction.
    process_authorize_nonce_account(accounts, &new_authority, bump)
}

/// Processes the `AuthorizeNonceAccount` instruction.
///
/// This function handles the logic for authorizing a new entity to execute nonce instructions
/// on the specified nonce account. It validates the accounts and signers, constructs the
/// instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `new_authority`: The public key of the new authority.
/// - `bump`: The bump seed used for generating the program-derived address.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[SIGNER]` The current Nonce authority.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_authorize_nonce_account<'a>(
    accounts: &'a [AccountInfo],
    new_authority: &Pubkey,  // Pubkey of the new entity to be authorized to execute nonce instructions on the account.
    bump: [u8; 1],
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [nonce_account, nonce_authority] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the nonce authority is a signer.
    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `AuthorizeNonceAccount` instruction.
    let authorize_nonce_instruction = AuthorizeNonceAccount {
        account: nonce_account,
        authority: nonce_authority,
        new_authority,
    };

    // Create the seeds array for the program-derived address.
    let seeds = [Seed::from(b"nonce_authority"), Seed::from(&bump)];

    // Create the signer array using the seeds.
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the signer.
    authorize_nonce_instruction.invoke_signed(&signer)?;

    Ok(())
}