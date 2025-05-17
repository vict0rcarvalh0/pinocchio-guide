use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{Signer, Seed},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::Approve;

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
    let amount = unsafe { *(data.as_ptr().add(0) as *const u64) };
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(8) as *const [u8; 1]) };
    process_approve(accounts, amount, bump)
}

/// Processes the Approve instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to approve.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The token account.
///   1. `[]` The delegate account.
///   2. `[SIGNER]` The source account owner.
pub fn process_approve<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,        // Amount of tokens to approve.
    bump: [u8; 1], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [source_account, delegate_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'source' account is writable
    assert!(
        source_account.is_writable(),
    );

    // Ensure that the 'authority' account is a signer
    assert!(
        authority_account.is_signer(),
    );

    // Creating the instruction instance
    let approve_instruction = Approve {
        source: source_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,
    };

    let seeds = [Seed::from(b"authority_account"), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    // Invoking the instruction
    approve_instruction.invoke_signed(&signers)?;

    Ok(())
}
