use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::CloseAccount;

/// Processes the CloseAccount instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to close.
///   1. `[WRITE]` The destination account.
///   2. `[SIGNER]` The account's owner.
pub fn process_close_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let close_account = &accounts[0];
    let destination_account = &accounts[1];
    let authority_account = &accounts[2];

    // Ensure that the 'close' account is writable
    if !close_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'destination' account is writable
    if !destination_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'authority' account is a signer
    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let close_account_instruction = CloseAccount {
        account: close_account,
        destination: destination_account,
        authority: authority_account,
    };

    // Invoking the instruction
    close_account_instruction.invoke_signed(signers)
}

#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_program_pack::Pack;
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        pubkey::Pubkey,
    };
    use spl_token::state::AccountState;

    #[test]
    fn test_close_account() {
        let program_id = Pubkey::new_unique();
        let (token_program, token_program_account) =
            mollusk_svm_programs_token::token::keyed_account();

        let mut mollusk = Mollusk::new(&program_id, "programs");
        mollusk_svm_programs_token::token::add_program(&mut mollusk);

        let mint = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let close_account_pubkey = Pubkey::new_unique();
        let destination = Pubkey::new_unique();

        let mut close_account =
            Account::new(1_000_000_000, spl_token::state::Account::LEN, &spl_token::id());
        spl_token::state::Account {
            mint,
            owner,
            amount: 0, // Must be 0 to close
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        }
        .pack_into_slice(close_account.data.as_mut_slice());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[28u8, 28u8],
            vec![
                AccountMeta::new(close_account_pubkey, false),
                AccountMeta::new(destination, false),
                AccountMeta::new_readonly(owner, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (close_account_pubkey, close_account),
                (
                    destination,
                    Account::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (
                    owner,
                    Account::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
