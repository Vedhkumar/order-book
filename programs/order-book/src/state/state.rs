use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub authority: Pubkey,
    pub base_mint: Pubkey,  // SOL
    pub quote_mint: Pubkey, // USDC
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub order_id: u64, // Unique ID for orders
    pub quote_vault_bump: u8,
    pub base_vault_bump: u8,
    pub market_bump: u8,
    pub order_book_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct OrderBook {
    pub market: Pubkey,
    #[max_len(100)]
    pub bids: Vec<Order>,
    #[max_len(100)]
    pub asks: Vec<Order>,
}

#[account]
#[derive(InitSpace)]
pub struct Order {
    pub id: u64,
    pub trader: Pubkey,
    pub order_type: OrderType,
    pub price: u64,    // price at which the order is placed
    pub quantity: u64, // quantity of the asset to buy/sell
    pub status: OrderStatus,
    pub timestamp: i64,
    pub order_bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum OrderType {
    Bid,
    Ask,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum OrderStatus {
    Open,
    Filled,
    Cancelled,
}
