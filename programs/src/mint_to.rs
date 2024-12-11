use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Signer,
    pubkey::Pubkey,
    program_error::ProgramError,
};

use pinocchio::instructions::MintTo;

/// Processes the MintTo instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to mint.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The mint account.
///   1. `[WRITE]` The account to mint tokens to.
///   2. `[SIGNER]` The mint's minting authority.
pub fn process_mint_to<'a>(
    accounts: &'a [AccountInfo<'a>],
    amount: u64,            // Amount of tokens to mint.
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
    let mint_to_instruction = MintTo {
        mint: mint_account,
        account: token_account,
        mint_authority,
        amount,
    };

    // Invoking the instruction
    mint_to_instruction.invoke_signed(signers)?;

    Ok(())
}