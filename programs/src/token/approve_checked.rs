use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::ApproveChecked;

/// Processes the ApproveChecked instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to approve.
/// - `decimals`: The number of decimals for the token.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[]` The token mint.
///   2. `[]` The delegate account.
///   3. `[SIGNER]` The source account owner.
pub fn process_approve_checked<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    decimals: u8,
    signers: &[Signer],
) -> ProgramResult {
    let [source_account, mint_account, delegate_account, authority_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the 'source' account is writable
    if !source_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'authority' account is a signer
    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let approve_checked_instruction = ApproveChecked {
        source: source_account,
        mint: mint_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,
        decimals,
    };

    // Invoking the instruction
    approve_checked_instruction.invoke_signed(signers)
}

#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_program_pack::Pack;
    use solana_sdk::{
        account::{Account, WritableAccount},
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        pubkey::Pubkey,
    };
    use spl_token::state::AccountState;

    #[test]
    fn test_approve_checked() {
        let program_id = Pubkey::new_unique();
        let (token_program, token_program_account) =
            mollusk_svm_programs_token::token::keyed_account();

        let mut mollusk = Mollusk::new(&program_id, "programs");
        mollusk_svm_programs_token::token::add_program(&mut mollusk);

        let mint_pubkey = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let delegate = Pubkey::new_unique();
        let source_account_pubkey = Pubkey::new_unique();

        let mut mint_account =
            Account::new(1_000_000_000, spl_token::state::Mint::LEN, &spl_token::id());
        spl_token::state::Mint {
            mint_authority: COption::Some(owner),
            supply: 1_000_000,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::None,
        }
        .pack_into_slice(mint_account.data_as_mut_slice());

        let mut source_account =
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
        .pack_into_slice(source_account.data_as_mut_slice());

        let amount: u64 = 500_000;
        let decimals: u8 = 9;

        let mut data = Vec::new();
        data.extend_from_slice(&amount.to_le_bytes());
        data.push(decimals);

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(source_account_pubkey, false),
                AccountMeta::new_readonly(mint_pubkey, false),
                AccountMeta::new_readonly(delegate, false),
                AccountMeta::new_readonly(owner, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (source_account_pubkey, source_account),
                (mint_pubkey, mint_account),
                (delegate, Account::new(0, 0, &Pubkey::default())),
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
