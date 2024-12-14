use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_system::instructions::CreateAccount;

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
    process_create_account(accounts, lamports, space, owner, signers)
}

/// Processes the `CreateAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer to the new account.
/// - `space`: The number of bytes to allocate for the new account.
/// - `owner`: The program that will own the new account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The funding account.
/// 1. `[WRITE, SIGNER]` The new account to be created.
pub fn process_create_account<'a>(
    accounts: &'a [AccountInfo],
    lamports: u64,   // Number of lamports to transfer to the new account.
    space: u64,      // Number of bytes to allocate for the new account.
    owner: &Pubkey,  // Pubkey of the program that will own the new account.
    signers: &[Signer],
) -> ProgramResult {
    // Extracting account information
    let [funding_account, new_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the funding account and new account are signers
    assert!(funding_account.is_signer() || new_account.is_signer());

    // Creating the instruction instance
    let create_account_instruction = CreateAccount {
        from: funding_account,
        to: new_account,
        lamports,
        space,
        owner,
    };

    // Invoking the instruction
    create_account_instruction.invoke_signed(signers)?;

    Ok(())
}