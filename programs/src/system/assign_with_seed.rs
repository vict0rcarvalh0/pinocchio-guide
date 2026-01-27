use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::AssignWithSeed;

/// Processes the `AssignWithSeed` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `seed`: The seed used to derive the account.
/// - `owner`: The public key of the new program owner.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The account to be reassigned.
/// 1. `[SIGNER]` The base account used to derive the reassigned account.
pub fn process_assign_with_seed<'a>(
    accounts: &'a [AccountView],
    seed: &str,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let assigned_account = &accounts[0];
    let base_account = &accounts[1];

    // Ensure the base account is a signer
    if !base_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let assign_with_seed_instruction = AssignWithSeed {
        account: assigned_account,
        base: base_account,
        seed,
        owner,
    };

    // Invoking the instruction
    assign_with_seed_instruction.invoke_signed(signers)
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
    fn test_assign_with_seed() {
        let program_id = Pubkey::new_unique();
        let mut mollusk = Mollusk::new(&program_id, "programs");
        

        let base_account = Pubkey::new_unique();
        let seed = "test_seed";
        let assigned_account =
            Pubkey::create_with_seed(&base_account, seed, &system_program::id()).unwrap();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[5u8, 5u8],
            vec![
                AccountMeta::new(assigned_account, false),
                AccountMeta::new_readonly(base_account, true),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    assigned_account,
                    Account::new(1_000_000_000, 0, &system_program::id()),
                ),
                (
                    base_account,
                    Account::new(1_000_000_000, 0, &system_program::id()),
                ),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
