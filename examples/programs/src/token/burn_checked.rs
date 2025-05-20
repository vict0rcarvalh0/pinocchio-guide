use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    instruction::{Signer, Seed},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::BurnChecked;

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
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Parse the amount, decimals, and bump from the data buffer.
    let amount = unsafe { *(data.as_ptr().add(0) as *const u64) };
    let decimals = unsafe { *(data.as_ptr().add(8) as *const u8) };
    let bump = unsafe { *(data.as_ptr().add(9) as *const [u8; 1]) };

    // Process the BurnChecked instruction.
    process_burn_checked(accounts, amount, decimals, bump)
}

/// Processes the `BurnChecked` instruction.
///
/// This function handles the logic for burning tokens. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to burn.
/// - `decimals`: The decimals for the token being burned.
/// - `bump`: The bump seed for the authority.
///
/// ### Accounts:
/// 0. `[WRITE]` The account to burn from.
/// 1. `[WRITE]` The token mint.
/// 2. `[SIGNER]` The account's owner/delegate.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_burn_checked<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,        // Amount of tokens to burn.
    decimals: u8,       // Number of decimals for the token.
    bump: [u8; 1],      // The bump seed for the authority.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [burn_account, mint_account, authority_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'burn' account is writable.
    if !burn_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'mint' account is writable.
    if !mint_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'authority' account is a signer.
    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `BurnChecked` instruction.
    let burn_checked_instruction = BurnChecked {
        account: burn_account,
        mint: mint_account,
        authority: authority_account,
        amount,
        decimals,
    };

    // Create the seeds and signer for the instruction.
    let seeds = [Seed::from(b"authority_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the signer.
    burn_checked_instruction.invoke_signed(&signer)?;

    Ok(())
}
