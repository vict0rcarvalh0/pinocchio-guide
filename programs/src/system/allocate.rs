use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Allocate;

/// Processes the `Allocate` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `space`: The number of bytes to allocate.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The account to allocate space for.
pub fn process_allocate<'a>(
    accounts: &'a [AccountView],
    space: u64,
    signers: &[Signer],
) -> ProgramResult {
    let [allocate_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the allocate account is a signer
    if !allocate_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let allocate_instruction = Allocate {
        account: allocate_account,
        space,
    };

    // Invoking the instruction
    allocate_instruction.invoke_signed(signers)
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
    fn test_allocate() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "programs");

        let allocate_account = Pubkey::new_unique();
        let space: u64 = 100;

        // Discriminator 3 = SYSTEM_ALLOCATE
        let mut data = vec![3u8];
        data.extend_from_slice(&space.to_le_bytes());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![AccountMeta::new(allocate_account, true)],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![(
                allocate_account,
                Account::new(1_000_000_000, 0, &system_program::ID),
            )],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
