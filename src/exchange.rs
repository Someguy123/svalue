#![feature(proc_macro_hygiene, decl_macro)]
#![feature(toowned_clone_into)]
#![feature(type_alias_impl_trait)]

extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate serde_json;
extern crate async_trait;
extern crate serde;
// extern crate anyhow;

use rust_decimal::Decimal;
use std::str;
use std::str::FromStr;
use std::fmt;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::{error};
use std::convert::From;
use std::fmt::Display;
use async_trait::async_trait;
use crate::adapter_core::{BoxErrGlobal, BoxErr, BoxErrStd};
use log::{info, trace, warn};
use std::ops::{Deref, DerefMut};
use serde::de::StdError;
use std::borrow::{Borrow, BorrowMut};
// use std::raw::TraitObject;

// #[derive(Copy)]
// #[derive(Debug)]
pub struct Pair {
    pub from_coin: String,
    pub to_coin: String,
}

impl Pair {
    pub fn from_str(from_coin: &str, to_coin: &str) -> Self {
        Pair {
            from_coin: String::from(from_coin).to_ascii_uppercase(),
            to_coin: String::from(to_coin).to_ascii_uppercase()
        }
    }
    pub fn from_symbol(symbol: &str) -> Self {
        let symbol: String = symbol.to_ascii_uppercase();

        let xp: Vec<&str> = symbol.split("_").into_iter().collect();
        // Pair { from_coin: xp[0], to_coin: xp[1].clone() }
        return Pair::from_str(xp[0], xp[1])
    }
    pub fn symbol(&self) -> String {
        format!("{}_{}", self.from_coin, self.to_coin).to_ascii_uppercase()
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

impl fmt::Display for Pair {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.symbol())
    }
}

// impl Copy for Pair { }

impl Clone for Pair {
    fn clone(&self) -> Pair {
        Pair { to_coin: self.to_coin.clone(), from_coin: self.from_coin.clone() }
    }
}

// #[derive(Debug)]
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

