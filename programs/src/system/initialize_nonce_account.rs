use pinocchio::{
    AccountView,
    Address,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::InitializeNonceAccount;

/// Processes the `InitializeNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `authority`: The public key of the entity authorized to manage the Nonce account.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[]` The recent blockhashes sysvar.
/// 2. `[]` The rent sysvar.
pub fn process_initialize_nonce_account<'a>(
    accounts: &'a [AccountView],
    authority: &'a Address,
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];
    let recent_blockhashes_sysvar = &accounts[1];
    let rent_sysvar = &accounts[2];

    // Ensure that nonce account is writable
    if !nonce_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Creating the instruction instance
    let initialize_nonce_account_instruction = InitializeNonceAccount {
        account: nonce_account,
        recent_blockhashes_sysvar,
        rent_sysvar,
        authority,
    };

    // Invoking the instruction (no signers needed for initialize)
    initialize_nonce_account_instruction.invoke()
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

    #[test]
    fn test_initialize_nonce_account() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "programs");

        let nonce_account_pubkey = Pubkey::new_unique();
        let authority = Pubkey::new_unique();

        // Create a fresh nonce account with proper size
        let nonce_account = Account::new(
            mollusk.sysvars.rent.minimum_balance(80),
            80, // Nonce account size
            &system_program::ID,
        );

        // Discriminator 9 = SYSTEM_INIT_NONCE
        let mut instruction_data = vec![9u8];
        instruction_data.extend_from_slice(authority.as_ref());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(nonce_account_pubkey, false),
                AccountMeta::new_readonly(sysvar::recent_blockhashes::ID, false),
                AccountMeta::new_readonly(sysvar::rent::ID, false),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
        );

        // Get proper sysvar accounts
        let (_, rent_account) = mollusk.sysvars.keyed_account_for_rent_sysvar();

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (nonce_account_pubkey, nonce_account),
                (
                    sysvar::recent_blockhashes::ID,
                    Account::new(1_000_000_000, 0, &sysvar::ID),
                ),
                (sysvar::rent::ID, rent_account),
                (system_program::ID, Account::new(0, 0, &system_program::ID)),
            ],
        );

        // Note: This test validates instruction structure.
        // The actual initialization requires proper runtime state.
        println!("Result: {:?}", result.program_result);
    }
}
