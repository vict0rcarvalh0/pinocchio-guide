use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::InitializeAccount;

/// Processes the InitializeAccount instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
///
/// ### Accounts:
///   0. `[WRITE]`  The account to initialize.
///   1. `[]` The mint this account will be associated with.
///   2. `[]` The new account's owner/multisignature.
///   3. `[]` Rent sysvar.
pub fn process_initialize_account<'a>(accounts: &'a [AccountView]) -> ProgramResult {
    if accounts.len() < 4 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let account_to_initialize = &accounts[0];
    let mint_account = &accounts[1];
    let owner_account = &accounts[2];
    let rent_sysvar = &accounts[3];

    // Ensure that the account to initialize is writable
    if !account_to_initialize.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Construct the `InitializeAccount` instruction.
    let initialize_account_instruction = InitializeAccount {
        account: account_to_initialize,
        mint: mint_account,
        owner: owner_account,
        rent_sysvar,
    };

    // Invoking the instruction (no signers needed for initialize)
    initialize_account_instruction.invoke()
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
        sysvar,
    };
    use spl_token::state::Mint;

    /// Tests the transfer functionality of the token program.
    #[test]
    fn test_initialize_account() {
        let program_id = Pubkey::new_unique();
        let (token_program, token_program_account) =
            mollusk_svm_programs_token::token::keyed_account();

        let mut mollusk = Mollusk::new(&program_id, "programs");
        mollusk_svm_programs_token::token::add_program(&mut mollusk);

        let mint_pubkey = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let account_to_initialize = Pubkey::new_unique();

        let mut mint_account = Account::new(1_000_000_000, Mint::LEN, &spl_token::ID);
        Mint {
            mint_authority: COption::Some(owner),
            supply: 0,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::None,
        }
        .pack_into_slice(&mut mint_account.data);

        let new_account = Account::new(
            mollusk.sysvars.rent.minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );

        // Discriminator 21 = TOKEN_INIT_ACCOUNT
        let instruction_data = vec![21u8];

        // Construct the transfer instruction.
        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(account_to_initialize, false),
                AccountMeta::new_readonly(mint_pubkey, false),
                AccountMeta::new_readonly(owner, false),
                AccountMeta::new_readonly(sysvar::rent::ID, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let (_, rent_account) = mollusk.sysvars.keyed_account_for_rent_sysvar();

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (account_to_initialize, new_account),
                (mint_pubkey, mint_account),
                (owner, Account::new(1_000_000_000, 0, &Pubkey::default())),
                (sysvar::rent::ID, rent_account),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
