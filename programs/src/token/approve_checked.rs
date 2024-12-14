use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    instruction::Signer,
    program_error::ProgramError,
    ProgramResult
};

use pinocchio_token::instructions::ApproveChecked;

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
    process_approve_checked(accounts, amount, decimals, signers)
}

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
    accounts: &'a [AccountInfo],
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
    );

    // Ensure that the 'authority' account is a signer
    assert!(
        authority_account.is_signer()
    );

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