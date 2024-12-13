use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::Signer,
    program_error::ProgramError,
    ProgramResult
};

use pinocchio_token::instructions::Approve;

/// Processes the Approve instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to approve.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The token account.
///   1. `[]` The delegate account.
///   2. `[SIGNER]` The source account owner.
pub fn process_approve<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,        // Amount of tokens to approve.
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [source_account, delegate_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'source' account is writable
    assert!(
        source_account.is_writable(),
        ProgramError::InvalidAccountData
    );

    // Ensure that the 'authority' account is a signer
    assert!(
        authority_account.is_signer(),
        ProgramError::MissingRequiredSignature
    );

    // Creating the instruction instance
    let approve_instruction = Approve {
        source: source_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,
    };

    // Invoking the instruction
    approve_instruction.invoke_signed(signers)?;

    Ok(())
}
