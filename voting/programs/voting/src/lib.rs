use anchor_lang::prelude::*;

declare_id!("CLNocC4Kp9sc4rLDxWyjFRXpZqhaJj8SR6GNxPAWYAYE");

#[program]
pub mod voting {
    use super::*;

    pub fn init_voting(ctx: Context<InitVoting>) -> Result<()> {
        ctx.accounts.vote.has_initiated = true;
        Ok(())
    }

    pub fn give_vote(ctx: Context<GiveVote>, vote_type: VoteType) -> Result<()> {
        match vote_type {
            VoteType::GM => {
                msg!("Voted for GM! 🤝");
                ctx.accounts.vote.gm += 1;
            }
            VoteType::GN => {
                msg!("Voted for GN! 🤞");
                ctx.accounts.vote.gn += 1;
            }
        };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVoting<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 1 + 8 + 8,
        seeds = [b"vote", signer.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, Vote>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Vote {
    pub has_initiated: bool,
    pub gn: u64,
    pub gm: u64,
}

#[derive(Accounts)]
pub struct GiveVote<'info> {
    #[account(
        mut,
        seeds = [b"vote", signer.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, Vote>,

    pub signer: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VoteType {
    GM,
    GN,
}
