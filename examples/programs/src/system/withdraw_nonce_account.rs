use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_system::instructions::WithdrawNonceAccount;

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
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(0) as *const [u8; 1]) };
    let lamports_to_withdraw = unsafe { *(data.as_ptr().add(1) as *const u64) };
    process_withdraw_nonce_account(accounts, bump, lamports_to_withdraw)
}

/// Processes the `WithdrawNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports_to_withdraw`: The number of lamports to withdraw.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[WRITE]` The recipient account.
/// 2. `[]` The recent blockhashes sysvar.
/// 3. `[]` The rent sysvar.
/// 4. `[SIGNER]` The Nonce authority.
pub fn process_withdraw_nonce_account<'a>(
    accounts: &'a [AccountInfo],
    bump: [u8; 1],
    lamports_to_withdraw: u64,   // The amount of lamports to withdraw.
) -> ProgramResult {
    // Extracting account information
    let [nonce_account, recipient_account, recent_blockhashes_sysvar, rent_sysvar, nonce_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the necessary accounts are writable or readonly as required
    assert!(nonce_account.is_writable() || recipient_account.is_writable());

    // Ensure the nonce authority is a signer
    assert!(nonce_authority.is_signer());

    // Creating the instruction instance
    let withdraw_nonce_instruction = WithdrawNonceAccount {
        account: nonce_account,
        recipient: recipient_account,
        recent_blockhashes_sysvar,
        rent_sysvar,
        authority: nonce_authority,
        lamports: lamports_to_withdraw,
    };

    let seeds = [Seed::from(b"nonce_authority"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];
    // Invoking the instruction
    withdraw_nonce_instruction.invoke_signed(&signer)?;

    Ok(())
}