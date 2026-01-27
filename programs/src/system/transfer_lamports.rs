use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

/// Processes the `Transfer` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The source account.
/// 1. `[WRITE]` The destination account.
pub fn process_transfer<'a>(
    accounts: &'a [AccountView],
    lamports: u64,
    signers: &[Signer],
) -> ProgramResult {
    let [from_account, to_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'from' account is a signer
    if !from_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure that the 'from' account is writable
    if !from_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'to' account is writable
    if !to_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Creating the instruction instance
    let transfer_instruction = Transfer {
        from: from_account,
        to: to_account,
        lamports,
    };

    // Invoking the instruction
    transfer_instruction.invoke_signed(signers)
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
    fn test_transfer() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "programs");

        let from_account = Pubkey::new_unique();
        let to_account = Pubkey::new_unique();
        let lamports: u64 = 1_000_000;

        // Discriminator 1 = SYSTEM_TRANSFER
        let mut data = vec![1u8];
        data.extend_from_slice(&lamports.to_le_bytes());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(from_account, true),
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
                    to_account,
                    Account::new(0, 0, &system_program::ID),
                ),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
