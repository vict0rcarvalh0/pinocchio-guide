use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_system::instructions::Assign;

const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 33 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let owner = unsafe { *(data.as_ptr() as *const Pubkey) };
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(32) as *const [u8; 1]) };
    process_assign(accounts, &owner, bump)
}

/// Processes the `Assign` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `owner`: The public key of the new program owner.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The account to be reassigned to a new program owner.
pub fn process_assign<'a>(
    accounts: &'a [AccountInfo],
    owner: &Pubkey,      // Public key of the program to assign as the new owner of the account.
    bump: [u8; 1],
) -> ProgramResult {
    // Extracting account information
    let [assigned_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    
    // Ensure the assigned account is a signer
    assert!(assigned_account.is_signer());

    // Creating the instruction instance
    let assign_instruction = Assign {
        account: assigned_account,
        owner,
    };

    let seeds = [Seed::from(b"assigned_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoking the instruction
    assign_instruction.invoke_signed(&signer)?;

    Ok(())
}