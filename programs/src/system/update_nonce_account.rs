use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::UpgradeNonceAccount;

/// Processes the `UpgradeNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
pub fn process_update_nonce_account<'a>(accounts: &'a [AccountView]) -> ProgramResult {
    let [nonce_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'nonce_account' is writable
    if !nonce_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Creating the instruction instance
    let update_nonce_instruction = UpgradeNonceAccount {
        account: nonce_account,
    };

    // Invoking the instruction
    update_nonce_instruction.invoke()
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
    fn test_update_nonce_account() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "programs");

        let nonce_account = Pubkey::new_unique();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[12u8, 12u8],
            vec![AccountMeta::new(nonce_account, false)],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![(
                nonce_account,
                Account::new(
                    mollusk.sysvars.rent.minimum_balance(80),
                    80,
                    &system_program::id(),
                ),
            )],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
