use crate::exchange::{BaseExchangeAdapter, Pair, Pairs, ExchangeRate};
use crate::adapter_core;
use serde::{Deserialize, Serialize};
extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate serde_json;
extern crate async_trait;
extern crate serde;

use std::str;
use std::str::FromStr;
use std::fmt;
use std::collections::HashMap;
// use std::f64::
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use async_trait::async_trait;


const KRAKEN_BASE: &str = "https://api.kraken.com/0/public";
const KRAKEN_PAIRS: &str = "https://api.kraken.com/0/public/AssetPairs?info=fees";
const KRAKEN_TICKER: &str = "https://api.kraken.com/0/public/Ticker?pair=";



#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenPairs {
    error: Vec<String>,
    result: HashMap<String, HashMap<String, serde_json::Value>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenTickerTick {
    #[serde(rename="a")]
    ask: Vec<String>,
    #[serde(rename="b")]
    bid: Vec<String>,
    #[serde(rename="c")]
    last: Vec<String>,
    #[serde(rename="o")]
    open: String,
    #[serde(rename="h")]
    high: Vec<String>,
    #[serde(rename="l")]
    low: Vec<String>,
    #[serde(rename="v")]
    volume: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenTicker {
    error: Vec<String>,
    result: HashMap<String, KrakenTickerTick>
}

pub struct KrakenCore<'a> {
    provides: Vec<String>,
    pairs: Vec<String>,
    symbol_map: HashMap<&'a str, &'a str>,
    symbol_map_expected: HashMap<&'a str, Vec<&'a str>>,
    symbol_map_keys: Vec<&'a str>,
    known_bases: Vec<&'a str>,
    known_pairs: HashMap<&'a str, &'a str>
}

pub fn find_base(pair: &str, known_bases: Vec<&str>) -> Option<String> {
    let pair = pair.to_ascii_uppercase();
    for b in known_bases {
        if pair.ends_with(b) {
            return Some(String::from(b));
        }
    }
    return None
}

// #[async_trait]
impl <'a>KrakenCore<'a> {
    pub async fn _load_pairs(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let resp: reqwest::Response = reqwest::get(KRAKEN_PAIRS).await.unwrap();
        let body: String = resp.text().await.unwrap();
        // let v: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
        let selfpairs = &mut self.pairs;
        let v: KrakenPairs = serde_json::from_str(body.as_str()).unwrap();
        let kres = v.result;

        for k in kres.keys() {
            if selfpairs.contains(&k) {
                continue;
            }
            selfpairs.push(k.to_string());
        }

        Ok(selfpairs.clone())
    }
    // fn find_base(&self, pair: &'a str) -> Option<&'a str> {
    //     let pair = pair.to_ascii_uppercase();
    //     for b in self.known_bases {
    //         if pair.ends_with(b) {
    //             return Some(b);
    //         }
    //     }
    //     return None
    // }
    pub async fn load_pairs(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        self._load_pairs().await?;
        let selfpairs = self.pairs.clone();
        let mut selfprov = &mut self.provides;
        selfprov.clear();
        for pair in selfpairs {
            let b = find_base(pair.as_str().clone(), self.known_bases.clone()).clone();

            if let Some(x) = &b {
                
                let from_coinz:  Vec<&str> = pair.split(x).collect();
                let mut from_coin = from_coinz[0];
                let mut to_coin: String = x.parse()?;
                if self.symbol_map_keys.contains(&to_coin.as_str()) {
                    to_coin = String::from(self.symbol_map[to_coin.as_str()]);
                }
                if self.symbol_map_keys.contains(&from_coin) {
                    from_coin = self.symbol_map[from_coin];
                }
                selfprov.push(format!("{}_{}", from_coin.to_ascii_uppercase(), to_coin.to_ascii_uppercase()));
            } else {
                continue;
            }

        }
        Ok(selfprov.clone())
    }

    pub async fn get_ticker(&self, pair: &str) -> Result<KrakenTicker, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}", KRAKEN_TICKER, pair);
        let resp: reqwest::Response = reqwest::get(&String::from(url)).await.unwrap();
        let body: String = resp.text().await.unwrap();
        // let v: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
        // let selfpairs = &mut self.pairs;
        let v: KrakenTicker = serde_json::from_str(body.as_str()).unwrap();
        // let kres = v.result;
        Ok(v)
    }

    pub fn new() -> Self {
        let symbol_map: HashMap<&'a str, &'a str> = [
            ("XXDG", "DOGE"),
            ("XDG", "DOGE"),
            ("XXBT", "BTC"),
            ("XBT", "BTC"),
            ("XLTC", "LTC"),
            ("ZUSD", "USD"),
            ("ZEUR", "EUR"),
            ("ZGBP", "GBP"),
            ("ZJPY", "JPY"),
        ].iter().cloned().collect();
        let symbol_map_expected: HashMap<&'a str, Vec<&'a str>> = [
            ("DOGE", vec!["XDG", "XXDG"]),
            ("BTC", vec!["XXBT", "XBT"]),
            ("XBT", vec!["XXBT", "XBT"]),
            ("LTC", vec!["XLTC", "LTC"]),
            ("ETH", vec!["XETH", "ETH"]),
            ("ETC", vec!["XETC", "ETC"]),
            ("XRP", vec!["XXRP", "XRP"]),
            ("XMR", vec!["XXMR", "XMR"]),
            ("USD", vec!["ZUSD", "USD", "USDT", "USDC"]),
            ("EUR", vec!["ZEUR", "EUR"]),
            ("GBP", vec!["ZGBP", "GBP"]),
            ("CAD", vec!["ZCAD", "CAD"]),
            ("JPY", vec!["ZJPY", "JPY"])
        ].iter().cloned().collect();
        let symbol_map_keys: Vec<&'a str> = symbol_map.keys().cloned().collect();
        
        let known_bases: Vec<&'a str> = symbol_map_keys.clone().into_iter().chain(vec![
            "BTC", "ETH", "USDT", "USDC",
            "USD", "GBP", "EUR", "JPY",
            "CAD", "CHF", "DAI"
        ].into_iter()).collect();
        
        
        let known_pairs: HashMap<&'a str, &'a str> = [
            ("BTC_USD", "XXBTZUSD"),
            ("LTC_USD", "XLTCZUSD"),
            ("ETH_USD", "XETHZUSD"),
            ("BTC_EUR", "XXBTZEUR"),
            ("LTC_EUR", "XLTCZEUR"),
            ("ETH_EUR", "XETHZEUR"),
            ("BTC_GBP", "XXBTZGBP"),
            ("LTC_GBP", "XLTCZGBP"),
            ("ETH_GBP", "XETHZGBP"),
            ("EOS_USD", "EOSUSD"),
            ("EOS_BTC", "EOSXBT"),
            ("LTC_BTC", "XLTCXXBT"),
            ("ETH_BTC", "XETHXXBT"),
            ("USD_EUR", "USDTEUR"),
            ("USD_GBP", "USDTGBP"),
            ("USD_CAD", "USDTCAD"),
        ].iter().cloned().collect();

        KrakenCore {
            provides: vec![],
            pairs: vec![],
            symbol_map: symbol_map,
            symbol_map_keys: symbol_map_keys,
            symbol_map_expected: symbol_map_expected,
            known_bases: known_bases,
            known_pairs: known_pairs
        }
    }
}

// pub async fn get_huobi_pairs() -> Result<Vec<HuobiPair>, Box<dyn std::error::Error + Send + Sync>> {

// }