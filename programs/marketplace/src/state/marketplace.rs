use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u16,
    pub bump: u8,
    pub rewards_bump: u8,
    pub treasure_bump: u8,
    pub name: String,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = ANCHOR_DISC + PUBKEY_L * 1 + U16_L * 1 + U8_L * 3 + STRING_L;
}