use crate::exchange::{Pair, Pairs, ExchangeRate, AdapterMeta, AdapterRate, AdapterPairs, AdapterLow, AdapterCombo, AdapterFull, StdExAdapter};
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
use crate::adapter_core::BoxErr;


const HUOBI_BASE: &str = "https://api.huobi.pro";
const HUOBI_PAIRS: &str = "https://api.huobi.pro/v1/common/symbols";
const HUOBI_TICKER: &str = "https://api.huobi.pro/market/detail/merged?symbol=";



#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize="snake_case", deserialize="kebab-case"))]
pub struct HuobiPair {
    base_currency: String,
    quote_currency: String,
    symbol: String,
    state: String,
    symbol_partition: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HuobiResult {
    status: String,
    data: Vec<HuobiPair>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HuobiTickerTick {
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

impl HuobiTickerTick {
    fn last_bid(&self) -> f64 {
        self.bid[0]
    }
    fn last_ask(&self) -> f64 {
        self.ask[0]
    }
    fn last(&self) -> f64 {
        (self.ask[0] + self.bid[0]) / 2.0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HuobiTicker {
    status: String,
    ch: String,
    ts: i64,
    tick: HuobiTickerTick
}

pub async fn get_ticker(from_coin: &str, to_coin: &str) -> Result<HuobiTicker, BoxErr>{
    let symbol = format!("{}{}", String::from(from_coin).to_ascii_lowercase(), String::from(to_coin).to_ascii_lowercase());
    let url = format!("{}{}", HUOBI_TICKER, symbol);
    let resp: reqwest::Response = reqwest::get(url.as_str()).await.unwrap();
    let body: String = resp.text().await.unwrap();
    let v: HuobiTicker = serde_json::from_str(body.as_str()).unwrap();
    Ok(v)
}

pub async fn get_huobi_pairs() -> Result<Vec<HuobiPair>, BoxErr> {
    let resp: reqwest::Response = reqwest::get(HUOBI_PAIRS).await.unwrap();
    let body: String = resp.text().await.unwrap();
    // let v: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
    let v: HuobiResult = serde_json::from_str(body.as_str()).unwrap();
    Ok(v.data)
    // println!("Data 0: {:#?}", v["data"][0]);
    // println!("Data 1: {:#?}", v["data"][1]);
}

pub struct HuobiAdapter {
    pub pairs: Vec<Pair>,
    pub pair_map: HashMap<String, Pair>,
    pub market_api: String
}

impl HuobiAdapter {
    pub fn new() -> Self {
        HuobiAdapter {
            market_api: String::from(HUOBI_BASE),
            pair_map: HashMap::new(),
            pairs: vec![]
        }
    }
}

impl Clone for HuobiAdapter {
    fn clone(&self) -> HuobiAdapter {
        HuobiAdapter {
            pairs: self.pairs.clone(),
            pair_map: self.pair_map.clone(),
            market_api: self.market_api.clone()
        }
    }
}

impl<'a> AdapterMeta<'a> for HuobiAdapter {
    fn name(&self) -> &'a str {
        "Huobi"
    }
    fn code(&self) -> &'a str {
        "huobi"
    }
}

#[async_trait]
impl AdapterPairs for HuobiAdapter {
    async fn get_pairs(&mut self) -> Result<Pairs, BoxErr>  {
        let pairlist: Vec<HuobiPair> = get_huobi_pairs().await?;
        // let mut new_pairs: Vec<Pair> = vec![];
        let pairmap = &mut self.pair_map;
        let selfpairs = &mut self.pairs;
        selfpairs.clear();
        // &mut self.pair_map;
        for p in pairlist {
            let np = Pair {
                from_coin: p.base_currency.clone(),
                to_coin: p.quote_currency.clone(),
            };
            selfpairs.push(np.clone());
            let str_pair: String = format!("{}_{}", p.base_currency, p.quote_currency);

            pairmap.insert(str_pair, np.clone());
        }
        Ok(selfpairs.to_vec())
    }
    async fn has_pair(&mut self, from_coin: &str, to_coin: &str)
            -> Result<bool, BoxErr>  {
        if self.pairs.len() == 0 || self.pair_map.len() == 0 {
            self.get_pairs().await?;
        }
        Ok(self.pair_map.contains_key(&format!("{}_{}", from_coin, to_coin)))
    }
}

#[async_trait]
impl AdapterRate for HuobiAdapter {
    async fn get_rate(&mut self, from_coin: &str, to_coin: &str)
            -> Result<ExchangeRate, BoxErr>  {
        let hres: HuobiTicker = get_ticker(from_coin, to_coin).await?;
        let ticker: HuobiTickerTick = hres.tick;
        Ok(ExchangeRate {
            last: Decimal::from_f64(ticker.last()).unwrap(),
            bid: Decimal::from_f64(ticker.last_bid()).unwrap(),
            ask: Decimal::from_f64(ticker.last_ask()).unwrap(),
            pair: Pair {
                from_coin: String::from(from_coin),
                to_coin: String::from(to_coin)
            },
            high: Decimal::from_f64(ticker.high).unwrap(),
            low: Decimal::from_f64(ticker.low).unwrap(),
            volume: Decimal::from_f64(ticker.vol).unwrap(),
            open: Decimal::from_f64(ticker.open).unwrap(),
            close: Decimal::from_f64(ticker.close).unwrap(),
        })
    }
}

#[async_trait]
impl AdapterLow for HuobiAdapter {
    fn build_uri(&self, uri: &str, endpoint: &str) -> String  {
        return adapter_core::build_uri(self.market_api.as_str(), uri, endpoint)
    }
    async fn json_get(&self, uri: &str, endpoint: &str)
            -> Result<HashMap<String, String>, BoxErr>  {
        Ok(adapter_core::json_get(self.market_api.as_str(), uri, endpoint).await?)
    }
}

impl<'a> AdapterCombo<'a> for HuobiAdapter {}

impl<'a> AdapterFull<'a> for HuobiAdapter {}

impl<'a>StdExAdapter<'a> for HuobiAdapter {}


/*
#[async_trait]
impl<'a, T: 'a + BaseExchangeAdapter<'a, T>> BaseExchangeAdapter<'a, T> for HuobiAdapter {
    // const MARKET_API: &'a str = BITTREX_API;
    fn name(&self) -> &'a str {
        "Huobi"
    }
    fn code(&self) -> &'a str {
        "huobi"
    }
    fn build_uri(&self, uri: &str, endpoint: &str) -> String {
        return adapter_core::build_uri(self.market_api.as_str(), uri, endpoint)
    }
    async fn json_get(&self, uri: &str, endpoint: &str) -> Result<HashMap<String, String>, BoxErr> {
        Ok(adapter_core::json_get(self.market_api.as_str(), uri, endpoint).await?)
    }
    async fn get_pairs(&mut self) -> Result<Pairs, BoxErr> {
        let pairlist: Vec<HuobiPair> = get_huobi_pairs().await?;
        // let mut new_pairs: Vec<Pair> = vec![];
        let pairmap = &mut self.pair_map;
        let selfpairs = &mut self.pairs;
        selfpairs.clear();
        // &mut self.pair_map;
        for p in pairlist {
            let np = Pair {
                from_coin: p.base_currency.clone(),
                to_coin: p.quote_currency.clone(),
            };
            selfpairs.push(np.clone());
            let str_pair: String = format!("{}_{}", p.base_currency, p.quote_currency);
            
            pairmap.insert(str_pair, np.clone());
        }
        Ok(selfpairs.to_vec())
    }

    async fn get_rate(&mut self, from_coin: &str, to_coin: &str) -> Result<ExchangeRate, BoxErr> {
        let hres: HuobiTicker = get_ticker(from_coin, to_coin).await?;
        let ticker: HuobiTickerTick = hres.tick;
        Ok(ExchangeRate {
            last: Decimal::from_f64(ticker.last()).unwrap(),
            bid: Decimal::from_f64(ticker.last_bid()).unwrap(),
            ask: Decimal::from_f64(ticker.last_ask()).unwrap(),
            pair: Pair {
                from_coin: String::from(from_coin), to_coin: String::from(to_coin)
            },
            high: Decimal::from_f64(ticker.high).unwrap(),
            low: Decimal::from_f64(ticker.low).unwrap(),
            volume: Decimal::from_f64(ticker.vol).unwrap(),
            open: Decimal::from_f64(ticker.open).unwrap(),
            close: Decimal::from_f64(ticker.close).unwrap(),
        })
    }

    async fn has_pair(&mut self, from_coin: &str, to_coin: &str) -> Result<bool, BoxErr> {
        if self.pairs.len() == 0 || self.pair_map.len() == 0 {
            self.get_pairs().await?;
        }
        Ok(self.pair_map.contains_key(&format!("{}_{}", from_coin, to_coin)))
    }

    fn new() -> Self {
        let adapter: HuobiAdapter = HuobiAdapter {
            market_api: String::from(HUOBI_BASE),
            pair_map: HashMap::new(),
            pairs: vec![]
        };
        adapter
    }
    
}
*/

pub fn new() -> HuobiAdapter {
    HuobiAdapter::new()
}
