use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_system::instructions::TransferWithSeed;

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
    process_transfer_with_seed(accounts, lamports, seed, owner, signers)
}

/// Processes the `TransferWithSeed` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer.
/// - `seed`: The seed used to derive the source account.
/// - `owner`: The program that owns the source account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The source account.
/// 1. `[SIGNER]` The base account used to derive the source account.
/// 2. `[WRITE]` The destination account.
pub fn process_transfer_with_seed<'a>(
    accounts: &'a [AccountInfo],
    lamports: u64,        //  The amount of lamports to transfer.
    seed: &'a str,        // The seed used to derive the address of the funding account.
    owner: &'a Pubkey,    // The address of the program that will own the new account.
    signers: &[Signer],   // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [from_account, base_account, to_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'from' account is writable
    assert!(from_account.is_writable());

    // Ensure that the 'base' account is a signer
    assert!(base_account.is_signer());

    // Ensure that the 'to' account is writable
    assert!(to_account.is_writable());

    // Creating the instruction instance
    let transfer_instruction = TransferWithSeed {
        from: from_account,
        base: base_account,
        to: to_account,
        lamports,
        seed,
        owner,
    };

    // Invoking the instruction
    transfer_instruction.invoke_signed(signers)?;

    Ok(())
}