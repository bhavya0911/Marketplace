use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked, CloseAccount, close_account}};

use crate::state::*;

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    taker: Signer<'info>,
    #[account(mut)]
    maker: SystemAccount<'info>,
    maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    taker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token:: authority = listing,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds  = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
    )]
    listing: Account<'info, Listing>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    treasury: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    rewards_mint: InterfaceAccount<'info, Mint>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn send_sol(&self) -> Result<()> {
        let program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };
        let ctx = CpiContext::new(program, accounts);

        let amount = self.listing.price.checked_sub(self.marketplace.fee as u64).unwrap();

        transfer(ctx, amount)
    }

    pub fn send_nft(&mut self) -> Result<()> {
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let program = self.token_program.to_account_info();
        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_mint.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);

        transfer_checked(ctx, 1, self.maker_mint.decimals)
    }

    pub fn close_mint_vault(&mut self) -> Result<()> {
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let program = self.token_program.to_account_info();
        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.treasury.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);

        close_account(ctx)
    }
}