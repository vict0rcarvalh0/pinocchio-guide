use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_system::instructions::AuthorizeNonceAccount;

/// Processes the `AuthorizeNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `new_authority`: The public key of the new authority.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[SIGNER]` The current Nonce authority.
pub fn process_authorize_nonce_account<'a>(
    accounts: &'a [AccountInfo],
    new_authority: &Pubkey,  // Pubkey of the new entity to be authorized to execute nonce instructions on the account.
    signers: &[Signer],
) -> ProgramResult {
    // Extracting account information
    let [nonce_account, nonce_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the nonce authority is a signer
    assert!(nonce_authority.is_signer(), ProgramError::MissingRequiredSignature);

    // Creating the instruction instance
    let authorize_nonce_instruction = AuthorizeNonceAccount {
        account: nonce_account,
        authority: nonce_authority,
        new_authority,
    };

    // Invoking the instruction
    authorize_nonce_instruction.invoke_signed(signers)?;

    Ok(())
}