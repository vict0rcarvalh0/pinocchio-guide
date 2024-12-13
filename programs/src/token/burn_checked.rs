use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    instruction::Signer,
    program_error::ProgramError,
    ProgramResult
};

use pinocchio_token::instructions::BurnChecked;

/// Processes the BurnChecked instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to burn.
/// - `decimals`: The decimals for the token being burned.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to burn from.
///   1. `[WRITE]` The token mint.
///   2. `[SIGNER]` The account's owner/delegate.
pub fn process_burn_checked<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,        // Amount of tokens to burn.
    decimals: u8,       // Number of decimals for the token.
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [burn_account, mint_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'burn' account is writable
    assert!(burn_account.is_writable(), ProgramError::InvalidAccountData);

    // Ensure that the 'mint' account is writable
    assert!(mint_account.is_writable(), ProgramError::InvalidAccountData);

    // Ensure that the 'authority' account is a signer
    assert!(
        authority_account.is_signer(),
        ProgramError::MissingRequiredSignature
    );

    // Creating the instruction instance
    let burn_checked_instruction = BurnChecked {
        account: burn_account,
        mint: mint_account,
        authority: authority_account,
        amount,
        decimals,
    };

    // Invoking the instruction
    burn_checked_instruction.invoke_signed(signers)?;

    Ok(())
}
