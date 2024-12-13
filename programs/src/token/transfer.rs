use pinocchio::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};
use pinocchio_token::instructions::Transfer;

const ID: [u8; 32] = five8_const::decode_32_const("77777777777777777777777777777777777777777777");
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = unsafe { *(data.as_ptr() as *const u64) };
    process_transfer(accounts, amount, program_id)
}

/// Processes the Transfer instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts involved in the transfer.
/// - `amount`: The amount of tokens to transfer.
/// - `program_id`: The ID of the current program.
///
/// ### Accounts:
///   0. `[WRITE]` The sender account.
///   1. `[WRITE]` The recipient account.
///   2. `[SIGNER]` The authority that approves the transfer.
pub fn process_transfer(
    accounts: &[AccountInfo],
    amount: u64,
    program_id: &Pubkey,
) -> ProgramResult {
    let [sender_account, recipient_account, authority_account, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate that the sender account is writable
    if !sender_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }
    msg!("Sender account is writable");

    // Validate that the recipient account is writable
    if !recipient_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }
    msg!("Recipient account is writable");

    // Validate the sender and recipient accounts are owned by the program
    if sender_account.owner() != token_program.key()
        || recipient_account.owner() != token_program.key()
    {
        return Err(ProgramError::IncorrectProgramId);
    }
    msg!("Sender and recipient accounts are owned by the program");

    // Validate the authority is a signer
    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    msg!("Authority is a signer");

    // Construct the Transfer instruction
    let transfer_instruction = Transfer {
        from: sender_account,
        to: recipient_account,
        authority: authority_account,
        amount,
    };
    msg!("Created the instruction instance");

    // Invoke the instruction
    transfer_instruction.invoke()?;
    msg!("Invoked the instruction");

    Ok(())
}

#[cfg(test)]
mod tests {
    use mollusk_svm::{result::Check, Mollusk};
    use pinocchio_token::state::TokenAccount;
    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount, WritableAccount},
        clock::Clock,
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
        sysvar::Sysvar,
    };
    use spl_token::state::AccountState;

    #[test]
    fn process_advance_nonce_account_test() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "77777777777777777777777777777777777777777777",
        ));
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();
        let mut mollusk = Mollusk::new(&program_id, "target/deploy/programs");
        mollusk_token::token::add_program(&mut mollusk);

        let mint = Pubkey::new_from_array(five8_const::decode_32_const(
            "44444444444444444444444444444444444444444444",
        ));

        let signer = Pubkey::new_unique();
        let signer_account = create_account(
            1_000_000_000 * 10,
            spl_token::state::Account::LEN,
            &program_id,
        );
        println!("signer_account balance: {:?}", signer_account.lamports());

        let recipient = Pubkey::new_unique();
        let recipient_account = create_account(
            1_000_000_000 * 10,
            spl_token::state::Account::LEN,
            &program_id,
        );
        println!(
            "recipient_account balance: {:?}",
            recipient_account.lamports()
        );

        let signer_ta = Pubkey::new_from_array(five8_const::decode_32_const(
            "33333333333333333333333333333333333333333333",
        ));
        let recipient_ta = Pubkey::new_from_array(five8_const::decode_32_const(
            "11111111111111111111111111111111111111111111",
        ));

        let signer_ta_account: AccountSharedData = pack_token_account(&signer, &mint, 1_000_000);
        let recipient_ta_account: AccountSharedData =
            pack_token_account(&recipient, &mint, 1_000_000);

        assert_eq!(signer_ta_account.owner(), &spl_token::id());
        assert_eq!(recipient_ta_account.owner(), &spl_token::id());

        let amount = 1_000_u64;
        let data = amount.to_le_bytes();

        println!("data: {:?}", data);

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
    }

    fn create_account(lamports: u64, data_len: usize, owner: &Pubkey) -> AccountSharedData {
        AccountSharedData::new(lamports, data_len, owner)
    }

    fn pack_mint(mint_authority: &Pubkey, supply: u64) -> AccountSharedData {
        let mut account = create_account(0, spl_token::state::Mint::LEN, &spl_token::id());
        spl_token::state::Mint {
            mint_authority: COption::Some(*mint_authority),
            supply,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::None,
        }
        .pack_into_slice(account.data_as_mut_slice());
        account
    }

    fn pack_token_account(owner: &Pubkey, mint: &Pubkey, amount: u64) -> AccountSharedData {
        let mut account = create_account(0, spl_token::state::Account::LEN, &spl_token::id());
        spl_token::state::Account {
            mint: *mint,
            owner: *owner,
            amount,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        }
        .pack_into_slice(account.data_as_mut_slice());
        account
    }
}
