use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::instructions::InitializeMint;

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
    // Ensure the data length is sufficient for processing.
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Call the `process_initialize_mint` function to handle the instruction logic.
    process_initialize_mint(accounts, decimals, mint_authority, freeze_authority, signers)
}

/// Processes the `InitializeMint` instruction.
///
/// This function handles the logic for initializing a mint account. It validates the accounts,
/// constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `decimals`: Number of decimals for the token.
/// - `mint_authority`: The public key of the mint authority.
/// - `freeze_authority`: An optional public key for the freeze authority.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITABLE]` Mint account.
/// 1. `[]` Rent sysvar.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_initialize_mint<'a>(
    accounts: &'a [AccountInfo],
    decimals: u8,                   // Decimals for the mint.
    mint_authority: &Pubkey,        // Public key of the mint authority.
    freeze_authority: Option<&Pubkey>, // Optional public key of the freeze authority.
    signers: &[Signer],             // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [mint_account, rent_sysvar] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the mint account is writable.
    if !mint_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the rent sysvar is valid (you might need additional checks here).
    if rent_sysvar.key() != &solana_program::sysvar::rent::ID {
        return Err(ProgramError::InvalidAccountData);
    }

    // Construct the `InitializeMint` instruction.
    let initialize_mint_instruction = InitializeMint {
        mint: mint_account,
        rent_sysvar,
        decimals,
        mint_authority,
        freeze_authority,
    };

    // Invoke the instruction with the provided signers.
    initialize_mint_instruction.invoke_signed(signers)?;

    Ok(())
}