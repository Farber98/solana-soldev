use anchor_lang::prelude::*;

declare_id!("EvyMLMUiSLkBPtuG94vbMViADdKHwKgoc4iwrJqv5Baq");

#[program]
pub mod movie_review_program {
    use super::*;

    pub fn delete_movie_review(
        ctx: Context<DeleteMovieReview>,
        title: String,
    ) -> Result<()> {
        msg!("Movie review for {} deleted", title);
        Ok(())
    }

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        msg!("Movie Review Account Created");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.title = title;
        movie_review.reviewer = ctx.accounts.reviewer.key();
        movie_review.description = description;
        movie_review.rating = rating;
        Ok(())
    }
    
    pub fn update_movie_review(
        ctx: Context<UpdateMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        msg!("Updating {} Movie Review Account", title);
        
        let movie_review = &mut ctx.accounts.movie_review;
        
        msg!("Previous Description: {}", movie_review.description);
        msg!("Previous Rating: {}", movie_review.rating);

        movie_review.reviewer = ctx.accounts.reviewer.key();
        movie_review.description = description;
        movie_review.rating = rating;
        
        msg!("New Description: {}", movie_review.description);
        msg!("New Rating: {}", movie_review.rating);
        
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct AddMovieReview<'info> {
    #[account(
        init, 
        seeds = [reviewer.key().as_ref(), title.as_bytes()],
        bump,
        payer = reviewer, 
        space = 8 + 32 + 1 + 4 + title.len() + 4 + description.len()
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub reviewer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut,
        seeds = [reviewer.key().as_ref(), title.as_bytes()],
        bump,
        realloc = 8 + 32 + 1 + 4 + title.len() + 4 + description.len(),
        realloc::payer = reviewer,
        realloc::zero = true,
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub reviewer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteMovieReview<'info> {
    #[account(
        mut,
        seeds = [reviewer.key().as_ref(), title.as_bytes()],
        bump,
        close = reviewer,
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub reviewer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MovieAccountState {
    reviewer: Pubkey,    // 32
    rating: u8,          // 1
    title: String,       // 4 + title.len()
    description: String, // 4 + description.len()
}
