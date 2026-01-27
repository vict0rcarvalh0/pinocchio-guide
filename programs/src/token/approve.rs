use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::Approve;

/// Processes the Approve instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to approve.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The token account.
///   1. `[]` The delegate account.
///   2. `[SIGNER]` The source account owner.
pub fn process_approve<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let source_account = &accounts[0];
    let delegate_account = &accounts[1];
    let authority_account = &accounts[2];

    // Ensure that the 'source' account is writable
    if !source_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure that the 'authority' account is a signer
    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let approve_instruction = Approve {
        source: source_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,
    };

    // Invoking the instruction
    approve_instruction.invoke_signed(signers)
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
    use spl_token::state::{AccountState, Account as TokenAccount};

    #[test]
    fn test_approve() {
        let program_id = Pubkey::new_unique();
        let (token_program, token_program_account) =
            mollusk_svm_programs_token::token::keyed_account();

        let mut mollusk = Mollusk::new(&program_id, "programs");
        mollusk_svm_programs_token::token::add_program(&mut mollusk);

        let mint = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let delegate = Pubkey::new_unique();
        let source_pubkey = Pubkey::new_unique();

        let mut source_account = Account::new(1_000_000_000, TokenAccount::LEN, &spl_token::ID);
        TokenAccount {
            mint,
            owner,
            amount: 1_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        }
        .pack_into_slice(&mut source_account.data);

        let amount: u64 = 500_000;

        // Discriminator 23 = TOKEN_APPROVE
        let mut data = vec![23u8];
        data.extend_from_slice(&amount.to_le_bytes());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(source_pubkey, false),
                AccountMeta::new_readonly(delegate, false),
                AccountMeta::new_readonly(owner, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (source_pubkey, source_account),
                (delegate, Account::new(0, 0, &Pubkey::default())),
                (owner, Account::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
