use crate::exchange::{Pair, Pairs, ExchangeRate, PairNotFound, AdapterMeta, AdapterRate, AdapterLow, AdapterPairs, AdapterCombo, AdapterFull, StdExAdapter};
use crate::adapter_core;
use crate::adapter_core::BoxErr;
use crate::adapter_core::BoxErrGlobal;
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
use std::future::Future;
// use std::f64::
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use async_trait::async_trait;

use std::panic;
use std::collections::hash_map::RandomState;
use std::error::Error;
use std::alloc::Global;

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

impl KrakenTickerTick {
    fn close(&self) -> String {
        self.last.get(0).unwrap().to_string()
    }
}

impl Clone for KrakenTickerTick {
    fn clone(&self) -> KrakenTickerTick {
        // { ask, bid, last, open, high, low, volume }
        KrakenTickerTick {
            ask: self.ask.clone(),
            bid: self.bid.clone(),
            last: self.last.clone(),
            open: self.open.clone(),
            high: self.high.clone(),
            low: self.low.clone(),
            volume: self.volume.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenTicker {
    error: Vec<String>,
    result: HashMap<String, KrakenTickerTick>
}

pub struct KrakenAdapter<'a> {
    provides: Vec<String>,
    pairs: Vec<String>,
    obj_pairs: Pairs,
    symbol_map: HashMap<&'a str, &'a str>,
    symbol_map_expected: HashMap<&'a str, Vec<&'a str>>,
    symbol_map_keys: Vec<&'a str>,
    known_bases: Vec<&'a str>,
    known_pairs: HashMap<&'a str, &'a str>,
    market_api: &'a str,
}

impl<'a> Clone for KrakenAdapter<'a> {
    fn clone(&self) -> KrakenAdapter<'a> {
        KrakenAdapter {
            provides: self.provides.clone(),
            pairs: self.pairs.clone(),
            obj_pairs: self.obj_pairs.clone(),
            symbol_map: self.symbol_map.clone(),
            symbol_map_expected: self.symbol_map_expected.clone(),
            symbol_map_keys: self.symbol_map_keys.clone(),
            known_bases: self.known_bases.clone(),
            known_pairs: self.known_pairs.clone(),
            market_api: self.market_api
        }
    }
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


impl <'a>KrakenAdapter<'a> {
    pub async fn _load_pairs(&mut self) -> Result<Vec<String>, BoxErr> {
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
    pub async fn load_pairs(&mut self, force: bool) -> Result<Vec<String>, BoxErr> {
        self._load_pairs().await?;
        let selfpairs = self.pairs.clone();
        let selfprov = &mut self.provides;
        if !selfprov.is_empty() && !force {
            return Ok(selfprov.clone());
        }
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

    pub async fn get_ticker(&self, pair: &str) -> Result<KrakenTicker, BoxErr> {
        let url = format!("{}{}", KRAKEN_TICKER, pair);
        let resp: reqwest::Response = reqwest::get(&String::from(url)).await.unwrap();
        let body: String = resp.text().await.unwrap();
        // let v: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
        // let selfpairs = &mut self.pairs;
        let z: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
        let za = z["error"].as_array();
        if let Some(x) = &za {
            if !x.is_empty() {
                return Err(Box::new(PairNotFound::new(pair)))
            }
        }
        let v: KrakenTicker = serde_json::from_str(body.as_str()).unwrap();
        // let kres = v.result;
        Ok(v)
    }

    pub async fn get_ticker_main(&mut self, from_coin: &str, to_coin: &str)
            -> Result<KrakenTickerTick, BoxErr> {
        let from_coin = String::from(from_coin).to_ascii_uppercase();
        let to_coin = String::from(to_coin).to_ascii_uppercase();
        let mut symx = format!("{from_coin}_{to_coin}", from_coin=from_coin, to_coin=to_coin);
        if self.known_pairs.contains_key(&symx.as_str()) {
            let xpair = self.known_pairs[symx.as_str()];
            println!("Found pair {} in known_pairs", xpair);
            let tres = self.get_ticker(xpair).await?;
            return Ok(tres.result[xpair].clone())
        }
        let mut from_coins = vec![from_coin.as_str()];
        let mut to_coins = vec![to_coin.as_str()];
        if self.symbol_map_expected.contains_key(&from_coin.as_str()) {
            from_coins = self.symbol_map_expected[from_coin.as_str()].clone();
        }
        if self.symbol_map_expected.contains_key(&to_coin.as_str()) {
            to_coins = self.symbol_map_expected[to_coin.as_str()].clone();
        }

        for fc in &from_coins {
            for tc in &to_coins {
                let xfc = &fc.to_string();
                let xtc = &tc.to_string();
                let mut fsym = format!("{}{}", xfc, xtc);
                let mut symk = format!("{}_{}", xfc, xtc);
                let fsym = fsym.as_mut_str();
                // let symk = symk.as_mut_str();
                // let bs = &self;
                // const tkcall: dyn Future<Result<KrakenTicker, BoxErr>> = self.get_ticker(fsym);
                let kcore = KrakenAdapter::new();
                // let mut tkcall = KrakenCore::new().get_ticker(fsym.clone());
                let result = panic::catch_unwind(|| {
                    // let res: Result<KrakenTicker, BoxErr> = kcore.get_ticker(fsym).await;
                    return kcore.get_ticker(fsym);
                });
                // let res: Result<KrakenTicker, BoxErr> = self.get_ticker(fsym).await;
                if result.is_err() {
                    println!("Kraken pair '{}' threw an error. Trying a different pair...", fsym);
                    continue;
                }
                // let result = panic::catch_unwind(|| async {

                let res = result.unwrap().await;
                if res.is_err() {
                    println!("Kraken pair '{}' threw an error. Trying a different pair...", fsym);
                    continue;
                }
                // });

                println!("Kraken pair '{}' was successful! Saving to known pairs for next time", fsym);
                
                // let xsymk = symk.as_str();
                // let xfsym = fsym.as_str();
                // let knwpairs = &mut self.known_pairs;
                // &knwpairs.insert(fsym, symk);
                let unwrapped = res.unwrap();
                return Ok(unwrapped.result[fsym].clone());
            }
        }
        Err(Box::new(PairNotFound::new(format!("{} / {}", from_coin, to_coin).as_str())))
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

        KrakenAdapter {
            provides: vec![],
            pairs: vec![],
            obj_pairs: vec![],
            market_api: KRAKEN_BASE,
            symbol_map,
            symbol_map_keys,
            symbol_map_expected,
            known_bases,
            known_pairs
        }
    }
}

impl<'a> AdapterMeta<'a> for KrakenAdapter<'_> {
    fn name(&self) -> &'a str { "Kraken" }

    fn code(&self) -> &'a str { "kraken" }
}

#[async_trait]
impl AdapterRate for KrakenAdapter<'_> {
    async fn get_rate(&mut self, from_coin: &str, to_coin: &str)
            -> Result<ExchangeRate, BoxErrGlobal>  {
        let ktick = self.get_ticker_main(
            from_coin.to_ascii_uppercase().as_str(), to_coin.to_ascii_uppercase().as_str()
        ).await?;
        Ok(
            ExchangeRate {
                last: Decimal::from_str(ktick.last[0].as_str()).unwrap(),
                bid: Decimal::from_str(ktick.bid[0].as_str()).unwrap(),
                ask: Decimal::from_str(ktick.ask[0].as_str()).unwrap(),
                low: Decimal::from_str(ktick.low[0].as_str()).unwrap(),
                high: Decimal::from_str(ktick.high[0].as_str()).unwrap(),
                volume: Decimal::from_str(ktick.volume[0].as_str()).unwrap(),
                open: Decimal::from_str(ktick.open.as_str()).unwrap(),
                close: Decimal::from_str(ktick.close().as_str()).unwrap(),
                pair: Pair::from_str(from_coin, to_coin)
            }
        )
    }
}

#[async_trait]
impl AdapterLow for KrakenAdapter<'_> {
    fn build_uri(&self, uri: &str, endpoint: &str) -> String  {
        return adapter_core::build_uri(self.market_api, uri, endpoint)
    }

    async fn json_get(&self, uri: &str, endpoint: &str)
            -> Result<HashMap<String, String>, BoxErr>  {
        Ok(adapter_core::json_get(self.market_api, uri, endpoint).await?)
    }
}

#[async_trait]
impl AdapterPairs for KrakenAdapter<'_> {
    async fn get_pairs(&mut self) -> Result<Pairs, BoxErrGlobal>  {
        if self.obj_pairs.is_empty() {
            let pairs = self.load_pairs(false).await?;
            let opairs: &mut Vec<Pair> = &mut self.obj_pairs;
            for p in pairs {
                opairs.insert(opairs.len(), Pair::from_symbol(p.as_str()));
            }
        }
        Ok(self.obj_pairs.clone())
    }
    async fn has_pair(&mut self, from_coin: &str, to_coin: &str)
            -> Result<bool, BoxErrGlobal>  {
        Ok(self.pairs.contains(&Pair::from_str(from_coin, to_coin).symbol()))
    }
}

