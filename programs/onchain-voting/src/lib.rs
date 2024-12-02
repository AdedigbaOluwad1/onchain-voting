use anchor_lang::prelude::*;

declare_id!("6k4ComXZa7CVTR4ghW4fqds2zFCdtGKMXAPDWvxTUFMJ");

#[program]
pub mod onchain_voting {
    use super::*;

    pub fn init_vote_registry(ctx: Context<InitVoteRegistry>) -> Result<()> {
        let vote_registry = &mut ctx.accounts.vote_registry;
        vote_registry.vote_accounts = Vec::new();

        msg!(&vote_registry.key().to_string());

        Ok(())
    }

    pub fn create_vote_bank(ctx: Context<InitVoteAccount>) -> Result<()> {
        ctx.accounts.vote_account.is_open_to_vote = true;
        ctx.accounts.vote_account.vote_options = Vec::new();
        ctx.accounts.vote_account.voters = Vec::new();
        ctx.accounts.vote_account.creator = ctx.accounts.signer.key();
        // change to current date / time
        ctx.accounts.vote_account.vote_bank_closes_in = 30;

        ctx.accounts
            .vote_registry
            .vote_accounts
            .push(ctx.accounts.vote_account.key());

        msg!(&ctx.accounts.vote_registry.vote_accounts.len().to_string());
        msg!("Voting bank initialized!");

        Ok(())
    }

    pub fn add_option_to_vote(ctx: Context<UpdateVote>, option_name: String) -> Result<()> {
        let accounts = ctx.accounts;
        let vote_account = &mut accounts.vote_account;

        if vote_account.creator != accounts.signer.key() {
            msg!("Vote account creator is not signer");
            return err!(VoteErrors::InvalidPermissions);
        }

        let new_vote_key = vote_account.vote_options.len() + 1;

        let new_vote_options = VoteOption {
            option_count: 0,
            option_name,
            option_id: new_vote_key as u64,
        };

        vote_account.vote_options.push(new_vote_options);

        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, vote_option_id: u64) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;

        if vote_account.voters.contains(&ctx.accounts.signer.key()) {
            err!(VoteErrors::DoubleVoteError)?;
        }

        let selected_vote_option = vote_account
            .vote_options
            .iter_mut()
            .find(|vote_option| vote_option.option_id == vote_option_id);

        match selected_vote_option {
            Some(vote_option) => {
                vote_option.cast_vote();
                vote_account.voters.push(ctx.accounts.signer.key());
                return Ok(());
            }
            None => Err(VoteErrors::InvalidVoteOption.into()),
        }
    }
}

#[derive(Accounts)]
pub struct InitVoteRegistry<'info> {
    #[account(
      init, // Indicates this account will be created
      payer = signer,
      space = 128, // Space needed to store the serialized data
      seeds = [b"seeds".as_ref()], // Optional seed for PDA derivation
      bump // The bump seed for the PDA
  )]
    pub vote_registry: Account<'info, VoteRegistry>, // Account to be initialized

    #[account(mut)] // Indicates this account can be written to (mutable)
    pub signer: Signer<'info>, // The payer/signer's account
    pub system_program: Program<'info, System>, // Reference to the System Program
}

#[derive(Accounts)]
pub struct InitVoteAccount<'info> {
    #[account(
  init,
  payer = signer,
  space = 8 + 1 + 8 + 32 + 10 + 8 + 8 + 32 + (4 + 50 * 16) + (4 + 100 * 32),
)]
    pub vote_account: Account<'info, VoteBank>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
      mut,
      seeds = [b"seeds".as_ref()], // Optional seed for PDA derivation
      bump // The bump seed for the PDA
  )]
    pub vote_registry: Account<'info, VoteRegistry>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateVote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteBank>,
    pub signer: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct VoteRegistry {
    // Store a list of active vote accounts
    pub vote_accounts: Vec<Pubkey>, // Map vote ID to the vote account's public key
}

#[account]
#[derive(Default)]
pub struct VoteBank {
    is_open_to_vote: bool,
    vote_bank_closes_in: u64,
    voters: Vec<Pubkey>,
    vote_options: Vec<VoteOption>,
    creator: Pubkey,
}

#[derive(Clone, Default, AnchorSerialize, AnchorDeserialize)]
pub struct VoteOption {
    pub option_name: String,
    pub option_count: u64,
    pub option_id: u64,
}

impl VoteOption {
    // fn set_option_name(&mut self, name: String) {
    //     self.option_name = name
    // }
    fn cast_vote(&mut self) {
        self.option_count += 1
    }
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    // we are going to store users vote in this account. Hence marking it as mutable(mut)
    #[account(mut)]
    pub vote_account: Account<'info, VoteBank>,
    pub signer: Signer<'info>,
}

#[error_code]
pub enum VoteErrors {
    #[msg("Vote option not found")]
    InvalidVoteOption,
    #[msg("Voter already voted")]
    DoubleVoteError,
    #[msg("Invalid permissions to carry out this action")]
    InvalidPermissions,
}
