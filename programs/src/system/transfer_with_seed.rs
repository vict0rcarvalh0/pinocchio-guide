use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::TransferWithSeed;

/// Processes the `TransferWithSeed` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer.
/// - `seed`: The seed used to derive the source account.
/// - `owner`: The program that owns the source account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The source account.
/// 1. `[SIGNER]` The base account used to derive the source account.
/// 2. `[WRITE]` The destination account.
pub fn process_transfer_with_seed<'a>(
    accounts: &'a [AccountView],
    lamports: u64,
    seed: &'a str,
    owner: &'a Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let from_account = &accounts[0];
    let base_account = &accounts[1];
    let to_account = &accounts[2];

    // Ensure that the 'from' account is writable
    if !from_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'base' account is a signer
    if !base_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure that the 'to' account is writable
    if !to_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Creating the instruction instance
    let transfer_instruction = TransferWithSeed {
        from: from_account,
        base: base_account,
        to: to_account,
        lamports,
        seed,
        owner,
    };

    // Invoking the instruction
    transfer_instruction.invoke_signed(signers)
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
    fn test_transfer_with_seed() {
        let program_id = Pubkey::new_unique();
        let mut mollusk = Mollusk::new(&program_id, "programs");
        

        let base_account = Pubkey::new_unique();
        let seed = "test_seed";
        let from_account =
            Pubkey::create_with_seed(&base_account, seed, &system_program::ID).unwrap();
        let to_account = Pubkey::new_unique();
        let lamports: u64 = 1_000_000;

        // Discriminator 11 = SYSTEM_TRANSFER_WITH_SEED
        let mut instruction_data = vec![11u8];
        instruction_data.extend_from_slice(&lamports.to_le_bytes());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(from_account, false),
                AccountMeta::new_readonly(base_account, true),
                AccountMeta::new(to_account, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    from_account,
                    Account::new(10_000_000_000, 0, &system_program::ID),
                ),
                (
                    base_account,
                    Account::new(1_000_000_000, 0, &system_program::ID),
                ),
                (
                    to_account,
                    Account::new(0, 0, &system_program::ID),
                ),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
