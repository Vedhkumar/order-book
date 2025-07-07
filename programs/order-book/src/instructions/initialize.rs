use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Market, OrderBook};

#[derive(Accounts)]
pub struct Initialize<'info> {
    // signer and the authority of the market
    #[account(mut)]
    pub authority: Signer<'info>,

    // Market account to store the market information
    #[account(
        init,
        payer = authority,
        space = 8 + Market::INIT_SPACE,
        seeds = [b"market", base_mint.key().as_ref(), quote_mint.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    // Order book account to store the bids and asks
    #[account(
        init,
        payer = authority,
        space = 8 + Market::INIT_SPACE,
        seeds = [b"order_book", market.key().as_ref()],
        bump
    )]
    pub order_book: Account<'info, OrderBook>,

    // Mint account for the base token (e.g., SOL)
    #[account(
        mint::token_program = token_program
    )]
    pub base_mint: InterfaceAccount<'info, Mint>,

    // Mint account for the quote token (e.g., USDC)
    #[account(
        mint::token_program = token_program
    )]
    pub quote_mint: InterfaceAccount<'info, Mint>,

    // These are not the ATA but the token accounts pdas
    #[account(
        init,
        payer = authority,
        token::mint = base_mint,
        token::authority = base_vault,
        seeds = [b"base_vault", market.key().as_ref()],
        bump
    )]
    pub base_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        token::mint = quote_mint,
        token::authority = quote_vault,
        seeds = [b"quote_vault", market.key().as_ref()],
        bump
    )]
    pub quote_vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init_market(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.market.set_inner(Market {
            authority: self.authority.key(),
            base_mint: self.base_mint.key(),
            quote_mint: self.quote_mint.key(),
            base_vault: self.base_vault.key(),
            quote_vault: self.quote_vault.key(),
            order_id: 0, // Start with 0, will be incremented on each order
            quote_vault_bump: bumps.quote_vault,
            base_vault_bump: bumps.base_vault,
            market_bump: bumps.market,
            order_book_bump: bumps.order_book,
        });
        Ok(())
    }
    pub fn init_order_book(&mut self) -> Result<()> {
        self.order_book.set_inner(OrderBook {
            market: self.market.key(),
            bids: Vec::new(),
            asks: Vec::new(),
        });
        Ok(())
    }
}
