use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::Transfer;

/// Processes the Transfer instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts involved in the transfer.
/// - `amount`: The amount of tokens to transfer.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The sender account.
///   1. `[WRITE]` The recipient account.
///   2. `[SIGNER]` The authority that approves the transfer.
pub fn process_transfer<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let sender_account = &accounts[0];
    let recipient_account = &accounts[1];
    let authority_account = &accounts[2];

    // Validate that the sender account is writable
    if !sender_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate that the recipient account is writable
    if !recipient_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate the authority is a signer
    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the Transfer instruction
    let transfer_instruction = Transfer {
        from: sender_account,
        to: recipient_account,
        authority: authority_account,
        amount,
    };

    // Invoke the instruction
    transfer_instruction.invoke_signed(signers)
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
    fn test_transfer() {
        let program_id = Pubkey::new_unique();
        let (token_program, token_program_account) =
            mollusk_svm_programs_token::token::keyed_account();

        let mut mollusk = Mollusk::new(&program_id, "programs");
        mollusk_svm_programs_token::token::add_program(&mut mollusk);

        let mint = Pubkey::new_unique();
        let sender = Pubkey::new_unique();
        let recipient = Pubkey::new_unique();
        let sender_ta = Pubkey::new_unique();
        let recipient_ta = Pubkey::new_unique();

        let mut sender_ta_account =
            Account::new(1_000_000_000, spl_token::state::Account::LEN, &spl_token::id());
        spl_token::state::Account {
            mint,
            owner: sender,
            amount: 1_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        }
        .pack_into_slice(sender_ta_account.data.as_mut_slice());

        let mut recipient_ta_account =
            Account::new(1_000_000_000, spl_token::state::Account::LEN, &spl_token::id());
        spl_token::state::Account {
            mint,
            owner: recipient,
            amount: 0,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        }
        .pack_into_slice(recipient_ta_account.data.as_mut_slice());

        let amount: u64 = 100_000;

        // Discriminator 22
        let mut instruction_data = vec![22u8];
        instruction_data.extend_from_slice(&amount.to_le_bytes());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(sender_ta, false),
                AccountMeta::new(recipient_ta, false),
                AccountMeta::new_readonly(sender, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (sender_ta, sender_ta_account),
                (recipient_ta, recipient_ta_account),
                (
                    sender,
                    Account::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
