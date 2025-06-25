use anchor_lang::prelude::*;

declare_id!("7SzXuda6cnHmUGk4VvxQdNShrhada93X9cfAAVVVXynw");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
