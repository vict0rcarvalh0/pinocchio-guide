use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_system::instructions::AuthorizeNonceAccount;

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
    let new_authority = unsafe { *(data.as_ptr() as *const Pubkey) };
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(32) as *const [u8; 1]) };
    process_authorize_nonce_account(accounts, &new_authority, bump)
}

/// Processes the `AuthorizeNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `new_authority`: The public key of the new authority.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[SIGNER]` The current Nonce authority.
pub fn process_authorize_nonce_account<'a>(
    accounts: &'a [AccountInfo],
    new_authority: &Pubkey,  // Pubkey of the new entity to be authorized to execute nonce instructions on the account.
    bump: [u8; 1],
) -> ProgramResult {
    // Extracting account information
    let [nonce_account, nonce_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the nonce authority is a signer
    assert!(nonce_authority.is_signer());

    // Creating the instruction instance
    let authorize_nonce_instruction = AuthorizeNonceAccount {
        account: nonce_account,
        authority: nonce_authority,
        new_authority,
    };

    let seeds = [Seed::from(b"nonce_authority"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoking the instruction
    authorize_nonce_instruction.invoke_signed(&signer)?;

    Ok(())
}