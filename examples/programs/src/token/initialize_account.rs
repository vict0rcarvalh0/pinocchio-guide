use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_token::instructions::InitializeAccount;
use spl_token::solana_program::sysvar;

const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    process_initialize_account(accounts, signer)
}

/// Processes the InitializeAccount instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
///   0. `[WRITE]`  The account to initialize.
///   1. `[]` The mint this account will be associated with.
///   2. `[]` The new account's owner/multisignature.
///   3. `[]` Rent sysvar.
pub fn process_initialize_account<'a>(
    accounts: &'a [AccountInfo],
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [account_to_initialize, mint_account, owner_account, rent_sysvar] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the account to initialize is writable
    assert!(account_to_initialize.is_writable());

    // Ensure the rent sysvar is valid (you might need additional checks here)
    assert_eq!(rent_sysvar.key(), &spl_token::solana_program::sysvar::rent::ID);

    // Creating the instruction instance
    let initialize_account_instruction = InitializeAccount {
        account: account_to_initialize,
        mint: mint_account,
        owner: owner_account,
        rent_sysvar,
    };

    // Invoking the instruction
    initialize_account_instruction.invoke_signed(signers)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use mollusk_svm::{result::Check, Mollusk};
    use pinocchio_token::state::TokenAccount;
    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount, WritableAccount},
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
        sysvar::Sysvar,
    };
    use spl_token::state::AccountState;

    #[test]
    fn transfer_test() {
        let program_id = Pubkey::new_from_array([
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01, 0x01, 0x01,
        ]);
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();
        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/programs");
        mollusk_token::token::add_program(&mut mollusk);

        let mint = Pubkey::new_from_array([
            0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
            0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
            0x02, 0x02, 0x02, 0x02,
        ]);

        let signer = Pubkey::new_unique();
        let signer_account = AccountSharedData::new(
            1_000_000_000 * 10,
            spl_token::state::Account::LEN,
            &program_id,
        );
        println!("signer_account balance: {:?}", signer_account.lamports());

        let recipient = Pubkey::new_unique();
        let recipient_account = AccountSharedData::new(
            1_000_000_000 * 10,
            spl_token::state::Account::LEN,
            &program_id,
        );
        println!(
            "recipient_account balance: {:?}",
            recipient_account.lamports()
        );

        let signer_ta = Pubkey::new_from_array([
            0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
            0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
            0x03, 0x03, 0x03, 0x03,
        ]);
        let recipient_ta = Pubkey::new_from_array([
            0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
            0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
            0x04, 0x04, 0x04, 0x04,
        ]);

        let mut signer_ta_account =
            AccountSharedData::new(0, spl_token::state::Account::LEN, &spl_token::id());
        spl_token::state::Account {
            mint,
            owner: signer,
            amount: 1_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        }
        .pack_into_slice(signer_ta_account.data_as_mut_slice());

        let mut recipient_ta_account =
            AccountSharedData::new(0, spl_token::state::Account::LEN, &spl_token::id());
        spl_token::state::Account {
            mint,
            owner: recipient,
            amount: 1_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        }
        .pack_into_slice(recipient_ta_account.data_as_mut_slice());

        assert_eq!(signer_ta_account.owner(), &spl_token::id());
        assert_eq!(recipient_ta_account.owner(), &spl_token::id());

        let amount = 1_000_u64;
        let data = amount.to_le_bytes();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(signer_ta, false),
                AccountMeta::new(recipient_ta, false),
                AccountMeta::new(signer, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (signer_ta, signer_ta_account.clone()),
                (recipient_ta, recipient_ta_account.clone()),
                (signer, signer_account.clone()),
                (token_program, token_program_account.clone()),
            ],
        );
        assert!(
            !result.program_result.is_err(),
            "Error while processing instruction",
        );
    }
}