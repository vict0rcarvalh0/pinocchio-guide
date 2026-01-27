use pinocchio::{
    cpi::Signer,
    error::ProgramError,
    AccountView,
    Address,
    ProgramResult,
};
use pinocchio_system::instructions::Assign;

/// Processes the `Assign` instruction.
///
/// Assigns an account to a new program owner.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `owner`: The public key of the new program owner.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The account to be reassigned.
pub fn process_assign<'a>(
    accounts: &'a [AccountView],
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let assigned_account = &accounts[0];

    if !assigned_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Assign {
        account: assigned_account,
        owner,
    }
    .invoke_signed(signers)
}

#[cfg(test)]
mod tests {
    use mollusk_svm::{program::keyed_account_for_system_program, Mollusk};
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };
    use solana_sdk_ids::system_program;

    #[test]
    fn test_assign() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "programs");

        let assigned_pubkey = Pubkey::new_unique();
        let new_owner = Pubkey::new_unique();

        // Include system program for CPI
        let (system_program_id, system_program_account) = keyed_account_for_system_program();

        // Discriminator 0 = SYSTEM_ASSIGN
        let mut instruction_data = vec![0u8];
        instruction_data.extend_from_slice(new_owner.as_ref());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(assigned_pubkey, true),
                AccountMeta::new_readonly(system_program_id, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &[
                (assigned_pubkey, Account::new(1_000_000_000, 0, &system_program::ID)),
                (system_program_id, system_program_account),
            ],
        );

        assert!(
            !result.program_result.is_err(),
            "Expected success, got: {:?}",
            result.program_result
        );
    }
}
