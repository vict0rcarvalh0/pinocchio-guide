use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    ProgramResult
};

use pinocchio_token::instructions::TransferChecked;

/// Processes the TransferChecked instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to transfer (in microtokens).
/// - `decimals`: The number of decimal places for the token.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[]` The token mint.
///   2. `[WRITE]` The destination account.
///   3. `[SIGNER]` The source account's owner/delegate.
pub fn process_transfer_checked<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,        // The amount of tokens to transfer.
    decimals: u8,       // The number of decimals for the token.
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [from_account, mint_account, to_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the 'from' account is writable
    assert!(from_account.is_writable(), ProgramError::InvalidAccountData);

    // Ensure the 'to' account is writable
    assert!(to_account.is_writable(), ProgramError::InvalidAccountData);

    // Ensure the authority account is a signer
    assert!(authority_account.is_signer(), ProgramError::MissingRequiredSignature);

    // Creating the instruction instance
    let transfer_checked_instruction = TransferChecked {
        from: from_account,
        mint: mint_account,
        to: to_account,
        authority: authority_account,
        amount,
        decimals,
    };

    // Invoking the instruction
    transfer_checked_instruction.invoke_signed(signers)?;

    Ok(())
}