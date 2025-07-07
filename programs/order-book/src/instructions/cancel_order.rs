use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Market, Order, OrderBook, OrderStatus, OrderType};

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

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
        mut,
        has_one = trader,
        seeds = [b"order", market.key().as_ref(), trader.key().as_ref()],
        bump = order.order_bump,
        // so the all the lamports are returned to the trader but the account is not closed immediately it takes time to close by the garbage collector so we can access the data in the implementation
        close = trader,
    )]
    pub order: Account<'info, Order>,

    #[account(
        mint::token_program = token_program
    )]
    pub base_mint: InterfaceAccount<'info, Mint>, // wrapped sol

    #[account(
        mint::token_program = token_program
    )]
    pub quote_mint: InterfaceAccount<'info, Mint>,

    #[account(
        token::mint = base_mint,
        token::authority = base_vault,
        seeds = [b"base_vault", market.key().as_ref()],
        bump = market.base_vault_bump,
    )]
    pub base_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        token::mint = quote_mint,
        token::authority = quote_vault,
        seeds = [b"quote_vault", market.key().as_ref()],
        bump = market.quote_vault_bump,
    )]
    pub quote_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = trader,
        associated_token::mint = base_mint,
        associated_token::authority = trader,
    )]
    pub trader_base_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = trader,
        associated_token::mint = quote_mint,
        associated_token::authority = trader,
    )]
    pub trader_quote_account: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CancelOrder<'info> {
    pub fn transfer_token(&self) -> Result<()> {
        let market = &self.market.key();
        let quote_vault_signer_seeds: &[&[&[u8]]] = &[&[
            b"quote_vault",
            market.as_ref(),
            &[self.market.quote_vault_bump],
        ]];

        let base_vault_signer_seeds: &[&[&[u8]]] = &[&[
            b"base_vault",
            market.as_ref(),
            &[self.market.base_vault_bump],
        ]];

        let (from, to, mint, signer_seeds) = match self.order.order_type {
            OrderType::Ask => (
                self.quote_vault.to_account_info(),
                self.trader_quote_account.to_account_info(),
                self.quote_mint.to_account_info(),
                quote_vault_signer_seeds,
            ),
            OrderType::Bid => (
                self.base_vault.to_account_info(),
                self.trader_base_account.to_account_info(),
                self.base_mint.to_account_info(),
                base_vault_signer_seeds,
            ),
        };

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from,
                    to,
                    authority: self.trader.to_account_info(),
                    mint,
                },
                &signer_seeds,
            ),
            self.order.quantity * self.order.price,
            self.quote_mint.decimals,
        )
    }
}
