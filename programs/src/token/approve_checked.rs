use pinocchio::{
    account_info::AccountInfo, entrypoint::ProgramResult, instruction::Signer,
    program_error::ProgramError,
};

use pinocchio_token::instructions::ApproveChecked;

/// Processes the ApproveChecked instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to approve.
/// - `decimals`: The number of decimals for the token.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[]` The token mint.
///   2. `[]` The delegate account.
///   3. `[SIGNER]` The source account owner.
pub fn process_approve_checked<'a>(
    accounts: &'a [AccountInfo<'a>],
    amount: u64,        // Amount of tokens to approve.
    decimals: u8,       // Token decimals for validation.
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [source_account, mint_account, delegate_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'source' account is writable
    assert!(
        source_account.is_writable(),
        ProgramError::InvalidAccountData
    )

    // Ensure that the 'authority' account is a signer
    assert!(
        authority_account.is_signer()
        ProgramError::MissingRequiredSignature
    )

    // Creating the instruction instance
    let approve_checked_instruction = ApproveChecked {
        source: source_account,
        mint: mint_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,
        decimals,
    };

    // Invoking the instruction
    approve_checked_instruction.invoke_signed(signers)?;

    Ok(())
}