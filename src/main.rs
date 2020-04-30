#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
mod alpaca;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _key = fs::read_to_string("< path to api key >").expect("Something went wrong");

    let _secret =
        fs::read_to_string("< path to api secret >").expect("Something went wrong");

    let key = _key.trim();
    let secret = _secret.trim();

    let symbol = "TSLA";

    let timeframe = "1D";
    let symbols = vec!["TSLA".to_string(), "GOOG".to_string()];

    // match alpaca::info::view_account_info(key, secret) {
    // match alpaca::info::view_asset_info(symbol, key, secret) {
    match alpaca::info::get_bars(timeframe, symbols, key, secret) {
        Err(err) => Err(Box::new(err)),
        _ => Ok(()),
    }
}
