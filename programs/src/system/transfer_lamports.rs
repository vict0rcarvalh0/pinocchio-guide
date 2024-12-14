use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    ProgramResult
};

use pinocchio_system::instructions::Transfer;

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
    process_transfer(accounts, lamports, signers)
}

/// Processes the `Transfer` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The source account.
/// 1. `[WRITE]` The destination account.
pub fn process_transfer<'a>(
    accounts: &'a [AccountInfo],
    lamports: u64,        // The amount of lamports to transfer.
    signers: &[Signer],   // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [from_account, to_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'from' account is writable and a signer
    assert!(from_account.is_writable() || from_account.is_signer());

    // Ensure that the 'to' account is writable
    assert!(to_account.is_writable());

    // Creating the instruction instance
    let transfer_instruction = Transfer {
        from: from_account,
        to: to_account,
        lamports,
    };

    // Invoking the instruction
    transfer_instruction.invoke_signed(signers)?;

    Ok(())
}