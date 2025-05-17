use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::Signer,
    pubkey::Pubkey,
    program_error::ProgramError,
    ProgramResult
};

use pinocchio_token::instructions::MintToChecked;

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
    let amount = unsafe { *(data.as_ptr() as *const u64) };
    let decimals = unsafe { *(data.as_ptr().add(8) as *const u8) };
    process_mint_to_checked(accounts, amount, decimals, signers)
}

/// Processes the MintToChecked instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to mint.
/// - `decimals`: The number of decimal places for the tokens.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The mint account.
///   1. `[WRITE]` The account to mint tokens to.
///   2. `[SIGNER]` The mint's minting authority.
pub fn process_mint_to_checked<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,            // Amount of tokens to mint.
    decimals: u8,           // Number of decimal places.
    signers: &[Signer],     // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [mint_account, token_account, mint_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the mint account is writable
    assert!(mint_account.is_writable());

    // Ensure the token account is writable
    assert!(token_account.is_writable());

    // Ensure the mint authority is a signer
    assert!(mint_authority.is_signer());

    // Creating the instruction instance
    let mint_to_checked_instruction = MintToChecked {
        mint: mint_account,
        account: token_account,
        mint_authority,
        amount,
        decimals,
    };

    // Invoking the instruction
    mint_to_checked_instruction.invoke_signed(signers)?;

    Ok(())
}