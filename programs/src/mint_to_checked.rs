use pinocchio::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::Signer,
    pubkey::Pubkey,
    program_error::ProgramError,
};

use pinocchio_token::instructions::MintToChecked;

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
    accounts: &'a [AccountInfo<'a>],
    amount: u64,            // Amount of tokens to mint.
    decimals: u8,           // Number of decimal places.
    signers: &[Signer],     // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [mint_account, token_account, mint_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the mint account is writable
    assert!(mint_account.is_writable(), ProgramError::InvalidAccountData);

    // Ensure the token account is writable
    assert!(token_account.is_writable(), ProgramError::InvalidAccountData);


    // Ensure the mint authority is a signer
    assert!(mint_authority.is_signer(), ProgramError::MissingRequiredSignature);


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