use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::SyncNative;

/// Process the SyncNative instruction.
///
/// ### Parameters:
/// - `accounts`: List of the accounts involved in the instruction.
///
/// ### Accounts:
///   0. `[WRITE]` The native token account to be synchronized with the underlying lamports.
pub fn process_sync_native<'a>(accounts: &'a [AccountView]) -> ProgramResult {
    if accounts.len() < 1 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let native_token_account = &accounts[0];

    // Validate if the account is writable
    if !native_token_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Construct the SyncNative instruction
    let sync_native_instruction = SyncNative {
        native_token: native_token_account,
    };

    // Invoke the instruction
    sync_native_instruction.invoke()
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
    fn test_sync_native() {
        let program_id = Pubkey::new_unique();
        let (token_program, token_program_account) =
            mollusk_svm_programs_token::token::keyed_account();

        let mut mollusk = Mollusk::new(&program_id, "programs");
        mollusk_svm_programs_token::token::add_program(&mut mollusk);

        let owner = Pubkey::new_unique();
        let native_token_account_pubkey = Pubkey::new_unique();

        // Native mint (wrapped SOL)
        let native_mint = spl_token::native_mint::id();

        let mut native_token_account =
            Account::new(2_000_000_000, spl_token::state::Account::LEN, &spl_token::id());
        spl_token::state::Account {
            mint: native_mint,
            owner,
            amount: 1_000_000_000, // Intentionally different from lamports
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::Some(1_000_000_000),
            delegated_amount: 0,
            close_authority: COption::None,
        }
        .pack_into_slice(native_token_account.data.as_mut_slice());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[35u8, 35u8],
            vec![
                AccountMeta::new(native_token_account_pubkey, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (native_token_account_pubkey, native_token_account),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
