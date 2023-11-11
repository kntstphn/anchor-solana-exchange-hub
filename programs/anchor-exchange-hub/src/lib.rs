use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("4VPiS6RyDfAooQyxfW2sbjaEFENCKLTSsVwoiEz8tEZm");

#[program]
pub mod anchor_exchange_hub {
    use super::*;

    pub fn create_post(ctx: Context<CreatePost>, topic: String, content: String) -> Result<()> {
        let post: &mut Account<Post> = &mut ctx.accounts.post;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get()?;

        post.author = *author.key;
        post.topic = topic;
        post.content = content;
        post.likes = 0;
        post.timestamp = clock.unix_timestamp;
        post.upvotes = 0;

        Ok(())
    }

    // pub fn upvote_post(ctx: Context<UpvotePost>) -> Result<()> {
    //     let post: &mut Account<Post> = &mut ctx.accounts.post;
    //     let user: &Signer = &ctx.accounts.user;

    //     if post.author == *user.key {
    //         return Err(ErrorCode::CannotUpvoteOwnPost.into());
    //     }

    //     if
    //         ctx.accounts.user_account.upvoted_posts
    //             .iter()
    //             .any(|&post_key| post_key == *post.to_account_info().key)
    //     {
    //         return Err(ErrorCode::AlreadyUpvoted.into());
    //     }

    //     post.upvotes += 1;

    //     ctx.accounts.user_account.upvoted_posts.push(*post.to_account_info().key);

    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = author, space = 264)]
    pub post: Account<'info, Post>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Post {
    pub author: Pubkey,
    pub topic: String,
    pub content: String,
    pub likes: u64,
    pub timestamp: i64,
    pub upvotes: i64,
}
// #[derive(Accounts)]
// pub struct UpvotePost<'info> {
//     #[account(mut)]
//     pub post: Account<'info, Post>,
//     #[account(mut)]
//     pub user_account: Account<'info, UserAccount>,
//     #[account(signer)]
//     pub user: Signer<'info>,
// }

// #[account]
// pub struct UserAccount {
//     pub upvoted_posts: Vec<Pubkey>,
// }

// #[error_code]
// pub enum ErrorCode {
//     #[msg("Cannot upvote your own post")]
//     CannotUpvoteOwnPost,
//     #[msg("Already upvoted this post")]
//     AlreadyUpvoted,
// }
