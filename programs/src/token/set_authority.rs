use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::{AuthorityType, SetAuthority};

/// Processes the SetAuthority instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `authority_type`: The type of authority to update.
/// - `new_authority`: Optional new authority.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The mint or account to change the authority of.
///   1. `[SIGNER]` The current authority of the mint or account.
pub fn process_set_authority<'a>(
    accounts: &'a [AccountView],
    authority_type: AuthorityType,
    new_authority: Option<&Address>,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let account_to_update = &accounts[0];
    let current_authority = &accounts[1];

    // Ensure the account to update is writable
    if !account_to_update.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the current authority account is a signer
    if !current_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Create the instruction instance
    let set_authority_instruction = SetAuthority {
        account: account_to_update,
        authority: current_authority,
        authority_type,
        new_authority,
    };

    // Invoke the instruction
    set_authority_instruction.invoke_signed(signers)
}

#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_program_pack::Pack;
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };
    use solana_sdk::program_option::COption;
    use spl_token::state::Mint;

    #[test]
    fn test_set_authority() {
        let program_id = Pubkey::new_unique();
        let (token_program, token_program_account) =
            mollusk_svm_programs_token::token::keyed_account();

        let mut mollusk = Mollusk::new(&program_id, "programs");
        mollusk_svm_programs_token::token::add_program(&mut mollusk);

        let mint_pubkey = Pubkey::new_unique();
        let current_authority = Pubkey::new_unique();
        let new_authority = Pubkey::new_unique();

        let mut mint_account = Account::new(1_000_000_000, Mint::LEN, &spl_token::ID);
        Mint {
            mint_authority: COption::Some(current_authority),
            supply: 1_000_000,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::None,
        }
        .pack_into_slice(&mut mint_account.data);

        // Discriminator 25 = TOKEN_SET_AUTHORITY
        // Data format: [discriminator, authority_type, option_flag, new_authority_pubkey]
        let mut data = vec![25u8];
        data.push(0); // MintTokens authority type
        data.push(1); // Some
        data.extend_from_slice(new_authority.as_ref());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(mint_pubkey, false),
                AccountMeta::new_readonly(current_authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (mint_pubkey, mint_account),
                (current_authority, Account::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
