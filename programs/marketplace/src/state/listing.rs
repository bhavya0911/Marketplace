use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Listing {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8,
}

impl Space for Listing {
    const INIT_SPACE: usize = ANCHOR_DISC + PUBKEY_L * 2 + U64_L * 1 + U8_L * 1;
}