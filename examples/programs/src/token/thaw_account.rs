use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::ThawAccount;

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
    process_thaw_account(accounts)
}

/// Processes the ThawAccount instruction.
///
/// ### Parameters:
/// - `accounts`: List of accounts involved in the instruction.
///
/// ### Accounts:
///   0. `[WRITE]` The token account to be thawed.
///   1. `[]` The token mint associated with the account.
///   2. `[SIGNER]` The freeze authority for the mint.
pub fn process_thaw_account<'a>(
    accounts: &'a [AccountInfo],
) -> ProgramResult {
    // Iterate over the provided accounts
    let [token_account, mint_account, freeze_authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate that the token account is writable
    assert!(token_account.is_writable());

    // Validate the freeze authority is a signer
    assert!(freeze_authority_account.is_signer());

    // Construct the ThawAccount instruction
    let thaw_account_instruction = ThawAccount {
        account: token_account,
        mint: mint_account,
        freeze_authority: freeze_authority_account,
    };

    // Invoke the instruction
    thaw_account_instruction.invoke()
}