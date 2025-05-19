use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_system::instructions::UpdateNonceAccount;

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
    process_update_nonce_account(accounts, bump)
}

/// Processes the `UpdateNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
pub fn process_update_nonce_account<'a>(
    accounts: &'a [AccountInfo],
    bump: [u8; 1],  
) -> ProgramResult {
    // Extracting account information
    let [nonce_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'nonce_account' is writable
    assert!(nonce_account.is_writable());

    // Creating the instruction instance
    let update_nonce_instruction = UpdateNonceAccount {
        account: nonce_account,
    };

    let seeds = [Seed::from(b"seeds"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoking the instruction
    update_nonce_instruction.invoke_signed(&signer)?;

    Ok(())
}