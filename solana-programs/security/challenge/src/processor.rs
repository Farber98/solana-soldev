use crate::error::StudentError;
use crate::instruction::StudentInstruction;
use crate::state::StudentAccountState;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Call unpack to deserialize instruction_data
    let instruction = StudentInstruction::unpack(instruction_data)?;
    // Match the returned data struct to what you expect
    match instruction {
        StudentInstruction::AddStudentGreeting { name, message } => {
            add_student_greeting(program_id, accounts, name, message)
        }
        StudentInstruction::UpdateStudentGreeting { message } => {
            update_student_greeting(program_id, accounts, message)
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

    // Signer Check
    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Derive PDA and check that it matches client
    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), name.as_bytes().as_ref()],
        program_id,
    );

    // Validate PDA
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(StudentError::InvalidPDA.into());
    }

    let total_len: usize = 1000;

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(total_len);

    // Create the account signing with program.
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            total_len.try_into().unwrap(), // 1000 Bytes max.
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

    // Checking if account data already initialized.
    if account_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

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

pub fn update_student_greeting(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    message: String,
) -> ProgramResult {
    msg!("Updating student greeting...");

    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    // Check program is owner of given PDA
    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    // Check Signer
    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<StudentAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("Student name: {}", account_data.name);

    // Derive PDA
    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[
            initializer.key.as_ref(),
            account_data.name.as_bytes().as_ref(),
        ],
        program_id,
    );

    // Check Derived PDA is equal to given PDA.
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(StudentError::InvalidPDA.into());
    }

    msg!("checking if student account is initialized");
    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(StudentError::UninitializedAccount.into());
    }

    let update_len: usize = 1 + 1 + (4 + message.len()) + account_data.name.len();
    if update_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(StudentError::InvalidDataLength.into());
    }

    msg!("Review before update:");
    msg!("Name: {}", account_data.name);
    msg!("Message: {}", account_data.message);

    account_data.message = message;

    msg!("Review after update:");
    msg!("Name: {}", account_data.name);
    msg!("Message: {}", account_data.message);

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}
