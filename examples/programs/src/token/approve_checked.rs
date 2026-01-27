use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    instruction::{Signer, Seed},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::ApproveChecked;

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
    // Validate the length of the data buffer.
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the `amount` from the data buffer.
    let amount = unsafe { *(data.as_ptr().add(0) as *const u64) };

    // Extract the `decimals` from the data buffer.
    let decimals = unsafe { *(data.as_ptr().add(8) as *const u8) };

    // Extract the `bump` from the data buffer.
    let bump = unsafe { *(data.as_ptr().add(9) as *const [u8; 1]) };

    // Process the `ApproveChecked` instruction with the extracted parameters.
    process_approve_checked(accounts, amount, decimals, bump)
}

/// Processes the `ApproveChecked` instruction.
///
/// This function handles the logic for approving a token transfer. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to approve.
/// - `decimals`: The number of decimals for the token.
/// - `bump`: The bump seed for the signer.
///
/// ### Accounts:
/// 0. `[WRITE]` The source account.
/// 1. `[]` The token mint.
/// 2. `[]` The delegate account.
/// 3. `[SIGNER]` The source account owner.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_approve_checked<'a>(
    accounts: &'a [AccountInfo],
    amount: u64,        // Amount of tokens to approve.
    decimals: u8,       // Token decimals for validation.
    bump: [u8; 1],      // The bump seed for the signer.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [source_account, mint_account, delegate_account, authority_account] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the `source_account` is writable.
    if !source_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the `authority_account` is a signer.
    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `ApproveChecked` instruction.
    let approve_checked_instruction = ApproveChecked {
        source: source_account,
        mint: mint_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,
        decimals,
    };

    // Create the signer seeds using the bump seed.
    let seeds = [Seed::from(b"authority_account"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Invoke the instruction with the signer seeds.
    approve_checked_instruction.invoke_signed(&signer)?;

    Ok(())
}