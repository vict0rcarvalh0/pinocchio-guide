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
    let [nonce_account, nonce_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

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

        let nonce_account = Pubkey::new_unique();
        let nonce_authority = Pubkey::new_unique();
        let new_authority = Pubkey::new_unique();

        // Discriminator 10
        let mut instruction_data = vec![10u8];
        instruction_data.extend_from_slice(new_authority.as_ref());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(nonce_account, false),
                AccountMeta::new_readonly(nonce_authority, true),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    nonce_account,
                    Account::new(
                        mollusk.sysvars.rent.minimum_balance(80),
                        80,
                        &system_program::id(),
                    ),
                ),
                (
                    nonce_authority,
                    Account::new(1_000_000_000, 0, &system_program::id()),
                ),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
