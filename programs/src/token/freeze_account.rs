use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::FreezeAccount;

/// Processes the FreezeAccount instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The account to freeze.
///   1. `[]` The token mint.
///   2. `[SIGNER]` The mint freeze authority.
pub fn process_freeze_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let account_to_freeze = &accounts[0];
    let mint_account = &accounts[1];
    let freeze_authority = &accounts[2];

    // Ensure that the account to freeze is writable
    if !account_to_freeze.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the freeze authority is a signer
    if !freeze_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let freeze_account_instruction = FreezeAccount {
        account: account_to_freeze,
        mint: mint_account,
        freeze_authority,
    };

    // Invoking the instruction
    freeze_account_instruction.invoke_signed(signers)
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
    fn test_freeze_account() {
        let program_id = Pubkey::new_unique();
        let (token_program, token_program_account) =
            mollusk_svm_programs_token::token::keyed_account();

        let mut mollusk = Mollusk::new(&program_id, "programs");
        mollusk_svm_programs_token::token::add_program(&mut mollusk);

        let mint_pubkey = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let freeze_authority = Pubkey::new_unique();
        let account_to_freeze_pubkey = Pubkey::new_unique();

        let mut mint_account =
            Account::new(1_000_000_000, spl_token::state::Mint::LEN, &spl_token::id());
        spl_token::state::Mint {
            mint_authority: COption::Some(owner),
            supply: 1_000_000,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::Some(freeze_authority),
        }
        .pack_into_slice(mint_account.data.as_mut_slice());

        let mut account_to_freeze =
            Account::new(1_000_000_000, spl_token::state::Account::LEN, &spl_token::id());
        spl_token::state::Account {
            mint: mint_pubkey,
            owner,
            amount: 1_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        }
        .pack_into_slice(account_to_freeze.data.as_mut_slice());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[29u8, 29u8],
            vec![
                AccountMeta::new(account_to_freeze_pubkey, false),
                AccountMeta::new_readonly(mint_pubkey, false),
                AccountMeta::new_readonly(freeze_authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (account_to_freeze_pubkey, account_to_freeze),
                (mint_pubkey, mint_account),
                (
                    freeze_authority,
                    Account::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
