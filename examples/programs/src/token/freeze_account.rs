use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::instructions::FreezeAccount;

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
    // Ensure the data length is sufficient for processing.
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the bump seed from the data.
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(0) as *const [u8; 1]) };

    // Delegate to the `process_freeze_account` function.
    process_freeze_account(accounts, bump)
}

/// Processes the `FreezeAccount` instruction.
///
/// This function handles the logic for freezing a token account. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `bump`: The bump seed for the signer account.
///
/// ### Accounts:
/// 0. `[WRITE]` The account to freeze.
/// 1. `[]` The token mint.
/// 2. `[SIGNER]` The mint freeze authority.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_freeze_account<'a>(
    accounts: &'a [AccountInfo],
    bump: [u8; 1], // Bump seed for the signer account.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [account_to_freeze, mint_account, freeze_authority] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the account to freeze is writable.
    if !account_to_freeze.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the freeze authority is a signer.
    if !freeze_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `FreezeAccount` instruction.
    let freeze_account_instruction = FreezeAccount {
        account: account_to_freeze,
        mint: mint_account,
        freeze_authority,
    };

    // Define the seeds for the signer.
    let seeds = [Seed::from(b"freeze_authority"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the signer seeds.
    freeze_account_instruction.invoke_signed(&signer)?;

    Ok(())
}