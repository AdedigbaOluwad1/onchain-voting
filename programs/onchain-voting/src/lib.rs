use anchor_lang::prelude::*;

declare_id!("6k4ComXZa7CVTR4ghW4fqds2zFCdtGKMXAPDWvxTUFMJ");

#[program]
pub mod onchain_voting {
    use super::*;

    pub fn create_vote_bank(ctx: Context<InitVote>) -> Result<()> {
        ctx.accounts.vote_account.is_open_to_vote = true;
        msg!("Voting bank initialized!");

        Ok(())
    }

    pub fn cast_vote(ctx: Context<GibVote>, vote_type: VoteType) -> Result<()> {
        match vote_type {
            VoteType::GM => {
                msg!("Voted for GM ☀️");
                ctx.accounts.vote_account.gm += 1;
            }
            VoteType::GN => {
                msg!("Voted for GN 🌑");
                ctx.accounts.vote_account.gn += 1;
            }
        };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVote<'info> {
    #[account(
    init,
    payer = signer,
    space = 8 + 1 + 8 + 8,
  )]
    pub vote_account: Account<'info, VoteBank>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct VoteBank {
    is_open_to_vote: bool,
    gm: u64,
    gn: u64,
}

#[derive(Accounts)]
pub struct GibVote<'info> {
    // we are going to store users vote in this account. Hence marking it as mutable(mut)
    #[account(mut)]
    pub vote_account: Account<'info, VoteBank>,
    pub signer: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VoteType {
    GM,
    GN,
}
