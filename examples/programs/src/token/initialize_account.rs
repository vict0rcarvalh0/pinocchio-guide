use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::instructions::InitializeAccount;
use spl_token::solana_program::sysvar;

// A constant representing the program ID, decoded from a base58 string.
const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");

// Macro to define the program's entry point.
entrypoint!(process_instruction);

/// Entry point for the program. This function is called when the program is invoked.
///
/// ### Parameters:
/// - `program_id`: The ID of the program being executed.
/// - `accounts`: The accounts passed to the program.
/// - `data`: Additional data passed to the program.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the program execution.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Ensure the data length is valid for the instruction.
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Process the InitializeAccount instruction.
    process_initialize_account(accounts, signer)
}

/// Processes the `InitializeAccount` instruction.
///
/// This function handles the logic for initializing a token account. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The account to initialize.
/// 1. `[]` The mint this account will be associated with.
/// 2. `[]` The new account's owner/multisignature.
/// 3. `[]` Rent sysvar.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_initialize_account<'a>(
    accounts: &'a [AccountInfo],
    signers: &[Signer], // The signers array needed to authorize the transaction.
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [account_to_initialize, mint_account, owner_account, rent_sysvar] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that the account to initialize is writable.
    assert!(account_to_initialize.is_writable());

    // Ensure the rent sysvar is valid by checking its key.
    assert_eq!(rent_sysvar.key(), &spl_token::solana_program::sysvar::rent::ID);

    // Construct the `InitializeAccount` instruction.
    let initialize_account_instruction = InitializeAccount {
        account: account_to_initialize,
        mint: mint_account,
        owner: owner_account,
        rent_sysvar,
    };

    // Invoke the instruction with the provided signers.
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

    /// Tests the transfer functionality of the token program.
    #[test]
    fn transfer_test() {
        // Define the program ID for the test.
        let program_id = Pubkey::new_from_array([
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01, 0x01, 0x01,
        ]);

        // Initialize the Mollusk virtual machine and add the token program.
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();
        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/programs");
        mollusk_token::token::add_program(&mut mollusk);

        // Define the mint and accounts for the test.
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

        // Define token accounts for the signer and recipient.
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

        // Initialize the token accounts with balances.
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

        // Verify the ownership of the token accounts.
        assert_eq!(signer_ta_account.owner(), &spl_token::id());
        assert_eq!(recipient_ta_account.owner(), &spl_token::id());

        // Define the transfer amount and construct the instruction data.
        let amount = 1_000_u64;
        let data = amount.to_le_bytes();

        // Construct the transfer instruction.
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

        // Process the instruction using the Mollusk virtual machine.
        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (signer_ta, signer_ta_account.clone()),
                (recipient_ta, recipient_ta_account.clone()),
                (signer, signer_account.clone()),
                (token_program, token_program_account.clone()),
            ],
        );

        // Assert that the instruction was processed successfully.
        assert!(
            !result.program_result.is_err(),
            "Error while processing instruction",
        );
    }
}