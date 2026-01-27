use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccountWithSeed;

/// Processes the `CreateAccountWithSeed` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `seed`: The seed used to derive the account.
/// - `lamports`: The number of lamports to transfer to the new account.
/// - `space`: The number of bytes to allocate for the new account.
/// - `owner`: The program that will own the new account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The funding account.
/// 1. `[WRITE]` The new account to be created.
/// 2. `[SIGNER]` The base account used to derive the new account.
pub fn process_create_account_with_seed<'a>(
    accounts: &'a [AccountView],
    seed: &'a str,
    lamports: u64,
    space: u64,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    let [funding_account, new_account, base_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that funding account is a signer
    if !funding_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let create_account_with_seed_instruction = CreateAccountWithSeed {
        from: funding_account,
        to: new_account,
        base: Some(base_account),
        seed,
        lamports,
        space,
        owner,
    };

    // Invoking the instruction
    create_account_with_seed_instruction.invoke_signed(signers)
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
    fn test_create_account_with_seed() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "target/deploy/programs");

        let funding_account = Pubkey::new_unique();
        let base_account = Pubkey::new_unique();
        let seed = "test_seed";
        let new_account =
            Pubkey::create_with_seed(&base_account, seed, &system_program::ID).unwrap();

        // Discriminator 6 = SYSTEM_CREATE_ACCOUNT_WITH_SEED
        let instruction = Instruction::new_with_bytes(
            program_id,
            &[6u8],
            vec![
                AccountMeta::new(funding_account, true),
                AccountMeta::new(new_account, false),
                AccountMeta::new_readonly(base_account, true),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    funding_account,
                    Account::new(10_000_000_000, 0, &system_program::ID),
                ),
                (
                    new_account,
                    Account::new(0, 0, &system_program::ID),
                ),
                (
                    base_account,
                    Account::new(1_000_000_000, 0, &system_program::ID),
                ),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
