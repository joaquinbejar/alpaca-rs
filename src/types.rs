use serde::{Deserialize, Serialize};

/// Account information from Alpaca API
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub account_number: String,
    pub status: String,
    pub currency: String,
    pub buying_power: String,
    pub cash: String,
    pub portfolio_value: String,
}

/// Asset information
#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub class: String,
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub status: String,
    pub tradable: bool,
}

/// Order information
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub client_order_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub submitted_at: String,
    pub asset_id: String,
    pub symbol: String,
    pub asset_class: String,
    pub qty: String,
    pub filled_qty: String,
    pub order_type: String,
    pub side: String,
    pub time_in_force: String,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub status: String,
}
