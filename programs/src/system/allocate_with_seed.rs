use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::AllocateWithSeed;

/// Processes the `AllocateWithSeed` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `seed`: The seed used to derive the account's address.
/// - `space`: The number of bytes to allocate.
/// - `owner`: The program that will own the allocated account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The allocated account.
/// 1. `[SIGNER]` The base account used to derive the allocated account.
pub fn process_allocate_with_seed<'a>(
    accounts: &'a [AccountView],
    seed: &str,
    space: u64,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let allocated_account = &accounts[0];
    let base_account = &accounts[1];

    // Ensure the base account is a signer
    if !base_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let allocate_with_seed_instruction = AllocateWithSeed {
        account: allocated_account,
        base: base_account,
        seed,
        space,
        owner,
    };

    // Invoking the instruction
    allocate_with_seed_instruction.invoke_signed(signers)
}

#[cfg(test)]
mod tests {
    use mollusk_svm::{program::keyed_account_for_system_program, Mollusk};
    use solana_sdk_ids::system_program;
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    #[test]
    fn test_allocate_with_seed() {
        let program_id = Pubkey::new_unique();
        let mut mollusk = Mollusk::new(&program_id, "programs");
        

        let base_account = Pubkey::new_unique();
        let seed = "test_seed";
        let allocated_account =
            Pubkey::create_with_seed(&base_account, seed, &system_program::ID).unwrap();

        // Discriminator 4 = SYSTEM_ALLOCATE_WITH_SEED
        let instruction = Instruction::new_with_bytes(
            program_id,
            &[4u8],
            vec![
                AccountMeta::new(allocated_account, false),
                AccountMeta::new_readonly(base_account, true),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    allocated_account,
                    Account::new(1_000_000_000, 0, &system_program::ID),
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
