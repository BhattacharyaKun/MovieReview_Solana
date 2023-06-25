use anchor_lang::prelude::*;

declare_id!("HxJucZqqcNUKiZcU2HkyvQkP179DmjwEEsZ6hEX4RhUv");

#[program]
pub mod backend_movie_review 
{
    use super::*;

    pub fn add_movie(ctx: Context<AddMovie>, title: String, description: String, review: u8) -> Result<()> 
    {
        let user_account = &mut ctx.accounts.movie_account;

        if review > 5
        {
            panic!("Review should be between 0 and 5");
        }

        user_account.title = title;
        user_account.description = description;
        user_account.review = review;
        user_account.comments = 0;

        Ok(())
    }

    pub fn edit_movie(ctx: Context<EditMovie>, title: String, description: String, review: u8) -> Result<()> 
    {
        let user_account = &mut ctx.accounts.movie_account;

        if review > 5
        {
            panic!("Review should be between 0 and 5");
        }

        user_account.title = title;
        user_account.description = description;
        user_account.review = review;

        Ok(())
    }
    
    pub fn add_comment(ctx: Context<AddComment>, _title: String, comment: String, comment_id: String) -> Result<()> 
    {
        let comment_account = &mut ctx.accounts.comment_account;
        let movie_account = &mut ctx.accounts.movie_account;

        comment_account.comment = comment;
        comment_account.comment_id = comment_id;
        movie_account.comments = movie_account.comments.checked_add(1).unwrap();

        Ok(())
    }

    pub fn edit_comment(ctx: Context<EditComment>, _title: String, comment: String, _comment_id: String) -> Result<()> 
    {
        let comment_account = &mut ctx.accounts.comment_account;

        comment_account.comment = comment;

        Ok(())
    }

    pub fn close_comment(ctx: Context<CloseComment>, _title: String, _comment_id: String) -> Result<()> 
    {
        let movie_account = &mut ctx.accounts.movie_account;

        movie_account.comments = movie_account.comments.checked_sub(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct AddMovie<'info>
{
    #[account(init, payer = user, seeds=[b"movie_account", user.key().as_ref(), title.as_ref()], bump, space = 8 + 4 + title.len() + 4 + description.len() + 1 + 1)]
    movie_account: Account<'info, MovieAccount>,
    
    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct EditMovie<'info>
{
    #[account(
        mut, 
        seeds=[b"movie_account", user.key().as_ref(), title.as_ref()], 
        bump, 
        realloc = 8 + 4 + title.len() + 4 + description.len() + 1 + 1, 
        realloc::payer = user,
        realloc::zero = false        
    )]
    movie_account: Account<'info, MovieAccount>,
    
    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title: String, comment: String, comment_id: String)]
pub struct AddComment<'info>
{
    #[account(mut, seeds=[b"movie_account", user.key().as_ref(), title.as_ref()], bump)]
    movie_account: Account<'info, MovieAccount>,

    #[account(init, payer = user, seeds=[b"comment_account", user.key().as_ref(), comment_id.as_ref(), movie_account.key().as_ref()], bump, space = 8 + 4 + comment.len() + 4 + comment_id.len())]
    comment_account: Account<'info, CommentAccount>,
    
    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title: String, comment: String, comment_id: String)]
pub struct EditComment<'info>
{
    #[account(mut, seeds=[b"movie_account", user.key().as_ref(), title.as_ref()], bump)]
    movie_account: Account<'info, MovieAccount>,

    #[account(
        mut, 
        seeds=[b"comment_account", user.key().as_ref(), comment_id.as_ref(), movie_account.key().as_ref()], 
        bump, 
        realloc = 8 + 4 + comment.len() + 4 + comment_id.len(),
        realloc::payer = user, 
        realloc::zero = false
    )]
    comment_account: Account<'info, CommentAccount>,
    
    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title: String, comment_id: String)]
pub struct CloseComment<'info>
{
    #[account(mut, seeds=[b"movie_account", user.key().as_ref(), title.as_ref()], bump)]
    movie_account: Account<'info, MovieAccount>,

    #[account(
        mut, 
        seeds=[b"comment_account", user.key().as_ref(), comment_id.as_ref(), movie_account.key().as_ref()], 
        bump, 
        close = user
    )]
    comment_account: Account<'info, CommentAccount>,
    
    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>
}

#[account]
pub struct MovieAccount
{
    title: String,
    description: String,
    review: u8,
    comments: u8
}

#[account]
pub struct CommentAccount
{
    comment_id: String,
    comment: String
}
