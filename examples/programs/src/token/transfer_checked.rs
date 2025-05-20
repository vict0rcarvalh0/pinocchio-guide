use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::TransferChecked;

// A constant representing the program ID, decoded from a base58 string.
const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");

// Macro to define the program's entry point.
entrypoint!(process_instruction);

/// Entry point for the program. This function is called when the program is invoked.
///
/// ### Parameters:
/// - `_program_id`: The ID of the program being executed.
/// - `accounts`: The accounts passed to the program.
/// - `data`: Additional data passed to the program.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the program execution.
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Validate the length of the data buffer.
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the amount, decimals, and bump from the data buffer.
    let amount = unsafe { *(data.as_ptr().add(0) as *const u64) };
    let decimals = unsafe { *(data.as_ptr().add(8) as *const u8) };
    let bump = unsafe { *(data.as_ptr().add(9) as *const [u8; 1]) };

    // Process the TransferChecked instruction.
    process_transfer_checked(accounts, amount, decimals, bump)
}

/// Processes the `TransferChecked` instruction.
///
/// This function handles the logic for transferring tokens with a specified amount and decimals.
/// It validates the accounts and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to transfer (in microtokens).
/// - `decimals`: The number of decimal places for the token.
/// - `bump`: The bump seed for the signer.
///
/// ### Accounts:
/// 0. `[WRITE]` The source account.
/// 1. `[]` The token mint.
/// 2. `[WRITE]` The destination account.
/// 3. `[SIGNER]` The source account's owner/delegate.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_transfer_checked<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,        // The amount of tokens to transfer.
    decimals: u8,       // The number of decimals for the token.
    bump: [u8; 1],      // The bump seed for the signer.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [from_account, mint_account, to_account, authority_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the 'from' account is writable.
    if !from_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the 'to' account is writable.
    if !to_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the authority account is a signer.
    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `TransferChecked` instruction.
    let transfer_checked_instruction = TransferChecked {
        from: from_account,
        mint: mint_account,
        to: to_account,
        authority: authority_account,
        amount,
        decimals,
    };

    // Create the signer seeds array.
    let seeds = [Seed::from(b"authority_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the signer seeds.
    transfer_checked_instruction.invoke_signed(&signer)?;

    Ok(())
}