impl<'a> AdapterCombo<'a> for KrakenAdapter<'_> {}

impl<'a> AdapterFull<'a> for KrakenAdapter<'_> {}

impl<'a>StdExAdapter<'a> for KrakenAdapter<'_> {}


/*
#[async_trait]
impl<'a, T: 'a + BaseExchangeAdapter<'a, T>> BaseExchangeAdapter<'a, T> for KrakenAdapter<'a> {

    fn name(&self) -> &'a str { "Kraken" }

    fn code(&self) -> &'a str { "kraken" }

    fn build_uri(&self, uri: &str, endpoint: &str) -> String {
        return adapter_core::build_uri(self.market_api, uri, endpoint)
    }

    async fn json_get(&self, uri: &str, endpoint: &str) -> Result<HashMap<String, String>, BoxErr> {
        Ok(adapter_core::json_get(self.market_api, uri, endpoint).await?)
    }

    async fn get_pairs(&mut self) -> Result<Pairs, BoxErrGlobal> {
        if self.obj_pairs.is_empty() {
            let pairs = self.load_pairs(false).await?;
            let opairs: &mut Vec<Pair> = &mut self.obj_pairs;
            for p in pairs {
                opairs.insert( opairs.len(), Pair::from_symbol(p.as_str()));
            }
        }
        Ok(self.obj_pairs.clone())
    }

    async fn get_rate(&mut self, from_coin: &str, to_coin: &str) -> Result<ExchangeRate, BoxErrGlobal> {
        let ktick = self.get_ticker_main(
            from_coin.to_ascii_uppercase().as_str(), to_coin.to_ascii_uppercase().as_str()
        ).await?;
        Ok(
            ExchangeRate {
                last: Decimal::from_str(ktick.last[0].as_str()).unwrap(),
                bid: Decimal::from_str(ktick.bid[0].as_str()).unwrap(),
                ask: Decimal::from_str(ktick.ask[0].as_str()).unwrap(),
                low: Decimal::from_str(ktick.low[0].as_str()).unwrap(),
                high: Decimal::from_str(ktick.high[0].as_str()).unwrap(),
                volume: Decimal::from_str(ktick.volume[0].as_str()).unwrap(),
                open: Decimal::from_str(ktick.open.as_str()).unwrap(),
                close: Decimal::from_str(ktick.close().as_str()).unwrap(),
                pair: Pair::from_str(from_coin, to_coin)
            }
        )
    }

    async fn has_pair(&mut self, from_coin: &str, to_coin: &str) -> Result<bool, BoxErrGlobal> {
        Ok(self.pairs.contains(&Pair::from_str(from_coin, to_coin).symbol()))
    }

    fn new() -> Self {
        KrakenAdapter::new()
    }
}

*/


pub fn new<'a>() -> KrakenAdapter<'a> {
    unsafe {
        KrakenAdapter::new()
    }
}

// pub async fn get_huobi_pairs() -> Result<Vec<HuobiPair>, BoxErr> {

// }