impl ExchangeRate {
    //noinspection ALL
    fn from_coin(&self) -> String { self.pair.clone().from_coin }
    fn to_coin(&self) -> String { self.pair.clone().to_coin }
    fn symbol(&self) -> String { self.pair.clone().symbol() }
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

impl fmt::Display for ExchangeRate {
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


// #[async_trait]
// pub trait BaseExchangeAdapter<'a, T> where Self: Sized {
//     // const MARKET_API: &'a str;
//     fn name(&self) -> &'a str;
//     fn code(&self) -> &'a str;
//     fn build_uri(&self, uri: &str, endpoint: &str) -> String;
//     async fn json_get(&self, uri: &str, endpoint: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>>;
//     async fn get_pairs(&mut self) -> Result<Pairs, Box<dyn std::error::Error + Send + Sync>>;
//     async fn get_rate(&mut self, from_coin: &str, to_coin: &str) -> Result<ExchangeRate, Box<dyn std::error::Error + Send + Sync>>;
//     async fn has_pair(&mut self, from_coin: &str, to_coin: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
//     // fn new() -> &'a mut Self;
//     fn new() -> Self;
//
// }

pub trait AdapterMeta<'a> {
    fn name(&self) -> &'a str;
    fn code(&self) -> &'a str;
}

#[async_trait]
pub trait AdapterLow {
    fn build_uri(&self, uri: &str, endpoint: &str) -> String;
    async fn json_get(&self, uri: &str, endpoint: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait AdapterRate {
    async fn get_rate(&mut self, from_coin: &str, to_coin: &str)
        -> Result<ExchangeRate, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait AdapterPairs {
    async fn get_pairs(&mut self) -> Result<Pairs, Box<dyn std::error::Error + Send + Sync>>;
    async fn has_pair(&mut self, from_coin: &str, to_coin: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait AdapterCombo<'a>: AdapterMeta<'a> + AdapterRate + AdapterPairs {}

#[async_trait]
pub trait AdapterFull<'a>: AdapterMeta<'a> + AdapterRate + AdapterPairs + AdapterLow {}

pub trait StdExAdapter<'a> where Self: AdapterCombo<'a> {}

// impl <'a>StdExAdapter<'a> {
//
// }
// impl Clone for StdExAdapter {
//     fn clone(&self) -> StdExAdapter {
//         StdExAdapter {
//             pairs: self.
//         }
//     }
// }

// pub type CoreAdapter<'a, 'l> = &'l (impl AdapterMeta<'a, 'l> + AdapterRate);
// pub type MostAdapter<'a, 'l> = &'l (impl AdapterMeta<'a, 'l> + AdapterRate + AdapterPairs);
// pub type FullAdapter<'a, 'l> = &'l (impl AdapterMeta<'a, 'l> + AdapterRate + AdapterPairs +
//                                         AdapterLow);

// const ComboAdapter: TraitObject = AdapterMeta;

// impl<'a> dyn BaseExchangeAdapter<'a> {}


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

pub type ExchangeRateMap<'a> = HashMap<&'a str, ExchangeRate>;

/*
pub struct ExchangeManager<X: StdExAdapter<'static>> {
    // exchanges: HashMap<&'a str, Box<dyn BaseExchangeAdapter<'a>>>,
    // exchanges: HashMap<&'a str, Box<dyn AdapterCombo<'a> + Sized>>,
    exchanges: HashMap<String, X>,
    pair_map: HashMap<String, Vec<String>>,
}

// enum AnyAdapter {
//     exchange::
// }

// impl<'a, T: BaseExchangeAdapter<'a, T>> ExchangeManager<'a, T> {
impl<'a, X: StdExAdapter<'static>> ExchangeManager<X> {
    pub fn add_exchange_pair(&mut self, ex: &mut X, pair: String) -> bool {
        // let mut psym: &'e String = &pair.symbol();
        let mut exchs = &mut self.exchanges;
        let excode: String = String::from(ex.code().clone());
        if !exchs.contains_key(&excode.clone()) {
            exchs.insert(excode.clone(), ex.clone());
        }
        // xpairs = map of pair symbols (BTC_LTC) to vectors of exchange codes ['bittrex', 'huobi']
        // xpairs['BTC_LTC'] = ['bittrex', 'huobi', 'kraken']
        let mut xpairs = &mut self.pair_map;
        // let all_pairs = ex.get_pairs().await?.clone();
        // for p in all_pairs
        // If the pair_map hash doesn't contain the pair symbol, then we add it,
        // initialising the vector to hold name codes of exchanges that support the pair.
        let epair: String = String::from(pair.clone());
        if !xpairs.contains_key(&epair.clone()) {
            xpairs.insert(epair.clone(), vec![]);
        }
        if let Some(pkey) = &mut xpairs.get_mut(&epair.clone()) {
            if !pkey.contains(&excode.clone()) {
                // xpairs.get_mut()
                // let mut mkey: &mut Vec<&str> = xpairs[p.symbol().as_str()].as_mut();
                pkey.insert(pkey.len(), excode.clone());
            }
        }
        true
    }
    // pub async fn register(&mut self, exchange: impl AdapterCombo<'a> + 'static)
    pub async fn register(&mut self, ex: &mut X)
            -> Result<bool, Box<dyn StdError + Sync + std::marker::Send>> {
        // let mut ex = exchange;
        // let name = ex.code();
        if self.exchanges.contains_key(ex.code()) {
            return Ok(false);
        }
        // let mut exchs = &mut self.exchanges;
        // // exchs[name] = Box::new(ex);
        // exchs.insert(name, ex.clone());
        // let mut xpairs = &mut self.pair_map;
        // let gpairs = ex.get_pairs();
        // let apairs = gpairs.await;
        // let xpairs = &mut apairs.unwrap();
        let xpairs = ex.get_pairs().await?;
        for p in xpairs {
            // let zp = p;
            // let mut _zs = zp.symbol();
            // let zs: &str = _zs.as_str();
            // let symz: &'static str = zs.as_str();
            // let mut p: &'static Pair = &zp.clone();
            // let osym: &'static String = &zp.symbol();
            // let mut xsym: &'e mut str = &mut osym.as_str();

            self.add_exchange_pair(ex, p.symbol());
            // // If the pair_map hash doesn't contain the pair symbol, then we add it,
            // // initialising the vector to hold name codes of exchanges that support the pair.
            // let _psym: &'e String = &p.symbol();
            // let psym: &'e str = _psym.as_str();
            // if !xpairs.contains_key(psym) {
            //     xpairs.insert(psym, vec![]);
            // }
            // // Next, if the pair_map[pair_symbol] vector doesn't contain the codename of this
            // // exchange we're registering - then we add the codename of the exchange.
            // if let Some(pkey) = &mut xpairs.get_mut(p.symbol().as_str()) {
            //     if !pkey.contains(&name) {
            //         // xpairs.get_mut()
            //         // let mut mkey: &mut Vec<&str> = xpairs[p.symbol().as_str()].as_mut();
            //         pkey.insert(pkey.len(), name);
            //     }
            // }
            // // let mut pkey: &mut &Vec<&'e str> = &mut xpairs.get(p.symbol().as_str()).unwrap();
            // // if !pkey.contains(&name) {
            // //     pkey.insert(pkey.len(),name);
            // // }
        }
        Ok(true)
    }

    pub fn has_pair(&self, from_coin: &str, to_coin: &str) -> bool {
        let pair = Pair::from_str(from_coin, to_coin);
        let xpairs = &self.pair_map.clone();
        xpairs.contains_key(&pair.symbol())
    }

    // fn get_exchange(&mut self, code: &str) -> impl BaseExchangeAdapter<'a, T> {
    // fn get_exchange(&mut self, code: &str) -> &&Box<impl AdapterCombo<'a>> {
    pub fn get_exchange(&mut self, code: String) -> X {
        let mut exchs: &mut HashMap<String, X> = &mut self.exchanges;
        let ex: &X = exchs.get(&code).clone().unwrap();
        return ex.clone();
    }

    pub async fn get_rate(&mut self, from_coin: &str, to_coin: &str)
            -> Result<ExchangeRateMap<'a>, BoxErrGlobal> {
        let pair = &Pair::from_str(from_coin, to_coin);
        // let mut _symbol: String = pair.symbol();
        // let mut symbol: &'a str = _symbol.as_mut_str();
        // let mut exchs = &mut self.exchanges.clone();
        let xpairs = &self.pair_map.clone();
        if xpairs.contains_key(pair.symbol().as_str()) {
            let pair_exs: Vec<String> = xpairs[&pair.symbol()].clone();
            let mut ratemap: ExchangeRateMap = HashMap::new();
            for ex in pair_exs {
                // let mut ex: &mut Box<dyn impl BaseExchangeAdapter<'a>> = &mut exchs[ex];
                let mut ex: X = self.get_exchange(ex).clone();
                // let mut ex: &mut &impl AdapterCombo<'a> = &mut _ex.deref().deref().deref();
                let excode = ex.code();
                let _rateres = ex.get_rate(pair.from_coin.as_str(), pair.to_coin.as_str());
                let rateres = _rateres.await;
                if rateres.is_err() {
                    warn!("Failed to get exchange rate from exchange '{}' (code '{}')",
                          ex.name(), excode);
                    warn!("Error from {} is: {}", excode, rateres.unwrap_err().to_string());
                    continue;
                }
                let xres = rateres.unwrap();
                // ratemap[excode] = xres;
                ratemap.insert(excode, xres);
            }
            if !ratemap.is_empty() {
                return Ok(ratemap);
            }
        }
        Err(
            Box::new(PairNotFound::new(pair.symbol().as_str()))
        )
    }

    pub fn new() -> ExchangeManager<X> {
        // let exs: HashMap<&'a str, Box<dyn AdapterCombo<'a>>> = HashMap::new();
        // let exs: HashMap<&'a str, Box<dyn AdapterCombo<'a>>> = HashMap::new();
        let exs: HashMap<String, X> = HashMap::new();
        ExchangeManager {
            exchanges: exs,
            pair_map: HashMap::new()
        }
    }
}
*/
