use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::TransferChecked;

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
    let decimals = unsafe { *(data.as_ptr().add(8) as *const u8) };
    let bump = unsafe { *(data.as_ptr().add(9) as *const [u8; 1]) };
    process_transfer_checked(accounts, amount, decimals, bump)
}

/// Processes the TransferChecked instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to transfer (in microtokens).
/// - `decimals`: The number of decimal places for the token.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[]` The token mint.
///   2. `[WRITE]` The destination account.
///   3. `[SIGNER]` The source account's owner/delegate.
pub fn process_transfer_checked<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,        // The amount of tokens to transfer.
    decimals: u8,       // The number of decimals for the token.
    bump: [u8; 1], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [from_account, mint_account, to_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the 'from' account is writable
    assert!(from_account.is_writable());

    // Ensure the 'to' account is writable
    assert!(to_account.is_writable());

    // Ensure the authority account is a signer
    assert!(authority_account.is_signer());

    // Creating the instruction instance
    let transfer_checked_instruction = TransferChecked {
        from: from_account,
        mint: mint_account,
        to: to_account,
        authority: authority_account,
        amount,
        decimals,
    };

    let seeds = [Seed::from(b"authority_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoking the instruction
    transfer_checked_instruction.invoke_signed(&signer)?;

    Ok(())
}