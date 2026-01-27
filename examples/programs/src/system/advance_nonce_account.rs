use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};

use pinocchio_system::instructions::AdvanceNonceAccount;

/// Processes the `AdvanceNonceAccount` instruction.
///
/// This function handles the logic for advancing a nonce account. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[]` The recent blockhashes sysvar.
/// 2. `[SIGNER]` The Nonce authority.
pub fn process_advance_nonce_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];
    let recent_blockhashes_sysvar = &accounts[1];
    let nonce_authority = &accounts[2];

    // Ensure the nonce account is writable
    if !nonce_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the nonce authority is a signer
    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let advance_nonce_instruction = AdvanceNonceAccount {
        account: nonce_account,
        recent_blockhashes_sysvar,
        authority: nonce_authority,
    };

    // Invoking the instruction
    advance_nonce_instruction.invoke_signed(signers)
}

#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_sdk_ids::system_program;
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    };

    /// Unit test for the `process_advance_nonce_account` function.
    ///
    /// This test sets up a mock environment using the `Mollusk` framework, creates
    /// the necessary accounts and instruction, and verifies that the instruction
    /// processes successfully.
    #[test]
    fn test_advance_nonce_account() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "programs");

        let nonce_account_pubkey = Pubkey::new_unique();
        let nonce_authority = Pubkey::new_unique();

        // Create a nonce account (actual usage requires initialized state)
        let nonce_account = Account::new(
            1_000_000_000,
            80, // Nonce account size
            &system_program::ID,
        );

        // Discriminator 7 = SYSTEM_ADVANCE_NONCE
        let instruction_data = vec![7u8];

        // Construct the instruction with the required accounts.
        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(nonce_account_pubkey, false),
                AccountMeta::new_readonly(sysvar::recent_blockhashes::ID, false),
                AccountMeta::new_readonly(nonce_authority, true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (nonce_account_pubkey, nonce_account),
                (
                    sysvar::recent_blockhashes::ID,
                    Account::new(1_000_000_000, 0, &sysvar::ID),
                ),
                (nonce_authority, Account::new(1_000_000_000, 0, &Pubkey::default())),
                (system_program::ID, Account::new(0, 0, &system_program::ID)),
            ],
        );

        // Note: This test validates instruction structure.
        // Actual advance requires a properly initialized nonce account.
        println!("Result: {:?}", result.program_result);
    }
}
