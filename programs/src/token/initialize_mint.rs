use pinocchio::{
    AccountView,
    Address,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::InitializeMint;

/// Processes the InitializeMint instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `decimals`: Number of decimals for the token.
/// - `mint_authority`: The public key of the mint authority.
/// - `freeze_authority`: An optional public key for the freeze authority.
///
/// ### Accounts:
///   0. `[WRITABLE]` Mint account.
///   1. `[]` Rent sysvar.
pub fn process_initialize_mint<'a>(
    accounts: &'a [AccountView],
    decimals: u8,
    mint_authority: &Address,
    freeze_authority: Option<&Address>,
) -> ProgramResult {
    let [mint_account, rent_sysvar] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the mint account is writable
    if !mint_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Creating the instruction instance
    let initialize_mint_instruction = InitializeMint {
        mint: mint_account,
        rent_sysvar,
        decimals,
        mint_authority,
        freeze_authority,
    };

    // Invoking the instruction (no signers needed for initialize)
    initialize_mint_instruction.invoke()
}

#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_program_pack::Pack;
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    };

    #[test]
    fn test_initialize_mint() {
        let program_id = Pubkey::new_unique();
        let (token_program, token_program_account) =
            mollusk_svm_programs_token::token::keyed_account();

        let mut mollusk = Mollusk::new(&program_id, "programs");
        mollusk_svm_programs_token::token::add_program(&mut mollusk);

        let mint_pubkey = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();

        let mint_account = Account::new(
            mollusk.sysvars.rent.minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &spl_token::id(),
        );

        let decimals: u8 = 9;

        let mut data = Vec::new();
        data.push(decimals);
        data.extend_from_slice(mint_authority.as_ref());
        data.push(0); // No freeze authority

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(mint_pubkey, false),
                AccountMeta::new_readonly(sysvar::rent::ID, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (mint_pubkey, mint_account),
                (sysvar::rent::ID, mollusk.sysvars.keyed_account_for_rent_sysvar().1),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err(), "Result: {:?}", result.program_result);
    }
}
