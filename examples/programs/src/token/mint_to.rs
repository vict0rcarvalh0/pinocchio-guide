use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::instructions::MintTo;

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
    // Ensure the data length is sufficient to extract the required fields.
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the amount to mint from the data.
    let amount = unsafe { *(data.as_ptr().add(0) as *const u64) };

    // Extract the bump seed from the data.
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(8) as *const [u8; 1]) };

    // Process the MintTo instruction.
    process_mint_to(accounts, amount, bump)
}

/// Processes the MintTo instruction.
///
/// This function handles the logic for minting tokens. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to mint.
/// - `bump`: The bump seed for the signer account.
///
/// ### Accounts:
/// 0. `[WRITE]` The mint account.
/// 1. `[WRITE]` The account to mint tokens to.
/// 2. `[SIGNER]` The mint's minting authority.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_mint_to(
    accounts: &[AccountInfo],
    amount: u64,   // Amount of tokens to mint.
    bump: [u8; 1], // Bump seed for the signer account.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [mint_account, token_account, mint_authority, _token_program] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the mint account is writable.
    assert!(mint_account.is_writable(), "Mint account is not writable");

    // Ensure the token account is writable.
    assert!(token_account.is_writable(), "Token account is not writable");

    // Ensure the mint authority is a signer.
    assert!(mint_authority.is_signer(), "Mint authority is not a signer");

    // Construct the MintTo instruction.
    let mint_to_instruction = MintTo {
        mint: mint_account,
        account: token_account,
        mint_authority,
        amount,
    };

    // Construct the signer seeds.
    let seeds = [Seed::from(b"mint_authority"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the signer.
    mint_to_instruction.invoke_signed(&signer)?;

    Ok(())
}
