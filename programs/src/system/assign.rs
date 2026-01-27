use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Assign;

/// Processes the `Assign` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `owner`: The public key of the new program owner.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The account to be reassigned to a new program owner.
pub fn process_assign<'a>(
    accounts: &'a [AccountView],
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    let [assigned_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the assigned account is a signer
    if !assigned_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let assign_instruction = Assign {
        account: assigned_account,
        owner,
    };

    // Invoking the instruction
    assign_instruction.invoke_signed(signers)
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
    fn test_assign() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "target/deploy/programs");

        let assigned_account = Pubkey::new_unique();
        let new_owner = Pubkey::new_unique();

        // Discriminator 0 = SYSTEM_ASSIGN
        let mut instruction_data = vec![0u8];
        instruction_data.extend_from_slice(new_owner.as_ref());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![AccountMeta::new(assigned_account, true)],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![(
                assigned_account,
                Account::new(1_000_000_000, 0, &system_program::id()),
            )],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
