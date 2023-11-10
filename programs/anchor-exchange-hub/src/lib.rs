use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("4VPiS6RyDfAooQyxfW2sbjaEFENCKLTSsVwoiEz8tEZm");

#[program]
pub mod anchor_exchange_hub {
    use super::*;

    pub fn send_post(ctx: Context<SendPost>, topic: String, content: String) -> Result<()> {
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
}

#[derive(Accounts)]
pub struct SendPost<'info> {
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
    pub upvotes: u64,
}
