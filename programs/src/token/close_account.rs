use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    ProgramResult
};

use pinocchio_token::instructions::CloseAccount;

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
    process_close_account(accounts, signers)
}

/// Processes the CloseAccount instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to close.
///   1. `[WRITE]` The destination account.
///   2. `[SIGNER]` The account's owner.
pub fn process_close_account<'a>(
    accounts: &'a [AccountInfo],
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [close_account, destination_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys) 
    };

    // Ensure that the 'close' account is writable
    assert!(close_account.is_writable());

    // Ensure that the 'destination' account is writable
    assert!(destination_account.is_writable());

    // Ensure that the 'authority' account is a signer
    assert!(authority_account.is_signer());

    // Creating the instruction instance
    let close_account_instruction = CloseAccount {
        account: close_account,
        destination: destination_account,
        authority: authority_account,
    };

    // Invoking the instruction
    close_account_instruction.invoke_signed(signers)?;

    Ok(())
}