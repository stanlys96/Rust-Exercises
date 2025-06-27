use anchor_lang::prelude::*;

declare_id!("CLNocC4Kp9sc4rLDxWyjFRXpZqhaJj8SR6GNxPAWYAYE");

#[program]
pub mod voting {
    use super::*;

    pub fn init_voting(ctx: Context<InitVoting>) -> Result<()> {
        ctx.accounts.vote_account.has_started = true;
        Ok(())
    }

    pub fn give_voting(ctx: Context<GiveVoting>, vote_type: VoteType) -> Result<()> {
        match vote_type {
            VoteType::GM => {
                msg!("GM chosen!");
                ctx.accounts.vote_account.gm += 1;
            }
            VoteType::GN => {
                msg!("GN chosen!");
                ctx.accounts.vote_account.gn += 1;
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVoting<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 1 + 8 + 8,
        seeds = [b"vote"],
        bump
    )]
    pub vote_account: Account<'info, Vote>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GiveVoting<'info> {
    #[account(
        mut,
        seeds = [b"vote"],
        bump
    )]
    pub vote_account: Account<'info, Vote>,

    pub signer: Signer<'info>,
}

#[account]
pub struct Vote {
    pub has_started: bool,
    pub gm: u64,
    pub gn: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VoteType {
    GM,
    GN,
}
