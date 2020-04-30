use serde::{Serialize, Deserialize};
use crate::alpaca::user;

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    symbol: String,
    qty: u64,
    side: String,
    r#type: String,
    time_in_force: String
}

impl Order {

    pub fn new(symbol: &str, qty: u64, side: &str, r#type: &str, time_in_force: &str) -> Order {
        Order {
            symbol: String::from(symbol),
            qty: qty,
            side: String::from(side),
            r#type: String::from(r#type),
            time_in_force: String::from(time_in_force),
        }
    }
}

pub fn make_day_market_buy(user: &user::User, symbol: &str, quantity: u64) -> Result<(), reqwest::Error> {
    let order = Order::new(symbol, quantity, "buy", "market", "day");
    let url = "https://paper-api.alpaca.markets/v2/orders";
    let response = reqwest::blocking::Client::new()
        .post(url)
        .json(&order)
        .header("APCA-API-KEY-ID", user.get_key())
        .header("APCA-API-SECRET-KEY", user.get_secret())
        .send()?;

    println!("Status: {}", response.status());
    println!("Headers: {:?}", response.headers());

    Ok(())
}
