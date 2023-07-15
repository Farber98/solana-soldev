use anchor_lang::prelude::*;

declare_id!("4RR9VmpLb2HLK7iKyPKz2gG1DvP4rsad13wZrppynmHy");

#[program]
pub mod student_program {
    use super::*;

    pub fn add_student_greeting(ctx: Context<AddStudentGreeting>, 
        student: String, 
        message: String) -> Result<()> {
            msg!("Student Greeting Account Created");
            msg!("Student: {}", student);
            msg!("Message: {}", message);

            let student_greeting = &mut ctx.accounts.student_greeting;
            student_greeting.user = ctx.accounts.user.key();
            student_greeting.student = student;
            student_greeting.message = message;

            Ok(())
        }

    pub fn update_student_greeting(ctx: Context<UpdateStudentGreeting>, 
        student: String, 
        message: String) -> Result<()> {
            msg!("Updating {} Student Account", student);

            let student_greeting = &mut ctx.accounts.student_greeting;
            
            msg!("Previous message: {}", student_greeting.message);

            student_greeting.message = message;

            msg!("New message: {}", student_greeting.message);

            Ok(())

    }

    pub fn delete_student_greeting(ctx: Context<DeleteStudentGreeting>, 
        student: String) -> Result<()> {
            msg!("Student greeting for {} deleted", student);
            Ok(())
    }
}


#[account]
pub struct StudentAccountState {
    user: Pubkey, // 32
    student: String, // 4 + student.len()
    message: String, // 4 + message.len()
}

#[derive(Accounts)]
#[instruction(student: String, message: String)]
pub struct AddStudentGreeting<'info> {
    #[account(
        init, 
        seeds = [user.key().as_ref(), student.as_bytes()],
        bump,
        payer = user, 
        space = 8 + 32 + 4 + student.len() + 4 + message.len()
    )]
    pub student_greeting: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(student: String, message: String)]
pub struct UpdateStudentGreeting<'info> {
    #[account(
        mut, 
        seeds = [user.key().as_ref(), student.as_bytes()],
        bump,
        realloc = 8 + 32 + 4 + student.len() + 4 + message.len(),
        realloc::payer = user,
        realloc::zero = true,
    )]
    pub student_greeting: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(student: String)]
pub struct DeleteStudentGreeting<'info> {
    #[account(
        mut, 
        seeds = [user.key().as_ref(), student.as_bytes()],
        bump,
        close = user
    )]
    pub student_greeting: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

