use serde::Deserialize;
use std::collections::HashMap;

extern crate plotly;
use plotly::common::{Mode, Title, Font};
use plotly::{Plot, Scatter, Layout};
use plotly::layout::{Axis, Legend};

use crate::alpaca::user;

#[derive(Deserialize, Debug)]
struct AccountInfo {
    // These are a subset of the available fields in the endpoint's response
    // They do not have to be in the same order as in the response
    id: String,
    status: String,
    buying_power: String,
    portfolio_value: String,
}

pub fn view_account_info(user: &user::User) -> Result<(), reqwest::Error> {
    let url = "https://paper-api.alpaca.markets/v2/account";
    let response = reqwest::blocking::Client::new()
        .get(url)
        .header("APCA-API-KEY-ID", user.get_key())
        .header("APCA-API-SECRET-KEY", user.get_secret())
        .send()?;

    println!("Status: {}", response.status());
    println!("Headers: {:?}", response.headers());

    println!("{:#?}", response.json::<AccountInfo>()?);

    Ok(())
}

#[derive(Deserialize, Debug)]
struct AssetInfo {
    // These are a subset of the available fields in the endpoint's response
    // They do not have to be in the same order as in the response
    id: String,
    symbol: String,
    class: String,
    status: String,
    tradable: bool,
}

pub fn view_asset_info(user: &user::User, symbol: &str) -> Result<(), reqwest::Error> {
    let url = format!("https://paper-api.alpaca.markets/v2/assets/{}", symbol);
    let response = reqwest::blocking::Client::new()
        .get(&url)
        .header("APCA-API-KEY-ID", user.get_key())
        .header("APCA-API-SECRET-KEY", user.get_secret())
        .send()?;

    println!("Status: {}", response.status());
    println!("Headers: {:?}", response.headers());

    println!("{:#?}", response.json::<AssetInfo>()?);

    Ok(())
}

#[derive(Deserialize, Debug)]
struct Bar {
    // Beginnging of the bar, Unix epoch (seconds)
    t: u64,
    // Open
    o: f32,
    // High
    h: f32,
    // Low
    l: f32,
    // Close
    c: f32,
    // Volume
    v: u64,
}

pub fn get_bars(
    user: &user::User,
    timeframe: &str,
    symbols: Vec<String>
) -> Result<(), reqwest::Error> {
    println!("Getting Bars for {}", symbols.join(", "));
    let url = format!(
        "https://data.alpaca.markets/v1/bars/{}?symbols={}",
        timeframe,
        symbols.join(",")
    );

    let response = reqwest::blocking::Client::new()
        .get(&url)
        .header("APCA-API-KEY-ID", user.get_key())
        .header("APCA-API-SECRET-KEY", user.get_secret())
        .send()?;

    println!("Status: {}", response.status());
    println!("Headers: {:?}", response.headers());

    let bar_map = response.json::<HashMap<String, Vec<Bar>>>().unwrap();
    println!("{:?}", &bar_map);

    /*
    let symbol = bar_map.keys().next().unwrap();
    let close_prices: Vec<f32> = match bar_map.get(symbol) {
        Some(bars) => { bars.iter().map(|b| b.c).collect::<Vec<f32>>() },
        _ => vec![]
    };

    let timestamps: Vec<u64> = match bar_map.get(symbol) {
        Some(bars) => { bars.iter().map(|b| b.t).collect::<Vec<u64>>() },
        _ => vec![]
    };

    assert_eq!(timestamps.len(), close_prices.len());

    println!("Close Prices: {}\n\t{:?}", &symbol, &close_prices);

    let close_prices_trace = Scatter::new(timestamps, close_prices)
        .name(&symbol[..])
        .mode(Mode::LinesMarkers);
    */

    let layout = Layout::new()
        .title(Title::new("Closing Prices per timestamp"))
        .legend(Legend::new().font(Font::new().size(16)))
        .xaxis(Axis::new().title(Title::new("Timestamp")))
        .yaxis(Axis::new().title(Title::new("Closing Price")));

    let mut plot = Plot::new();
    plot.set_layout(layout);
    // plot.add_trace(close_prices_trace);
    // plot.show();

    for symbol in bar_map.keys() {
        let prices: Vec<f32> = match bar_map.get(symbol) {
            Some(bars) => bars.iter().map(|b| b.c).collect::<Vec<f32>>(),
            _ => vec![]
        };
        let timestamps: Vec<u64> = match bar_map.get(symbol) {
            Some(bars) => bars.iter().map(|b| b.t).collect::<Vec<u64>>(),
            _ => vec![]
        };
        assert_eq!(prices.len(), timestamps.len());
        let trace = Scatter::new(timestamps, prices)
            .name(&symbol[..])
            .mode(Mode::LinesMarkers);
        plot.add_trace(trace);
    }
    plot.show_png(1024, 680);

    Ok(())
}

