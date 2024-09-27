use anchor_lang::prelude::*;

declare_id!("EjawMqs2qzV2tnHhu1nqFVGpXEYLbeCi8ToKzbWm5kSd");

mod constants;
mod state;
mod contexts;
use contexts::*;
mod error;

#[program]
pub mod week4_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u16, name: String) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.send_nft()?;
        ctx.accounts.close_mint_vault()
    }
}
