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
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let allocate_account = &accounts[0];

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
    use mollusk_svm::{program::keyed_account_for_system_program, Mollusk};
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

        let allocate_pubkey = Pubkey::new_unique();
        let space: u64 = 100;

        let (system_program_id, system_program_account) = keyed_account_for_system_program();

        // Discriminator 3 = SYSTEM_ALLOCATE
        let mut data = vec![3u8];
        data.extend_from_slice(&space.to_le_bytes());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(allocate_pubkey, true),
                AccountMeta::new_readonly(system_program_id, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (allocate_pubkey, Account::new(1_000_000_000, 0, &system_program::ID)),
                (system_program_id, system_program_account),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
