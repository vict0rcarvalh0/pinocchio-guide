use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::instructions::Transfer;

// A constant representing the program ID, decoded from a base58 string.
// const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");

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
    // Ensure the data length is valid.
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Parse the amount from the data.
    let amount = unsafe { *(data.as_ptr() as *const u64) };

    // Process the transfer instruction.
    process_transfer(accounts, amount)
}

/// Processes the `Transfer` instruction.
///
/// This function handles the logic for transferring tokens between accounts. It validates
/// the accounts, constructs the transfer instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts involved in the transfer.
/// - `amount`: The amount of tokens to transfer.
///
/// ### Accounts:
/// 0. `[WRITE]` The sender account.
/// 1. `[WRITE]` The recipient account.
/// 2. `[SIGNER]` The authority that approves the transfer.
/// 3. `[]` The token program.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_transfer(
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [sender_account, recipient_account, authority_account, token_program] = accounts else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate that the sender and recipient accounts are writable.
    assert!(
        sender_account.is_writable(),
        "Sender account is not writable"
    );
    assert!(
        recipient_account.is_writable(),
        "Recipient account is not writable"
    );

    // Validate that the sender and recipient accounts are owned by the token program.
    assert_eq!(
        sender_account.owner(),
        token_program.key(),
        "Sender account is not owned by the token program"
    );
    assert_eq!(
        recipient_account.owner(),
        token_program.key(),
        "Recipient account is not owned by the token program"
    );

    // Validate that the authority account is a signer.
    assert!(authority_account.is_signer(), "Authority is not a signer");

    // Construct the `Transfer` instruction.
    let transfer_instruction = Transfer {
        from: sender_account,
        to: recipient_account,
        authority: authority_account,
        amount,
    };

    // Invoke the transfer instruction.
    transfer_instruction.invoke()?;

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

    /// Tests the `Transfer` instruction.
    ///
    /// This test sets up a mock environment, creates accounts, and verifies that the
    /// transfer instruction processes successfully.
    #[test]
    fn transfer_test() {
        // Define the program ID.
        let program_id = Pubkey::new_from_array([
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01, 0x01, 0x01,
        ]);

        // Initialize the token program and Mollusk environment.
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();
        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/programs");
        mollusk_token::token::add_program(&mut mollusk);

        // Define the mint and accounts.
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

        // Ensure the accounts are owned by the token program.
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

        // Process the instruction and validate the result.
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
