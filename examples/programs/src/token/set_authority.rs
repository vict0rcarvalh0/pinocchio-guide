use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::Signer,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::instructions::{AuthorityType, SetAuthority};

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
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Call the `process_set_authority` function to handle the instruction logic.
    process_set_authority(accounts, authority_type, new_authority, signers)
}

/// Processes the `SetAuthority` instruction.
///
/// This function handles the logic for setting a new authority for a mint or account.
/// It validates the accounts and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `authority_type`: The type of authority to set.
/// - `new_authority`: The optional new authority to set.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The mint or account to change the authority of.
/// 1. `[SIGNER]` The current authority of the mint or account.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_set_authority<'a>(
    accounts: &'a [AccountInfo],
    authority_type: AuthorityType,
    new_authority: Option<&Pubkey>, // Optional new authority
    signers: &[Signer],
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [account_to_update, current_authority] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the account to update is writable.
    if !account_to_update.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the current authority account is a signer.
    if !current_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `SetAuthority` instruction.
    let set_authority_instruction = SetAuthority {
        account: account_to_update,
        authority: current_authority,
        authority_type,
        new_authority,
    };

    // Invoke the instruction with the provided signers.
    set_authority_instruction.invoke_signed(signers)?;

    Ok(())
}