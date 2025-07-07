pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6NnwaBDZ8M3iSaNnetsaQ9CrdpSnStSaGhKb8Z7nm4fK");

#[program]
pub mod order_book {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.init_market(&ctx.bumps)?;
        ctx.accounts.init_order_book()
    }
    pub fn place_bid(ctx: Context<PlaceBid>, price: u64, quantity: u64) -> Result<()> {
        ctx.accounts.increse_order_id()?;
        ctx.accounts.create_order(price, quantity, &ctx.bumps)?;
        //q check the price and quantity
        ctx.accounts.transfer_token(price * quantity)
    }
    pub fn place_ask(ctx: Context<PlaceAsk>, price: u64, quantity: u64) -> Result<()> {
        ctx.accounts.increse_order_id()?;
        ctx.accounts.create_order(price, quantity, &ctx.bumps)?;
        ctx.accounts.transfer_token(price * quantity)
    }
    pub fn cancle_order(ctx: Context<CancelOrder>) -> Result<()> {
        ctx.accounts.transfer_token()
    }

    pub fn match_order(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
