#![feature(proc_macro_hygiene, decl_macro)]
#![feature(toowned_clone_into)]

// extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate serde_json;
extern crate async_trait;
extern crate serde;

use rust_decimal::Decimal;
use std::str;
use std::str::FromStr;
use std::fmt;
use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde_json::Value;

const HUOBI_PAIRS: &str = "https://api.huobi.pro/v1/common/symbols";
const HUOBI_TICKER: &str = "https://api.huobi.pro/market/detail/merged?symbol=";



#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize="snake_case", deserialize="kebab-case"))]
struct HuobiPair {
    base_currency: String,
    quote_currency: String,
    symbol: String,
    state: String,
    symbol_partition: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct HuobiResult {
    status: String,
    data: Vec<HuobiPair>
}

#[derive(Serialize, Deserialize, Debug)]
struct HuobiTickerTick {
    amount: f64,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    id: i64,
    count: i64,
    version: i64,
    vol: f64,
    bid: Vec<f64>,
    ask: Vec<f64>
}

#[derive(Serialize, Deserialize, Debug)]
struct HuobiTicker {
    status: String,
    ch: String,
    ts: i64,
    tick: HuobiTickerTick
}

pub async fn get_ticker(from_coin: &str, to_coin: &str) {
    let symbol = format!("{}{}", String::from(from_coin).to_ascii_lowercase(), String::from(to_coin).to_ascii_lowercase());
    let url = format!("{}{}", HUOBI_TICKER, symbol);
    println!("Huobi Ticker URL: {}", url);

    let resp: reqwest::Response = reqwest::get(url.as_str()).await.unwrap();
    let body: String = resp.text().await.unwrap();
    println!("Raw body: {}", body);

    let v: HuobiTicker = serde_json::from_str(body.as_str()).unwrap();
    println!("Ticker for {}: {:#?}", symbol, v);
    println!("Volume {}: {:#?}", symbol, v.tick.vol);
}

pub async fn get_syms() {
    let resp: reqwest::Response = reqwest::get(HUOBI_PAIRS).await.unwrap();
    let body: String = resp.text().await.unwrap();
    // let v: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
    let v: HuobiResult = serde_json::from_str(body.as_str()).unwrap();
    println!("Data 0: {:#?}", v.data[0]);
    println!("Data 1: {:#?}", v.data[1]);
    // println!("Data 0: {:#?}", v["data"][0]);
    // println!("Data 1: {:#?}", v["data"][1]);
}

