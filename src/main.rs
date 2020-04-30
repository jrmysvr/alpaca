#![allow(dead_code)]
#![allow(unused_variables)]

mod alpaca;

fn info_demo(user: &alpaca::user::User) -> Result<(), Box<dyn std::error::Error>> {
    let symbol = "TSLA";

    let timeframe = "1D";
    let symbols = vec!["TSLA".to_string(), "GOOG".to_string()];

    // match alpaca::info::view_account_info(user) {
    // match alpaca::info::view_asset_info(user, symbol) {
    match alpaca::info::get_bars(user, timeframe, symbols) {
        Err(err) => Err(Box::new(err)),
        _ => Ok(()),
    }

}

fn trade_demo(user: &alpaca::user::User) {
    let symbol = "TSLA";
    let quantity = 1;
    let side = "buy";
    let order_type = "market";
    let time_in_force = "day";
    let order = alpaca::trade::Order::new(symbol,
                                          quantity,
                                          side,
                                          order_type,
                                          time_in_force);
    println!("{:#?}", order);

    match alpaca::trade::make_day_market_buy(user, symbol, quantity) {
        Ok(_) => println!("Order made successfully"),
        Err(err) => println!("Something went wrong\n{}", err),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let key = std::env::var("APCA_API_KEY_ID")?;
    // let secret = std::env::var("APCA_SECRET_API_KEY")?;
    // let user = alpaca::user::User::new(key, secret);

    let user = alpaca::user::User::from_env()?;

    info_demo(&user)?;
    // trade_demo(&user);
    Ok(())
}
