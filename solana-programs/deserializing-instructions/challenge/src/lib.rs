use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;
pub mod instruction;
pub mod state;
use borsh::BorshSerialize;
use instruction::StudentInstruction;
use state::StudentAccountState;
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
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String,
) -> ProgramResult {
    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Derive PDA and check that it matches client
    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), name.as_bytes().as_ref()],
        program_id,
    );

    // Calculate account size required
    // The MovieAccountState struct has four fields.
    // 1 byte for is_initialized.
    // 4 bytes + len() each for name and message.
    let account_len: usize = 1 + 1 + (4 + name.len()) + (4 + message.len());

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    // Create the account signing with program.
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            name.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("PDA created: {}", pda);

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<StudentAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    // Logging instruction data that was passed in
    msg!("Adding student greeting...");
    msg!("Student: {}", name);
    msg!("Greeting: {}", message);
    account_data.is_initialized = true;
    account_data.name = name;
    account_data.message = message;

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}
