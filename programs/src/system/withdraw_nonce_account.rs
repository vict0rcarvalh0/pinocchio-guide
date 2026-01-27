use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::WithdrawNonceAccount;

/// Processes the `WithdrawNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to withdraw.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[WRITE]` The recipient account.
/// 2. `[]` The recent blockhashes sysvar.
/// 3. `[]` The rent sysvar.
/// 4. `[SIGNER]` The Nonce authority.
pub fn process_withdraw_nonce_account<'a>(
    accounts: &'a [AccountView],
    lamports: u64,
    signers: &[Signer],
) -> ProgramResult {
    let [nonce_account, recipient_account, recent_blockhashes_sysvar, rent_sysvar, nonce_authority] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the necessary accounts are writable
    if !nonce_account.is_writable() || !recipient_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the nonce authority is a signer
    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let withdraw_nonce_instruction = WithdrawNonceAccount {
        account: nonce_account,
        recipient: recipient_account,
        recent_blockhashes_sysvar,
        rent_sysvar,
        authority: nonce_authority,
        lamports,
    };

    // Invoking the instruction
    withdraw_nonce_instruction.invoke_signed(signers)
}

#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_sdk_ids::system_program;
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    };

    #[test]
    fn test_withdraw_nonce_account() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "target/deploy/programs");

        let nonce_account = Pubkey::new_unique();
        let recipient_account = Pubkey::new_unique();
        let nonce_authority = Pubkey::new_unique();
        let lamports: u64 = 1_000_000;

        // Discriminator 8 = SYSTEM_WITHDRAW_NONCE
        let mut instruction_data = vec![8u8];
        instruction_data.extend_from_slice(&lamports.to_le_bytes());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(nonce_account, false),
                AccountMeta::new(recipient_account, false),
                AccountMeta::new_readonly(sysvar::recent_blockhashes::ID, false),
                AccountMeta::new_readonly(sysvar::rent::ID, false),
                AccountMeta::new_readonly(nonce_authority, true),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    nonce_account,
                    Account::new(
                        mollusk.sysvars.rent.minimum_balance(80) + lamports,
                        80,
                        &system_program::ID,
                    ),
                ),
                (
                    recipient_account,
                    Account::new(0, 0, &system_program::ID),
                ),
                (
                    sysvar::recent_blockhashes::ID,
                    Account::new(1_000_000_000, 0, &sysvar::ID),
                ),
                (sysvar::rent::ID, mollusk.sysvars.keyed_account_for_rent_sysvar().1),
                (
                    nonce_authority,
                    Account::new(1_000_000_000, 0, &system_program::ID),
                ),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
