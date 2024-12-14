use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    instruction::Signer,
    program_error::ProgramError,
    ProgramResult
};

use pinocchio_token::instructions::Burn;

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
    process_burn(accounts, amount, signers)
}

/// Processes the Burn instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to burn.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to burn from.
///   1. `[WRITE]` The token mint.
///   2. `[SIGNER]` The account's owner/delegate.
pub fn process_burn<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,        // Amount of tokens to burn.
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [burn_account, mint_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'burn' account is writable
    if !burn_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'mint' account is writable
    if !mint_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'authority' account is a signer
    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let burn_instruction = Burn {
        account: burn_account,
        mint: mint_account,
        authority: authority_account,
        amount,
    };

    // Invoking the instruction
    burn_instruction.invoke_signed(signers)?;

    Ok(())
}
