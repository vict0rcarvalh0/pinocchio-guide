use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    instruction::Signer,
    pubkey::Pubkey,
    ProgramResult
};

use pinocchio_system::instructions::InitializeNonceAccount;

const ID: [u8; 32] = five8_const::decode_32_const("11111111111111111111111111111111111111111111");
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    process_initialize_nonce_account(accounts, authority, signers)
}

/// Processes the `InitializeNonceAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `authority`: The public key of the entity authorized to manage the Nonce account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE]` The Nonce account.
/// 1. `[]` The recent blockhashes sysvar.
/// 2. `[]` The rent sysvar.
pub fn process_initialize_nonce_account<'a>(
    accounts: &'a [AccountInfo],
    authority: &'a Pubkey,   // Pubkey representing the entity authorized to interact with the nonce account.
    signers: &[Signer],      // Signers array needed to authorize the transaction.
) -> ProgramResult {
    // Extracting account information
    let [nonce_account, recent_blockhashes_sysvar, rent_sysvar] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that nonce account is writable
    assert!(nonce_account.is_writable());

    // Creating the instruction instance
    let initialize_nonce_account_instruction = InitializeNonceAccount {
        account: nonce_account,
        recent_blockhashes_sysvar,
        rent_sysvar,
        authority,
    };

    // Invoking the instruction
    initialize_nonce_account_instruction.invoke_signed(signers)?;

    Ok(())
}