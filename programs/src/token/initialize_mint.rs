use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::instructions::InitializeMint;

const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    process_initialize_mint(accounts, decimals, mint_authority, freeze_authority, signers)
}

/// Processes the InitializeMint instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `decimals`: Number of decimals for the token.
/// - `mint_authority`: The public key of the mint authority.
/// - `freeze_authority`: An optional public key for the freeze authority.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITABLE]` Mint account.
///   1. `[]` Rent sysvar.
pub fn process_initialize_mint<'a>(
    accounts: &'a [AccountInfo],
    decimals: u8,                   // Decimals for the mint.
    mint_authority: &Pubkey,        // Public key of the mint authority.
    freeze_authority: Option<&Pubkey>, // Optional public key of the freeze authority.
    signers: &[Signer],             // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [mint_account, rent_sysvar] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the mint account is writable
    assert!(mint_account.is_writable());

    // Ensure the rent sysvar is valid (you might need additional checks here)
    assert!(rent_sysvar.key() != &solana_program::sysvar::rent::ID);

    // Creating the instruction instance
    let initialize_mint_instruction = InitializeMint {
        mint: mint_account,
        rent_sysvar,
        decimals,
        mint_authority,
        freeze_authority,
    };

    // Invoking the instruction
    initialize_mint_instruction.invoke_signed(signers)?;

    Ok(())
}