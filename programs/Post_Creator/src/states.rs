use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserPost {
    pub description: String, //4 + 2048
    pub url: String,         //4 + 2048
    pub authority: Pubkey,   //32
    pub post_id: u8,         //8
    pub like_count : u32, //8
    pub comments: Vec<Comment>,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Comment {
    pub author: Pubkey,
    pub content: String,
    pub timestamp: i64,
}