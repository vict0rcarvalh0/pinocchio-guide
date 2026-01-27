use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

/// Processes the `CreateAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer to the new account.
/// - `space`: The number of bytes to allocate for the new account.
/// - `owner`: The program that will own the new account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The funding account.
/// 1. `[WRITE, SIGNER]` The new account to be created.
pub fn process_create_account<'a>(
    accounts: &'a [AccountView],
    lamports: u64,
    space: u64,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    let [funding_account, new_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the funding account and new account are signers
    if !funding_account.is_signer() || !new_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let create_account_instruction = CreateAccount {
        from: funding_account,
        to: new_account,
        lamports,
        space,
        owner,
    };

    // Invoking the instruction
    create_account_instruction.invoke_signed(signers)
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
    fn test_create_account() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "programs");

        let funding_account = Pubkey::new_unique();
        let new_account = Pubkey::new_unique();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[2u8, 2u8],
            vec![
                AccountMeta::new(funding_account, true),
                AccountMeta::new(new_account, true),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    funding_account,
                    Account::new(10_000_000_000, 0, &system_program::id()),
                ),
                (
                    new_account,
                    Account::new(0, 0, &system_program::id()),
                ),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
