use anchor_lang::prelude::*;
use anchor_spl::token::{burn, Burn, Mint, Token, TokenAccount};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("9fxRCMa1BgFfYJKwnmoBAaUEAz4eQ4zcD2pHMg9pHLLb");

const PROPOSAL_SEED: &[u8] = b"proposal pda";
const OPTION_SEED: &[u8] = b"option pda";
const PROPOSAL_ACCOUNT: &[u8] = b"proposal account";

#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Proposal Creator minted");
        Ok(())
    }

    pub fn mint_proposal(ctx: Context<MintProposal>, details: ProposalParams) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.maker = ctx.accounts.payer.key();
        proposal.description = details.description;
        proposal.uri = details.uri;

        let options = &mut ctx.accounts.options;
        let option_list_len = details.options.len();
        options.option_number = option_list_len;
        options.option_list = details.options;
        options.vote_list = vec![0u64; option_list_len];

        let proposal_account = &mut ctx.accounts.proposal_account;
        proposal_account.options_list = options.key();
        proposal_account.proposal_details = proposal.key();
        proposal_account.start_time = details.start_time;
        proposal_account.end_time = details.end_time;

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, option_number: usize) -> Result<()> {
        require!(
            ctx.accounts.options.option_number <= option_number,
            ProposalError::InvalidOption
        );
        require!(
            ctx.accounts.proposal_account.start_time >= Clock::get()?.unix_timestamp,
            ProposalError::NotStarted
        );
        require!(
            ctx.accounts.proposal_account.end_time <= Clock::get()?.unix_timestamp,
            ProposalError::HasEnded
        );

        let options = &mut ctx.accounts.options;
        options.vote_list[option_number] += 1;

        let burn_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.from.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        );

        burn(burn_ctx, 1)?;

        Ok(())
    }
}

#[account]
pub struct ProposalParams {
    pub description: String,
    pub uri: String,
    pub options: Vec<String>,
    pub start_time: i64,
    pub end_time: i64,
}

#[account]
pub struct ProposalAccount {
    pub proposal_details: Pubkey,
    pub options_list: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
}

#[account]
pub struct ProposalDetails {
    pub maker: Pubkey,
    pub description: String,
    pub uri: String,
}

#[account]
pub struct OptionsList {
    pub option_number: usize,
    pub option_list: Vec<String>,
    pub vote_list: Vec<u64>,
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(details: ProposalParams)]
pub struct MintProposal<'info> {
    #[account(init, seeds = [PROPOSAL_ACCOUNT, payer.key.as_ref()], bump, payer = payer, space = 8 + 32 + 32 + 8 + 8)]
    pub proposal_account: Account<'info, ProposalAccount>,
    #[account(init, seeds = [PROPOSAL_SEED, proposal_account.key().as_ref()], bump, payer = payer, space = 8 + 32 + ( 4 + 100 ) + ( 4 + 110 ) + 32)]
    pub proposal: Account<'info, ProposalDetails>,
    #[account(init, seeds = [OPTION_SEED, proposal_account.key().as_ref()], bump, payer = payer, space = 8 + 4 + ( ( 4 + 70 ) * 10 ) + ( 8 * 10 ))]
    pub options: Account<'info, OptionsList>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(option_number: usize)]
pub struct Vote<'info> {
    pub proposal_account: Account<'info, ProposalAccount>,
    #[account(mut, seeds = [OPTION_SEED, proposal_account.key().as_ref()], bump)]
    pub options: Account<'info, OptionsList>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ProposalError {
    #[msg("No such option exists")]
    InvalidOption,
    #[msg("Proposal voting not started")]
    NotStarted,
    #[msg("Proposal voting has ended")]
    HasEnded,
}
