use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("HV1QiuAftn7JQMh9DCf7NQ5Wt3k3tnVdcbTsfKh7yW9M");

const ACCOUNT_HOLDER: &[u8] = b"account_pda";
const FOLLOWER_SEED: &[u8] = b"follower_pda";
const FOLLOWED_SEED: &[u8] = b"followed_pda";
const POST_SEED: &[u8] = b"post_pda";

#[program]
pub mod social_media {
    use super::*;

    pub fn initialize_account(ctx: Context<InitializeAccount>, name: String) -> Result<()> {
        let account_details = &mut ctx.accounts.account_details;
        account_details.owner = *ctx.accounts.owner.key;
        account_details.name = name;
        account_details.created_at = Clock::get()?.unix_timestamp;
        account_details.followers_pda = ctx.accounts.followers_list.key();
        account_details.followed_pda = ctx.accounts.followed_list.key();
        account_details.posts_pda = ctx.accounts.posts_list.key();
        Ok(())
    }

    pub fn add_follower(ctx: Context<UpdateFollowers>, follower: Pubkey) -> Result<()> {
        let followers_list = &mut ctx.accounts.followers_list;
        followers_list.followers.push(follower);
        followers_list.follower_count += 1;
        Ok(())
    }

    pub fn remove_follower(ctx: Context<UpdateFollowers>, follower: Pubkey) -> Result<()> {
        let followers_list = &mut ctx.accounts.followers_list;
        if let Some(pos) = followers_list.followers.iter().position(|&x| x == follower) {
            followers_list.followers.remove(pos);
            followers_list.follower_count -= 1;
        }
        Ok(())
    }

    pub fn add_followed(ctx: Context<UpdateFollowed>, followed: Pubkey) -> Result<()> {
        let followed_list = &mut ctx.accounts.followed_list;
        followed_list.followed.push(followed);
        followed_list.followed_count += 1;
        Ok(())
    }

    pub fn remove_followed(ctx: Context<UpdateFollowed>, followed: Pubkey) -> Result<()> {
        let followed_list = &mut ctx.accounts.followed_list;
        if let Some(pos) = followed_list.followed.iter().position(|&x| x == followed) {
            followed_list.followed.remove(pos);
            followed_list.followed_count -= 1;
        }
        Ok(())
    }

    pub fn add_post(ctx: Context<UpdatePosts>, content: String) -> Result<()> {
        let posts_list = &mut ctx.accounts.posts_list;
        posts_list.posts.push(content);
        posts_list.post_count += 1;
        Ok(())
    }
}

#[account]
pub struct AccountDetails {
    pub owner: Pubkey,
    pub name: String,
    pub created_at: i64,
    pub followers_pda: Pubkey, // PDA for followers list
    pub followed_pda: Pubkey,  // PDA for followed accounts list
    pub posts_pda: Pubkey,     // PDA for posts list
}

#[account]
pub struct FollowersList {
    pub follower_count: u64,
    pub followers: Vec<Pubkey>,
}

#[account]
pub struct FollowedList {
    pub followed_count: u64,
    pub followed: Vec<Pubkey>,
}

#[account]
pub struct PostsList {
    pub post_count: u64,
    pub posts: Vec<String>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeAccount<'info> {
    #[account(init, seeds = [ACCOUNT_HOLDER, owner.key().as_ref()], bump, payer = owner, space = 8 + 32 + 40 + 8 + 32 + 32 + 32)]
    pub account_details: Account<'info, AccountDetails>,
    #[account(init, seeds = [FOLLOWER_SEED, owner.key().as_ref()], bump, payer = owner, space = 8 + 8 + 32 * 100)]
    pub followers_list: Account<'info, FollowersList>,
    #[account(init, seeds = [FOLLOWED_SEED, owner.key().as_ref()], bump, payer = owner, space = 8 + 8 + 32 * 100)]
    pub followed_list: Account<'info, FollowedList>,
    #[account(init, seeds = [POST_SEED, owner.key().as_ref()], bump, payer = owner, space = 8 + 8 + 40 * 100)]
    pub posts_list: Account<'info, PostsList>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(follower: Pubkey)]
pub struct UpdateFollowers<'info> {
    #[account(mut, seeds = [ACCOUNT_HOLDER, owner.key().as_ref()], bump, has_one = owner)]
    pub account_details: Account<'info, AccountDetails>,
    #[account(mut, seeds = [FOLLOWER_SEED, account_details.owner.as_ref()], bump)]
    pub followers_list: Account<'info, FollowersList>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(followed: Pubkey)]
pub struct UpdateFollowed<'info> {
    #[account(mut, seeds = [ACCOUNT_HOLDER, owner.key().as_ref()], bump, has_one = owner)]
    pub account_details: Account<'info, AccountDetails>,
    #[account(mut, seeds = [FOLLOWED_SEED, account_details.owner.as_ref()], bump)]
    pub followed_list: Account<'info, FollowedList>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(content: String)]
pub struct UpdatePosts<'info> {
    #[account(mut, seeds = [ACCOUNT_HOLDER, owner.key().as_ref()], bump, has_one = owner)]
    pub account_details: Account<'info, AccountDetails>,
    #[account(mut, seeds = [POST_SEED, account_details.owner.as_ref()], bump)]
    pub posts_list: Account<'info, PostsList>,
    #[account(mut)]
    pub owner: Signer<'info>,
}
