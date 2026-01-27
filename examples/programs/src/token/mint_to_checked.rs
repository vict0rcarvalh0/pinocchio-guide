use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::instructions::MintToChecked;

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
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the amount, decimals, and bump seed from the instruction data.
    let amount = unsafe { *(data.as_ptr() as *const u64) };
    let decimals = unsafe { *(data.as_ptr().add(8) as *const u8) };
    let bump = unsafe { *(data.as_ptr().add(9) as *const [u8; 1]) };

    // Process the MintToChecked instruction.
    process_mint_to_checked(accounts, amount, decimals, bump)
}

/// Processes the `MintToChecked` instruction.
///
/// This function handles the logic for minting tokens to a specified account. It validates
/// the accounts and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to mint.
/// - `decimals`: The number of decimal places for the tokens.
/// - `bump`: The bump seed for the signer account.
///
/// ### Accounts:
/// 0. `[WRITE]` The mint account.
/// 1. `[WRITE]` The account to mint tokens to.
/// 2. `[SIGNER]` The mint's minting authority.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_mint_to_checked<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,            // Amount of tokens to mint.
    decimals: u8,           // Number of decimal places.
    bump: [u8; 1],          // Bump seed for the signer account.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [mint_account, token_account, mint_authority] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the mint account is writable.
    if !mint_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the token account is writable.
    if !token_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the mint authority is a signer.
    if !mint_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `MintToChecked` instruction.
    let mint_to_checked_instruction = MintToChecked {
        mint: mint_account,
        account: token_account,
        mint_authority,
        amount,
        decimals,
    };

    // Create the seeds and signers for the instruction.
    let seeds = [Seed::from(b"mint_authority"), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    // Invoke the instruction with the provided signers.
    mint_to_checked_instruction.invoke_signed(&signers)?;

    Ok(())
}