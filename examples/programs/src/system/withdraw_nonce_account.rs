use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::{Signer, Seed},
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_system::instructions::WithdrawNonceAccount;

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
    // Ensure the data length is sufficient for processing.
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the bump seed and lamports to withdraw from the data.
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(0) as *const [u8; 1]) };
    let lamports_to_withdraw = unsafe { *(data.as_ptr().add(1) as *const u64) };

    // Call the function to process the `WithdrawNonceAccount` instruction.
    process_withdraw_nonce_account(accounts, bump, lamports_to_withdraw)
}

/// Processes the `WithdrawNonceAccount` instruction.
///
/// This function handles the logic for withdrawing lamports from a nonce account. It validates
/// the accounts and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `bump`: The bump seed for the nonce authority.
/// - `lamports_to_withdraw`: The number of lamports to withdraw.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[WRITE]` The recipient account.
/// 2. `[]` The recent blockhashes sysvar.
/// 3. `[]` The rent sysvar.
/// 4. `[SIGNER]` The Nonce authority.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_withdraw_nonce_account<'a>(
    accounts: &'a [AccountInfo],
    bump: [u8; 1],
    lamports_to_withdraw: u64,
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [nonce_account, recipient_account, recent_blockhashes_sysvar, rent_sysvar, nonce_authority] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the necessary accounts are writable or readonly as required.
    if !nonce_account.is_writable() || !recipient_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the nonce authority is a signer.
    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `WithdrawNonceAccount` instruction.
    let withdraw_nonce_instruction = WithdrawNonceAccount {
        account: nonce_account,
        recipient: recipient_account,
        recent_blockhashes_sysvar,
        rent_sysvar,
        authority: nonce_authority,
        lamports: lamports_to_withdraw,
    };

    // Create the seeds and signer for the instruction.
    let seeds = [Seed::from(b"nonce_authority"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the signer.
    withdraw_nonce_instruction.invoke_signed(&signer)?;

    Ok(())
}