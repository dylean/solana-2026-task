#![no_std]

use pinocchio::{
    entrypoint,
    nostd_panic_handler,
    error::ProgramError,
    Address,
    AccountView,
    ProgramResult,
};

entrypoint!(process_instruction);
nostd_panic_handler!();

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

/// 程序 ID: 22222222222222222222222222222222222222222222
pub const ID: Address = Address::new_from_array([
    0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07,
    0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb, 0xee,
    0x19, 0x92, 0xba, 0xe8, 0xaf, 0xd1, 0xcd, 0x07,
    0x8e, 0xf8, 0xaf, 0x70, 0x47, 0xdc, 0x11, 0xf7,
]);

/// 程序入口点
fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((0, data)) => initialize(program_id, data, accounts),
        Some((1, data)) => deposit(program_id, data, accounts),
        Some((2, data)) => withdraw(program_id, data, accounts),
        Some((3, data)) => swap(program_id, data, accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
