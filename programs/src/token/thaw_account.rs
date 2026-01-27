use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::ThawAccount;

/// Processes the ThawAccount instruction.
///
/// ### Parameters:
/// - `accounts`: List of accounts involved in the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The token account to be thawed.
///   1. `[]` The token mint associated with the account.
///   2. `[SIGNER]` The freeze authority for the mint.
pub fn process_thaw_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let token_account = &accounts[0];
    let mint_account = &accounts[1];
    let freeze_authority_account = &accounts[2];

    // Validate that the token account is writable
    if !token_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate the freeze authority is a signer
    if !freeze_authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the ThawAccount instruction
    let thaw_account_instruction = ThawAccount {
        account: token_account,
        mint: mint_account,
        freeze_authority: freeze_authority_account,
    };

    // Invoke the instruction
    thaw_account_instruction.invoke_signed(signers)
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
    fn test_thaw_account() {
        let program_id = Pubkey::new_unique();
        let (token_program, token_program_account) =
            mollusk_svm_programs_token::token::keyed_account();

        let mut mollusk = Mollusk::new(&program_id, "programs");
        mollusk_svm_programs_token::token::add_program(&mut mollusk);

        let mint_pubkey = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let freeze_authority = Pubkey::new_unique();
        let token_account_pubkey = Pubkey::new_unique();

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

        let mut token_account =
            Account::new(1_000_000_000, spl_token::state::Account::LEN, &spl_token::id());
        spl_token::state::Account {
            mint: mint_pubkey,
            owner,
            amount: 1_000_000,
            delegate: COption::None,
            state: AccountState::Frozen, // Account is frozen
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        }
        .pack_into_slice(token_account.data.as_mut_slice());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[30u8, 30u8],
            vec![
                AccountMeta::new(token_account_pubkey, false),
                AccountMeta::new_readonly(mint_pubkey, false),
                AccountMeta::new_readonly(freeze_authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (token_account_pubkey, token_account),
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
