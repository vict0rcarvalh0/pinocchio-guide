use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{Signer, Seed},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::instructions::Approve;

// A constant representing the program ID, decoded from a base58 string.
// const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");

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
    // Validate that the data length is sufficient for the instruction.
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the amount from the data (first 8 bytes).
    let amount = unsafe { *(data.as_ptr().add(0) as *const u64) };

    // Extract the bump seed from the data (9th byte).
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(8) as *const [u8; 1]) };

    // Process the Approve instruction with the extracted parameters.
    process_approve(accounts, amount, bump)
}

/// Processes the `Approve` instruction.
///
/// This function handles the logic for approving a token transfer. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to approve.
/// - `bump`: The bump seed used for signer derivation.
///
/// ### Accounts:
/// 0. `[WRITE]` The token account.
/// 1. `[]` The delegate account.
/// 2. `[SIGNER]` The source account owner.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_approve<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,        // Amount of tokens to approve.
    bump: [u8; 1],      // The bump seed used for signer derivation.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [source_account, delegate_account, authority_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'source' account is writable.
    if !source_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'authority' account is a signer.
    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `Approve` instruction.
    let approve_instruction = Approve {
        source: source_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,
    };

    // Derive the signers using the bump seed.
    let seeds = [Seed::from(b"authority_account"), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    // Invoke the instruction with the derived signers.
    approve_instruction.invoke_signed(&signers)?;

    Ok(())
}
