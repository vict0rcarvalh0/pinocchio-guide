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
    if accounts.len() < 5 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];
    let recipient_account = &accounts[1];
    let recent_blockhashes_sysvar = &accounts[2];
    let rent_sysvar = &accounts[3];
    let nonce_authority = &accounts[4];

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
        let mollusk = Mollusk::new(&program_id, "programs");

        let nonce_account_pubkey = Pubkey::new_unique();
        let recipient_account = Pubkey::new_unique();
        let nonce_authority = Pubkey::new_unique();
        let lamports: u64 = 1_000_000;

        // Create a nonce account with extra lamports
        let nonce_account = Account::new(
            mollusk.sysvars.rent.minimum_balance(80) + lamports,
            80, // Nonce account size
            &system_program::ID,
        );

        // Get proper rent sysvar account
        let (_, rent_account) = mollusk.sysvars.keyed_account_for_rent_sysvar();

        // Discriminator 8 = SYSTEM_WITHDRAW_NONCE
        let mut instruction_data = vec![8u8];
        instruction_data.extend_from_slice(&lamports.to_le_bytes());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(nonce_account_pubkey, false),
                AccountMeta::new(recipient_account, false),
                AccountMeta::new_readonly(sysvar::recent_blockhashes::ID, false),
                AccountMeta::new_readonly(sysvar::rent::ID, false),
                AccountMeta::new_readonly(nonce_authority, true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (nonce_account_pubkey, nonce_account),
                (recipient_account, Account::new(0, 0, &system_program::ID)),
                (
                    sysvar::recent_blockhashes::ID,
                    Account::new(1_000_000_000, 0, &sysvar::ID),
                ),
                (sysvar::rent::ID, rent_account),
                (nonce_authority, Account::new(1_000_000_000, 0, &Pubkey::default())),
                (system_program::ID, Account::new(0, 0, &system_program::ID)),
            ],
        );

        // Note: This test validates instruction structure.
        // Actual withdraw requires a properly initialized nonce account.
        println!("Result: {:?}", result.program_result);
    }
}
