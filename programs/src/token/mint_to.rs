use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::instructions::MintTo;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = unsafe { *(data.as_ptr().add(0) as *const u64) };
    let bump: [u8; 1] = unsafe { *(data.as_ptr().add(8) as *const [u8; 1]) };
    process_mint_to(accounts, amount, bump)
}

/// Processes the MintTo instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `amount`: The amount of tokens to mint.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]` The mint account.
///   1. `[WRITE]` The account to mint tokens to.
///   2. `[SIGNER]` The mint's minting authority.
pub fn process_mint_to(
    accounts: &[AccountInfo],
    amount: u64,   // Amount of tokens to mint.
    bump: [u8; 1], // Bump seed for the signer account.
) -> ProgramResult {
    // Extracting account information
    let [mint_account, token_account, mint_authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the mint account is writable
    assert!(mint_account.is_writable(), "Mint account is not writable");

    // Ensure the token account is writable
    assert!(token_account.is_writable(), "Token account is not writable");

    // Ensure the mint authority is a signer
    assert!(mint_authority.is_signer(), "Mint authority is not a signer");

    // Creating the instruction instance
    let mint_to_instruction = MintTo {
        mint: mint_account,
        account: token_account,
        mint_authority,
        amount,
    };

    // Invoking the instruction
    // let seeds = &[&[b"mint_authority", bump.as_ref()]];
    // let signers = &[Signer::new(seeds)];
    let seeds = [Seed::from(b"mint_authority"), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    mint_to_instruction.invoke_signed(&signer)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount, WritableAccount},
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
    };
    use spl_token::state::AccountState;

    #[test]
    fn mint_to_test() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "11111111111111111111111111111111111111111111",
        ));

        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/programs");
        mollusk_token::token::add_program(&mut mollusk);

        let mint_pubkey = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let seed = b"mint_authority";
        let (mint_authority_pda, bump) = Pubkey::find_program_address(&[seed], &program_id);
        let bump_byte = [bump];

        let mint_authority_account = AccountSharedData::new(0, 0, &program_id);

        let mut mint_account =
            AccountSharedData::new(1_000_000_000, spl_token::state::Mint::LEN, &spl_token::id());
        spl_token::state::Mint {
            mint_authority: COption::Some(mint_authority_pda),
            supply: 0,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::None,
        }
        .pack_into_slice(mint_account.data_as_mut_slice());

        let recipient = Pubkey::new_unique();

        let token_account_pubkey = Pubkey::new_from_array([
            0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
            0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
            0x04, 0x04, 0x04, 0x04,
        ]);

        let mut token_account =
            AccountSharedData::new(0, spl_token::state::Account::LEN, &spl_token::id());
        spl_token::state::Account {
            mint: mint_pubkey,
            owner: recipient,
            amount: 0,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        }
        .pack_into_slice(token_account.data_as_mut_slice());

        assert_eq!(mint_account.owner(), &spl_token::id());
        assert_eq!(token_account.owner(), &spl_token::id());

        let amount = 6_9420_u64;
        let mut data = Vec::new();
        data.extend_from_slice(&amount.to_le_bytes());
        data.extend_from_slice(&bump_byte);

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(mint_pubkey, false),
                AccountMeta::new(token_account_pubkey, false),
                AccountMeta::new(mint_authority_pda, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (mint_pubkey, mint_account.clone()),
                (token_account_pubkey, token_account.clone()),
                (mint_authority_pda, mint_authority_account.clone()),
                (token_program, token_program_account.clone()),
            ],
        );
        assert!(
            !result.program_result.is_err(),
            "Error while processing instruction: {:?}",
            result.program_result
        );

        let token_account = result.get_account(&token_account_pubkey).unwrap();
        let token_account_data = spl_token::state::Account::unpack(token_account.data()).unwrap();
        assert_eq!(token_account_data.amount, amount);
    }
}
