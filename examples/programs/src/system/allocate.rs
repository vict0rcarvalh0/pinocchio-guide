use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    instruction::Signer,
    ProgramResult
};

use pinocchio_system::instructions::Allocate;

const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
    signers: &[Signer],
) -> ProgramResult {
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let space = unsafe { *(data.as_ptr() as *const u64) };

    process_allocate(accounts, space, signers)
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
    space: u64,                       // Determines how many bytes of memory are allocated for the account.
    signers: &[Signer],
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

    // Invoking the instruction
    allocate_instruction.invoke_signed(signers)?;

    Ok(())
}