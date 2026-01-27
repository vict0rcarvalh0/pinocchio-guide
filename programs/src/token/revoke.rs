use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::Revoke;

/// Processes the Revoke instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[SIGNER]` The source account owner.
pub fn process_revoke<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let source_account = &accounts[0];
    let owner_account = &accounts[1];

    // Ensure the source account is writable
    if !source_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the owner account is a signer
    if !owner_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let revoke_instruction = Revoke {
        source: source_account,
        authority: owner_account,
    };

    // Invoking the instruction
    revoke_instruction.invoke_signed(signers)
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
    fn test_revoke() {
        let program_id = Pubkey::new_unique();
        let (token_program, token_program_account) =
            mollusk_svm_programs_token::token::keyed_account();

        let mut mollusk = Mollusk::new(&program_id, "programs");
        mollusk_svm_programs_token::token::add_program(&mut mollusk);

        let mint = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let delegate = Pubkey::new_unique();
        let source_account_pubkey = Pubkey::new_unique();

        let mut source_account =
            Account::new(1_000_000_000, spl_token::state::Account::LEN, &spl_token::id());
        spl_token::state::Account {
            mint,
            owner,
            amount: 1_000_000,
            delegate: COption::Some(delegate),
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 500_000,
            close_authority: COption::None,
        }
        .pack_into_slice(source_account.data.as_mut_slice());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[24u8, 24u8],
            vec![
                AccountMeta::new(source_account_pubkey, false),
                AccountMeta::new_readonly(owner, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (source_account_pubkey, source_account),
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
