#![feature(proc_macro_hygiene, decl_macro)]
#![feature(toowned_clone_into)]


extern crate futures;
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
use std::io::{Error, ErrorKind};
use std::{error};
use std::fmt::Display;
use async_trait::async_trait;

// #[derive(Copy)]
pub struct Pair {
    pub from_coin: String,
    pub to_coin: String,
}

impl Pair {
    pub fn from_str(from_coin: &str, to_coin: &str) -> Self {
        Pair { from_coin: String::from(from_coin), to_coin: String::from(to_coin) }
    }
    pub fn symbol(&self) -> String {
        format!("{}_{}", self.from_coin, self.to_coin)
    }
}
impl fmt::Debug for Pair {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Pair")
            .field("from_coin", &self.from_coin)
            .field("to_coin", &self.to_coin)
            .finish()
    }
}

// impl Copy for Pair { }

impl Clone for Pair {
    fn clone(&self) -> Pair {
        Pair { to_coin: self.to_coin.clone(), from_coin: self.from_coin.clone() }
    }
}

pub struct ExchangeRate {
    pub last: Decimal,
    pub bid: Decimal,
    pub ask: Decimal,
    pub low: Decimal,
    pub high: Decimal,
    pub volume: Decimal,
    pub open: Decimal,
    pub close: Decimal,
    pub pair: Pair
}

impl Clone for ExchangeRate {
    fn clone(&self) -> ExchangeRate {
        ExchangeRate {
            last: Decimal::from_str(self.last.to_string().as_str()).unwrap(),
            bid: Decimal::from_str(self.bid.to_string().as_str()).unwrap(),
            ask: Decimal::from_str(self.ask.to_string().as_str()).unwrap(),
            low: Decimal::from_str(self.low.to_string().as_str()).unwrap(),
            high: Decimal::from_str(self.high.to_string().as_str()).unwrap(),
            volume: Decimal::from_str(self.volume.to_string().as_str()).unwrap(),
            open: Decimal::from_str(self.open.to_string().as_str()).unwrap(),
            close: Decimal::from_str(self.close.to_string().as_str()).unwrap(),
            pair: self.pair.clone()
        }
    }
}

impl fmt::Debug for ExchangeRate {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ExchangeRate")
           .field("pair", &self.pair)
           .field("last", &self.last)
           .field("bid", &self.bid)
           .field("ask", &self.ask)
           .field("high", &self.high)
           .field("low", &self.low)
           .finish()
    }
}

pub type Pairs = Vec<Pair>;

// struct ExchangeAdapter {
//     pairs: Vec<Pair>
// }

pub struct ExchangeAdapter {
    pub pairs: Vec<Pair>,
    pub pair_map: HashMap<String, Pair>,
    pub market_api: String
}


#[async_trait]
pub trait BaseExchangeAdapter<'a> {
    // const MARKET_API: &'a str;
    fn name(&self) -> &'a str;
    fn code(&self) -> &'a str;
    fn build_uri(&self, uri: &str, endpoint: &str) -> String;
    async fn json_get(&self, uri: &str, endpoint: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_pairs(&mut self) -> Result<Pairs, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_rate(&mut self, from_coin: &str, to_coin: &str) -> Result<ExchangeRate, Box<dyn std::error::Error + Send + Sync>>;
    async fn has_pair(&mut self, from_coin: &str, to_coin: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    // fn new() -> &'a mut Self;
    fn new() -> Self;

}


#[derive(Debug)]
pub struct PairNotFound {
    v: String,
}

impl PairNotFound {
    pub fn new(pair: &str) -> PairNotFound {
        PairNotFound {
            v: format!("The requested pair '{}' does not exist!", pair).to_string()
        }
    }

    pub fn change_message(&mut self, new_message: &str) {
        self.v = new_message.to_string();
    }
}

impl std::error::Error for PairNotFound {}

impl Display for PairNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PairNotFound: {}", &self.v)
    }
}
