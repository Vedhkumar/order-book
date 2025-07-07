use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Market, Order, OrderBook, OrderStatus, OrderType};

#[derive(Accounts)]
pub struct MatchOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        has_one = base_mint,
        has_one = quote_mint,
        seeds = [b"market", base_mint.key().as_ref(), quote_mint.key().as_ref()],
        bump = market.market_bump,
    )]
    pub market: Account<'info, Market>,

    #[account(
        has_one = market,
        mut,
        seeds = [b"order_book", market.key().as_ref()],
        bump = market.order_book_bump,
    )]
    pub order_book: Account<'info, OrderBook>,

    #[account(
        seeds = [b"order", market.key().as_ref(), trader.key().as_ref()],
        bump = order_bid.order_bump,
    )]
    pub order_bid: Account<'info, Order>,

    #[account(
        seeds = [b"order", market.key().as_ref(), trader.key().as_ref()],
        bump = order_ask.order_bump,
    )]
    pub order_ask: Account<'info, Order>,

    #[account(
        mint::token_program = token_program
    )]
    pub base_mint: InterfaceAccount<'info, Mint>, // wrapped sol

    #[account(
        mint::token_program = token_program
    )]
    pub quote_mint: InterfaceAccount<'info, Mint>,

    #[account(
        token::mint = quote_mint,
        token::authority = base_vault,
        seeds = [b"base_vault", market.key().as_ref()],
        bump = market.base_vault_bump,
    )]
    pub base_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = base_mint,
        associated_token::authority = trader,
    )]
    pub trader_base_account: InterfaceAccount<'info, TokenAccount>, // user SOL token account to sell SOL and buy usdc

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> MatchOrder<'info> {
    pub fn create_order(&mut self, price: u64, quantity: u64, bumps: &PlaceAskBumps) -> Result<()> {
        self.order.set_inner(Order {
            id: self.market.order_id,
            trader: self.trader.key(),
            order_type: OrderType::Ask,
            price,
            quantity,
            status: OrderStatus::Open,
            timestamp: Clock::get()?.unix_timestamp,
            order_bump: bumps.order,
        });
        Ok(())
    }

    pub fn transfer_token(&self, amount: u64) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.trader_base_account.to_account_info(),
                    to: self.base_vault.to_account_info(),
                    authority: self.trader.to_account_info(),
                    mint: self.base_mint.to_account_info(),
                },
            ),
            amount,
            self.quote_mint.decimals,
        )
    }
}
