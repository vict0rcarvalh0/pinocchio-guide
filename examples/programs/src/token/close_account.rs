use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
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

    let bump = unsafe { *(data.as_ptr().add(0) as *const [u8; 1]) };

    process_close_account(accounts, bump)
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
    bump: [u8; 1], // Bump seed for the signer account.
) -> ProgramResult {
    // Extracting account information
    let [close_account, destination_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys) 
    };

    // Ensure that the 'close' account is writable
    assert!(close_account.is_writable(), "Clsoe account is not writable");

    // Ensure that the 'destination' account is writable
    assert!(destination_account.is_writable(), "Destination account is not writable");

    // Ensure that the 'authority' account is a signer
    assert!(authority_account.is_signer(), "Authority account is not writable");

    // Creating the instruction instance
    let close_account_instruction = CloseAccount {
        account: close_account,
        destination: destination_account,
        authority: authority_account,
    };

    let seeds = [Seed::from(b"authority_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoking the instruction
    close_account_instruction.invoke_signed(&signer)?;

    Ok(())
}