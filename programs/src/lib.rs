pub mod system;
pub mod token;

use pinocchio::{
    entrypoint,
    error::ProgramError,
    AccountView,
    Address,
    ProgramResult,
};

// Instruction discriminators
const SYSTEM_ASSIGN: u8 = 0;
const SYSTEM_TRANSFER: u8 = 1;
const SYSTEM_CREATE_ACCOUNT: u8 = 2;
const SYSTEM_ALLOCATE: u8 = 3;
const SYSTEM_ALLOCATE_WITH_SEED: u8 = 4;
const SYSTEM_ASSIGN_WITH_SEED: u8 = 5;
const SYSTEM_CREATE_ACCOUNT_WITH_SEED: u8 = 6;
const SYSTEM_ADVANCE_NONCE: u8 = 7;
const SYSTEM_WITHDRAW_NONCE: u8 = 8;
const SYSTEM_INIT_NONCE: u8 = 9;
const SYSTEM_AUTHORIZE_NONCE: u8 = 10;
const SYSTEM_TRANSFER_WITH_SEED: u8 = 11;
const SYSTEM_UPDATE_NONCE: u8 = 12;

const TOKEN_INIT_MINT: u8 = 20;
const TOKEN_INIT_ACCOUNT: u8 = 21;
const TOKEN_TRANSFER: u8 = 22;
const TOKEN_APPROVE: u8 = 23;
const TOKEN_REVOKE: u8 = 24;
const TOKEN_SET_AUTHORITY: u8 = 25;
const TOKEN_MINT_TO: u8 = 26;
const TOKEN_BURN: u8 = 27;
const TOKEN_CLOSE_ACCOUNT: u8 = 28;
const TOKEN_FREEZE: u8 = 29;
const TOKEN_THAW: u8 = 30;
const TOKEN_TRANSFER_CHECKED: u8 = 31;
const TOKEN_APPROVE_CHECKED: u8 = 32;
const TOKEN_MINT_TO_CHECKED: u8 = 33;
const TOKEN_BURN_CHECKED: u8 = 34;
const TOKEN_SYNC_NATIVE: u8 = 35;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    let discriminator = instruction_data[0];
    
    match discriminator {
        // System instructions
        SYSTEM_ASSIGN | SYSTEM_TRANSFER | SYSTEM_CREATE_ACCOUNT | SYSTEM_ALLOCATE |
        SYSTEM_ALLOCATE_WITH_SEED | SYSTEM_ASSIGN_WITH_SEED | SYSTEM_CREATE_ACCOUNT_WITH_SEED |
        SYSTEM_ADVANCE_NONCE | SYSTEM_WITHDRAW_NONCE | SYSTEM_INIT_NONCE |
        SYSTEM_AUTHORIZE_NONCE | SYSTEM_TRANSFER_WITH_SEED | SYSTEM_UPDATE_NONCE => Ok(()),
        // Token instructions
        TOKEN_INIT_MINT | TOKEN_INIT_ACCOUNT | TOKEN_TRANSFER | TOKEN_APPROVE |
        TOKEN_REVOKE | TOKEN_SET_AUTHORITY | TOKEN_MINT_TO | TOKEN_BURN |
        TOKEN_CLOSE_ACCOUNT | TOKEN_FREEZE | TOKEN_THAW | TOKEN_TRANSFER_CHECKED |
        TOKEN_APPROVE_CHECKED | TOKEN_MINT_TO_CHECKED | TOKEN_BURN_CHECKED |
        TOKEN_SYNC_NATIVE => Ok(()),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
