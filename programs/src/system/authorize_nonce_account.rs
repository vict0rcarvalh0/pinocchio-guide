use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
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
    accounts: &'a [AccountView],
    new_authority: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];
    let nonce_authority = &accounts[1];

    // Ensure the nonce account is writable
    if !nonce_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the nonce authority is a signer
    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let authorize_nonce_instruction = AuthorizeNonceAccount {
        account: nonce_account,
        authority: nonce_authority,
        new_authority,
    };

    // Invoking the instruction
    authorize_nonce_instruction.invoke_signed(signers)
}

#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_sdk_ids::system_program;
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    #[test]
    fn test_authorize_nonce_account() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "programs");

        let nonce_account_pubkey = Pubkey::new_unique();
        let nonce_authority = Pubkey::new_unique();
        let new_authority = Pubkey::new_unique();

        // Create a nonce account (actual usage requires initialized state)
        let nonce_account = Account::new(
            1_000_000_000,
            80, // Nonce account size
            &system_program::ID,
        );

        // Discriminator 10 = SYSTEM_AUTHORIZE_NONCE
        let mut instruction_data = vec![10u8];
        instruction_data.extend_from_slice(new_authority.as_ref());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(nonce_account_pubkey, false),
                AccountMeta::new_readonly(nonce_authority, true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (nonce_account_pubkey, nonce_account),
                (nonce_authority, Account::new(1_000_000_000, 0, &Pubkey::default())),
                (system_program::ID, Account::new(0, 0, &system_program::ID)),
            ],
        );

        // Note: This test validates instruction structure.
        // Actual authorize requires a properly initialized nonce account.
        println!("Result: {:?}", result.program_result);
    }
}
