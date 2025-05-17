use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{Signer, Seed},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::Revoke;

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
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(0) as *const [u8; 1]) };
    process_revoke(accounts, bump)
}

/// Processes the Revoke instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[SIGNER]` The source account owner.
pub fn process_revoke<'a>(
    accounts: &'a [AccountInfo],
    bump: [u8; 1], // The signers array for authorization.
) -> ProgramResult {
    // Extracting account information
    let [source_account, owner_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the source account is writable
    assert!(source_account.is_writable());

    // Ensure the owner account is a signer
    assert!(owner_account.is_signer());

    // Creating the instruction instance
    let revoke_instruction = Revoke {
        source: source_account,
        authority: owner_account,
    };

    let seeds = [Seed::from(b"owner_account"), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    // Invoking the instruction
    revoke_instruction.invoke_signed(&signers)?;

    Ok(())
}