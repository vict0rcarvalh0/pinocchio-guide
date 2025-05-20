use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_system::instructions::AdvanceNonceAccount;

// A constant representing the program ID, decoded from a base58 string.
const ID: [u8; 32] = five8_const::decode_32_const("77777777777777777777777777777777777777777777");

// Macro to define the program's entry point.
entrypoint!(process_instruction);

/// Entry point for the program. This function is called when the program is invoked.
///
/// ### Parameters:
/// - `_program_id`: The ID of the program being executed.
/// - `accounts`: The accounts passed to the program.
/// - `_data`: Additional data passed to the program.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the program execution.
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    process_advance_nonce_account(accounts)
}

/// Processes the `AdvanceNonceAccount` instruction.
///
/// This function handles the logic for advancing a nonce account. It validates the accounts
/// and signers, constructs the instruction, and invokes it.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[]` The recent blockhashes sysvar.
/// 2. `[SIGNER]` The Nonce authority.
///
/// ### Returns:
/// - `ProgramResult`: Indicates success or failure of the instruction processing.
pub fn process_advance_nonce_account<'a>(accounts: &'a [AccountInfo]) -> ProgramResult {
    // Destructure the accounts array into individual accounts.
    let [nonce_account, recent_blockhashes_sysvar, nonce_authority, _system_program] = accounts
    else {
        // Return an error if there are not enough accounts provided.
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the nonce authority is a signer.
    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Construct the `AdvanceNonceAccount` instruction.
    let advance_nonce_instruction = AdvanceNonceAccount {
        account: nonce_account,
        recent_blockhashes_sysvar,
        authority: nonce_authority,
    };

    // Invoke the instruction.
    advance_nonce_instruction.invoke()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_sdk::{
        account::{AccountSharedData},
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    /// Unit test for the `process_advance_nonce_account` function.
    ///
    /// This test sets up a mock environment using the `Mollusk` framework, creates
    /// the necessary accounts and instruction, and verifies that the instruction
    /// processes successfully.
    #[test]
    fn process_advance_nonce_account_test() {
        // Define the program ID.
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "77777777777777777777777777777777777777777777",
        ));

        // Initialize the Mollusk virtual machine for testing.
        let mollusk = Mollusk::new(&program_id, "target/deploy/programs");

        // Create unique public keys for the accounts.
        let nonce_account = Pubkey::new_unique();
        let recent_blockhashes_sysvar = solana_sdk::sysvar::recent_blockhashes::ID;
        let nonce_authority = Pubkey::new_unique();

        // Retrieve the system program and its associated account.
        let (system_program, system_program_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        // Construct the instruction with the required accounts.
        let instruction = Instruction::new_with_bytes(
            program_id,
            &[], // No additional data is passed.
            vec![
                AccountMeta::new(nonce_account, true), // Nonce account (writable).
                AccountMeta::new(recent_blockhashes_sysvar, true), // Recent blockhashes sysvar.
                AccountMeta::new(nonce_authority, true), // Nonce authority (signer).
                AccountMeta::new(system_program, true), // System program.
            ],
        );

        // Process the instruction using the Mollusk virtual machine.
        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    nonce_account,
                    AccountSharedData::new(
                        mollusk.sysvars.rent.minimum_balance(82), // Minimum balance for rent exemption.
                        82, // Account data size.
                        &solana_sdk::system_program::ID, // Owner program ID.
                    ),
                ),
                (
                    recent_blockhashes_sysvar,
                    AccountSharedData::new(
                        1_000_000_000u64, // Mocked balance.
                        82, // Account data size.
                        &solana_sdk::sysvar::recent_blockhashes::ID, // Owner program ID.
                    ),
                ),
                (
                    nonce_authority,
                    AccountSharedData::new(
                        1_000_000_000u64, // Mocked balance.
                        82, // Account data size.
                        &solana_sdk::system_program::ID, // Owner program ID.
                    ),
                ),
                (system_program, system_program_account), // System program account.
            ],
        );

        // Print the result of the instruction processing.
        println!("Result: {:?}", result.program_result);

        // Assert that the instruction processed successfully.
        assert!(
            !result.program_result.is_err(),
            "Error while processing instruction",
        );
    }
}
