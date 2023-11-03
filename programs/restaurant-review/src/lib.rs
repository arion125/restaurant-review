use anchor_lang::prelude::*;

declare_id!("J6oMR1YjgRnP1tc2eAgWsTWBA7kCeAGT7wECjSJKcZun");

#[program]
pub mod restaurant_review {
    use super::*;

    pub fn create_review(
        ctx: Context<ReviewAccounts>,
        restaurant: String,
        location: String,
        review: String,
        rating: u8,
    ) -> Result<()> {
        let new_review = &mut ctx.accounts.review;
        new_review.reviewer = ctx.accounts.signer.key();
        new_review.restaurant = restaurant;
        new_review.review = review;
        new_review.rating = rating;

        msg!(
            "Restaurant review for {} - {} stars",
            new_review.restaurant,
            new_review.rating
        );
        msg!("Review: {}", new_review.review);
        Ok(())
    }

    pub fn update_review(
        ctx: Context<ReviewAccounts>,
        _restaurant: String,
        _location: String,
        review: String,
        rating: u8,
    ) -> Result<()> {
        let existing_review = &mut ctx.accounts.review;
        existing_review.review = review;
        existing_review.rating = rating;
        msg!(
            "Restaurant review for {} - {} stars",
            existing_review.restaurant,
            existing_review.rating
        );
        msg!("Review: {}", existing_review.review);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(restaurant: String, location: String)]
pub struct ReviewAccounts<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = 500,
        seeds = [restaurant.as_bytes().as_ref(), location.as_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub review: Account<'info, Review>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Review {
    pub reviewer: Pubkey,
    pub restaurant: String,
    pub location: String,
    pub review: String,
    pub rating: u8,
}
