use anchor_lang::prelude::*;

declare_id!("4VPiS6RyDfAooQyxfW2sbjaEFENCKLTSsVwoiEz8tEZm");

#[program]
pub mod anchor_exchange_hub {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
