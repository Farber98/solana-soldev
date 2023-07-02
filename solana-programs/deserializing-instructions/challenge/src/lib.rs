pub mod instruction;
use instruction::StudentInstruction;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// Entry point is a function call process_instruction
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Call unpack to deserialize instruction_data
    let instruction = StudentInstruction::unpack(instruction_data)?;
    // Match the returned data struct to what you expect
    match instruction {
        StudentInstruction::AddStudentGreeting { name, message } => {
            add_student_greeting(_program_id, _accounts, name, message)
        }
    }
}

pub fn add_student_greeting(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    name: String,
    message: String,
) -> ProgramResult {
    // Logging instruction data that was passed in
    msg!("Adding student greeting...");
    msg!("Student: {}", name);
    msg!("Greeting: {}", message);

    Ok(())
}
