use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    instruction::{Signer, Seed},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::BurnChecked;

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
    process_burn_checked(accounts, amount, decimals, bump)
}

/// Processes the BurnChecked instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to burn.
/// - `decimals`: The decimals for the token being burned.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to burn from.
///   1. `[WRITE]` The token mint.
///   2. `[SIGNER]` The account's owner/delegate.
pub fn process_burn_checked<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,        // Amount of tokens to burn.
    decimals: u8,       // Number of decimals for the token.
    bump: [u8; 1], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [burn_account, mint_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'burn' account is writable
    assert!(burn_account.is_writable());

    // Ensure that the 'mint' account is writable
    assert!(mint_account.is_writable());

    // Ensure that the 'authority' account is a signer
    assert!(
        authority_account.is_signer(),
    );

    // Creating the instruction instance
    let burn_checked_instruction = BurnChecked {
        account: burn_account,
        mint: mint_account,
        authority: authority_account,
        amount,
        decimals,
    };

    let seeds = [Seed::from(b"authority_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoking the instruction
    burn_checked_instruction.invoke_signed(&signer)?;

    Ok(())
}
