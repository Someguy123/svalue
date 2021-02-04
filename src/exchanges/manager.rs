// use crate::exchanges::{BittrexAdapter};
use crate::exchanges;
use crate::exchange::{StdExAdapter, AdapterCombo, Pair, ExchangeRateMap, PairNotFound};
use std::collections::HashMap;
use serde::de::StdError;
use crate::adapter_core::BoxErrGlobal;
use log::{info, trace, warn};
use std::ops::Deref;


// enum AnyAdapter {
//     Bittrex(exchanges::bittrex::BittrexAdapter),
//     Huobi(exchanges::huobi::HuobiAdapter),
//     Kraken(exchanges::kraken::KrakenAdapter<'static>),
// }

// pub type AdapterBox = Box<dyn StdExAdapter<'static>>;
pub type AdapterBox = Box<dyn StdExAdapter<'static>>;

// pub struct ExchangeManager<X: StdExAdapter<'static>> {
pub struct ExchangeManager {
    // exchanges: HashMap<&'a str, Box<dyn BaseExchangeAdapter<'a>>>,
    // exchanges: HashMap<&'a str, Box<dyn AdapterCombo<'a> + Sized>>,
    exchanges: HashMap<String, &'static AdapterBox>,
    pair_map: HashMap<String, Vec<String>>,
}
// impl<'a, T: BaseExchangeAdapter<'a, T>> ExchangeManager<'a, T> {
// impl<'a, X: StdExAdapter<'static>> ExchangeManager<X> {
impl<'a> ExchangeManager {
    pub fn add_exchange_pair(&mut self, ex: &'static mut Box<dyn StdExAdapter<'static> + 'static>,
                             pair: String) -> bool {
        // let mut psym: &'e String = &pair.symbol();
        let mut exchs: &mut HashMap<String, &'static Box<dyn StdExAdapter<'static> + 'static>> =
            &mut self.exchanges;
        let excode: String = String::from(ex.code().clone());
        let statex: &'static Box<dyn StdExAdapter<'static> + 'static> = ex.deref();
        if !exchs.contains_key(&excode.clone()) {
            exchs.insert(excode.clone(), statex);
        }
        // xpairs = map of pair symbols (BTC_LTC) to vectors of exchange codes ['bittrex', 'huobi']
        // xpairs['BTC_LTC'] = ['bittrex', 'huobi', 'kraken']
        let mut xpairs: &mut HashMap<String, Vec<String>> = &mut self.pair_map;
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
    pub async fn register(&mut self, ex: &'static mut Box<dyn StdExAdapter<'static> + 'static>)
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
    pub fn get_exchange(&mut self, code: String) -> &Box<dyn StdExAdapter<'static>> {
        let mut exchs: &mut HashMap<String, &'static Box<dyn StdExAdapter<'static> + 'static>> = &mut self.exchanges;
        let ex = exchs.get(&code).unwrap();
        return ex;
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
                let mut ex = &mut self.get_exchange(ex).deref().deref();
                // let mut ex: dyn StdExAdapter<'static> = Box::into_raw(_ex);
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

    // pub fn new() -> ExchangeManager<X> {
    pub fn new() -> ExchangeManager {
        // let exs: HashMap<&'a str, Box<dyn AdapterCombo<'a>>> = HashMap::new();
        // let exs: HashMap<&'a str, Box<dyn AdapterCombo<'a>>> = HashMap::new();
        let exs: HashMap<String, &'static Box<dyn StdExAdapter<'static> + 'static>> = HashMap::new();
        ExchangeManager {
            exchanges: exs,
            pair_map: HashMap::new()
        }
    }
}
