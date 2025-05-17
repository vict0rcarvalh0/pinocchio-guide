use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    instruction::{Seed, Signer},
    ProgramResult
};

use pinocchio_system::instructions::Allocate;

const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let space = unsafe { *(data.as_ptr() as *const u64) };
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(8) as *const [u8; 1]) };


    process_allocate(accounts, space, bump)
}

/// Processes the `Allocate` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `space`: The number of bytes to allocate.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The account to allocate space for.
pub fn process_allocate<'a>(
    accounts: &'a [AccountInfo],
    space: u64,
    bump: [u8; 1],
) -> ProgramResult {
    // Extracting account information
    let [allocate_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the allocate account is a signer
    assert!(allocate_account.is_signer());

    // Creating the instruction instance
    let allocate_instruction = Allocate {
        account: allocate_account,
        space,
    };

    let seeds = [Seed::from(b"seeds"), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    // Invoking the instruction
    allocate_instruction.invoke_signed(&signers)?;

    Ok(())
}