use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};
use pinocchio_system::instructions::AdvanceNonceAccount;

const ID: [u8; 32] = five8_const::decode_32_const("77777777777777777777777777777777777777777777");
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    msg!("process_instruction");
    process_advance_nonce_account(accounts)
}

/// Processes the `AdvanceNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[]` The recent blockhashes sysvar.
/// 2. `[SIGNER]` The Nonce authority.
pub fn process_advance_nonce_account<'a>(accounts: &'a [AccountInfo]) -> ProgramResult {
    let [nonce_account, recent_blockhashes_sysvar, nonce_authority, _system_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    msg!("process_advance_nonce_account");

    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    msg!("Nonce authority is a signer");

    let advance_nonce_instruction = AdvanceNonceAccount {
        account: nonce_account,
        recent_blockhashes_sysvar,
        authority: nonce_authority,
    };
    msg!("Created the instruction instance");

    advance_nonce_instruction.invoke()?;
    msg!("Invoked the instruction");

    Ok(())
}

#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount},
        entrypoint::ProgramResult,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    #[test]
    fn process_advance_nonce_account_test() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "77777777777777777777777777777777777777777777",
        ));

        let mollusk = Mollusk::new(&program_id, "target/deploy/programs");

        let nonce_account = Pubkey::new_unique();
        let recent_blockhashes_sysvar = solana_sdk::sysvar::recent_blockhashes::ID;
        let nonce_authority = Pubkey::new_unique();
        let (system_program, system_program_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[],
            vec![
                AccountMeta::new(nonce_account, true),
                AccountMeta::new(recent_blockhashes_sysvar, true),
                AccountMeta::new(nonce_authority, true),
                AccountMeta::new(system_program, true),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    nonce_account,
                    AccountSharedData::new(
                        mollusk.sysvars.rent.minimum_balance(82),
                        82,
                        &solana_sdk::system_program::ID,
                    ),
                ),
                (
                    recent_blockhashes_sysvar,
                    AccountSharedData::new(
                        1_000_000_000u64,
                        82,
                        &solana_sdk::sysvar::recent_blockhashes::ID,
                    ),
                ),
                (
                    nonce_authority,
                    AccountSharedData::new(1_000_000_000u64, 82, &solana_sdk::system_program::ID),
                ),
                (system_program, system_program_account),
            ],
        );
        println!("Result: {:?}", result.program_result);
        assert!(
            !result.program_result.is_err(),
            "Error while processing instruction",
        );
    }
}
