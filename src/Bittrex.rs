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
use rust_decimal::Decimal;
use async_trait::async_trait;

pub const BITTREX_API: &str = "https://api.bittrex.com/v3/markets";

#[derive(Serialize, Deserialize)]
pub struct BittrexPair {
    pub associatedTermsOfService: Vec<String>,
    pub baseCurrencySymbol: String,
    pub createdAt: String,
    pub minTradeSize: String,
    pub precision: i16,
    pub prohibitedIn: Vec<String>,
    pub quoteCurrencySymbol: String,
    pub status: String,
    pub symbol: String,
    pub tags: Vec<String>,
}

impl fmt::Debug for BittrexPair {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BittrexPair")
            .field("baseCurrencySymbol", &self.baseCurrencySymbol)
            .field("quoteCurrencySymbol", &self.quoteCurrencySymbol)
            .field("symbol", &self.symbol)
            .field("precision", &self.precision)
            .field("minTradeSize", &self.minTradeSize)
            .finish()
    }
}


// pub struct BittrexAdapter<'a> {
//     // parent: BaseExchangeAdapter<'a>,
//     pairs: Vec<Pair>,
//     MARKET_API: &'a str
// }

// #[async_trait]
// trait BittrexAdapter<'a> {
//     const MARKET_API: &'a str;
//     fn build_uri(&self, uri: &str, endpoint: &str) -> String;
//     async fn json_get(&self, uri: &str, endpoint: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>>;
//     async fn get_pairs(&self) -> Result<Pairs, Box<dyn std::error::Error + Send + Sync>>;
// }

pub struct BittrexAdapter {
    pub pairs: Vec<Pair>,
    pub pair_map: HashMap<String, Pair>,
    pub market_api: String
}

pub async fn get_bittrex_pairs() -> Result<Vec<BittrexPair>, Box<dyn std::error::Error + Send + Sync>> {
    let raw_res: String = adapter_core::json_get_str(BITTREX_API, "/", "").await?;
    let res: Vec<BittrexPair> = serde_json::from_str(raw_res.as_str()).unwrap();
    Ok(res)
}

#[async_trait]
impl <'a> BaseExchangeAdapter<'a> for BittrexAdapter {
    // const MARKET_API: &'a str = BITTREX_API;

    fn build_uri(&self, uri: &str, endpoint: &str) -> String {
        return adapter_core::build_uri(self.market_api.as_str(), uri, endpoint)
    }
    async fn json_get(&self, uri: &str, endpoint: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(adapter_core::json_get(self.market_api.as_str(), uri, endpoint).await?)
    }
    async fn get_pairs(&mut self) -> Result<Pairs, Box<dyn std::error::Error + Send + Sync>> {
        let pairlist: Vec<BittrexPair> = get_bittrex_pairs().await?;
        // let mut new_pairs: Vec<Pair> = vec![];
        let pairmap = &mut self.pair_map;
        let selfpairs = &mut self.pairs;
        selfpairs.clear();
        // &mut self.pair_map;
        for p in pairlist {
            let np = Pair {
                from_coin: p.baseCurrencySymbol.clone(),
                to_coin: p.quoteCurrencySymbol.clone(),
            };
            selfpairs.push(np.clone());
            let str_pair: String = format!("{}_{}", p.baseCurrencySymbol, p.quoteCurrencySymbol);
            
            pairmap.insert(str_pair, np.clone());
        }
        Ok(selfpairs.to_vec())
    }

    async fn has_pair(&mut self, from_coin: &str, to_coin: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if self.pairs.len() == 0 || self.pair_map.len() == 0 {
            self.get_pairs().await?;
        }
        Ok(self.pair_map.contains_key(&format!("{}_{}", from_coin, to_coin)))
    }

    async fn get_rate(&mut self, from_coin: &str, to_coin: &str) -> Result<ExchangeRate, Box<dyn std::error::Error + Send + Sync>> {
        let from_coin = from_coin.to_ascii_uppercase();
        let to_coin = to_coin.to_ascii_uppercase();
        let ticker_data: HashMap<String, String> = self.json_get(
            format!("/{}-{}/ticker", from_coin, to_coin).as_str(), ""
        ).await?;
        Ok(ExchangeRate {
            last: Decimal::from_str(ticker_data["lastTradeRate"].as_str()).unwrap(),
            bid: Decimal::from_str(ticker_data["bidRate"].as_str()).unwrap(),
            ask: Decimal::from_str(ticker_data["askRate"].as_str()).unwrap(),
            pair: Pair {
                from_coin: from_coin, to_coin: to_coin
            },
            high: Decimal::new(0, 8),
            low: Decimal::new(0, 8),
        })
    }

    fn new() -> Self {
        // let mut adapter: &'a mut BittrexAdapter = &mut BittrexAdapter {
        //     market_api: String::from(BITTREX_API),
        //     pair_map: HashMap::new(),
        //     pairs: vec![]
        // };
        // &mut 'a adapter
        // let mut adapter: &'a mut BittrexAdapter = &mut BittrexAdapter {
        //     market_api: String::from(BITTREX_API),
        //     pair_map: HashMap::new(),
        //     pairs: vec![]
        // };
        let adapter: BittrexAdapter = BittrexAdapter {
            market_api: String::from(BITTREX_API),
            pair_map: HashMap::new(),
            pairs: vec![]
        };
        adapter
    }
    
}

pub fn new() -> BittrexAdapter {
    unsafe {
        BittrexAdapter::new()
    }
}

// pub fn new<'a>() -> &'a BittrexAdapter {
//     unsafe {
//         &BittrexAdapter::new()
//     }
// }