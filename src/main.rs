#![feature(proc_macro_hygiene, decl_macro)]
#![feature(toowned_clone_into)]

#[macro_use] extern crate rocket;
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
use async_trait::async_trait;

// #[derive(Copy)]
struct Pair {
    from_coin: String,
    to_coin: String,
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

struct ExchangeRate {
    last: Decimal,
    bid: Decimal,
    ask: Decimal,
    low: Decimal,
    high: Decimal,
    pair: Pair
}

impl Clone for ExchangeRate {
    fn clone(&self) -> ExchangeRate {
        ExchangeRate {
            last: Decimal::from_str(self.last.to_string().as_str()).unwrap(),
            bid: Decimal::from_str(self.bid.to_string().as_str()).unwrap(),
            ask: Decimal::from_str(self.ask.to_string().as_str()).unwrap(),
            low: Decimal::from_str(self.low.to_string().as_str()).unwrap(),
            high: Decimal::from_str(self.high.to_string().as_str()).unwrap(),
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

type Pairs = Vec<Pair>;

// struct ExchangeAdapter {
//     pairs: Vec<Pair>
// }

struct ExchangeAdapter<'a> {
    pairs: Vec<Pair>,
    pair_map: HashMap<String, Pair>,
    market_api: &'a str
}


#[async_trait]
trait BaseExchangeAdapter<'a> {
    // const MARKET_API: &'a str;
    fn build_uri(&self, uri: &str, endpoint: &str) -> String;
    async fn json_get(&self, uri: &str, endpoint: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_pairs(&mut self) -> Result<Pairs, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_rate(&mut self, from_coin: &str, to_coin: &str) -> Result<ExchangeRate, Box<dyn std::error::Error + Send + Sync>>;
    async fn has_pair(&mut self, from_coin: &str, to_coin: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    fn new() -> Self;
}

// struct BaseExchangeAdapter<'a> {
//     pairs: Vec<Pair>,
//     MARKET_API: &'a str
// }



mod adapter_core {
    use std::str;
    use std::collections::HashMap;

    pub fn build_uri(market_api: &str, uri: &str, endpoint: &str) -> String {
        // let mut market_api: String = market_api.parse().unwrap();
        // let mut uri: String = uri.parse().unwrap();
        let uri = uri.trim_start_matches("/");
        let market_api = market_api.trim_end_matches("/");
        let mut endpoint: String = endpoint.parse().unwrap();
        if endpoint != "" && endpoint != "/" {
            endpoint = format!("/{ep}", ep=endpoint.trim_start_matches("/"));
        }
        return format!(
            "{market_api}/{uri}{endpoint}", 
            market_api=market_api.trim_end_matches("/"),
            uri=uri.trim_start_matches("/"), 
            endpoint=endpoint
        );
    }
    pub async fn json_get(market_api: &str, uri: &str, endpoint: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>> {
        let api_url = build_uri(market_api, uri, endpoint);
        let resp = reqwest::get(&api_url).await?
                   .json::<HashMap<String, String>>().await?;
        Ok(resp)
    }
    pub async fn json_get_serde(market_api: &str, uri: &str, endpoint: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        let api_url = build_uri(market_api, uri, endpoint);
        println!("json_get_list - URL is: {}", api_url);

        println!("json_get_list - sending GET request");
        let resp: reqwest::Response = reqwest::get(&api_url).await?;
        println!("json_get_list - got response from server. decoding text.");
        let body: String = resp.text().await?;
        println!("json_get_list - decoded text. parsing it with Serde...");
        // println!("json_get_list - response is: {:#?}", resp.text()?);
        // println!("json_get_list - response is: {}", body);
        let v: serde_json::Value = serde_json::from_str(body.as_str())?;
        Ok(v)
    }

    use serde::de::Deserialize;

    pub async fn json_get_str(market_api: &str, uri: &str, endpoint: &str) 
    -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    {
        let api_url = build_uri(market_api, uri, endpoint);
        println!("json_get_list - URL is: {}", api_url);

        println!("json_get_list - sending GET request");
        let resp: reqwest::Response = reqwest::get(&api_url).await?;

        println!("json_get_list - got response from server. decoding text.");
        let rtext = resp.text().await?;
        Ok(rtext)
    }

    // pub async fn json_get_cast<'a, 'b, 'c, T>(market_api: &str, uri: &str, endpoint: &str) 
    //     -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    //     where
    //     T: Deserialize<'_>,
    // {
    //     let api_url = build_uri(market_api, uri, endpoint);
    //     println!("json_get_list - URL is: {}", api_url);

    //     println!("json_get_list - sending GET request");
    //     let _resp: Result<reqwest::Response, reqwest::Error> = reqwest::get(&api_url).await;
    //     let resp: reqwest::Response = _resp.unwrap_or_else(|error| {
    //         panic!(error)
    //     });

    //     println!("json_get_list - got response from server. decoding text.");
    //     let _rtext = resp.text();
    //     let _bodyx = _rtext.await;
    //     // let mut _bodyy: &'a str = _bodyx.unwrap_or(String::from("{\"error\": true}"));
    //     let body: &str = _bodyx.unwrap_or(String::from("{\"error\": true}")).as_str();
    //     let xres: T = serde_json::from_str(body).unwrap();
    //     return Ok(xres)
    // }

    // #[derive(Serialize, Deserialize)]
    // use serde::de::Deserialize;
    // pub async fn json_get_cast<'a, T>(market_api: &str, uri: &str, endpoint: &str) 
    //     -> serde_json::Result<T>
    // where
    //     T: Deserialize<'a>,
    // {
    //     let api_url = build_uri(market_api, uri, endpoint);
    //     println!("json_get_list - URL is: {}", api_url);

    //     println!("json_get_list - sending GET request");
    //     let _resp: Result<reqwest::Response, reqwest::Error> = reqwest::get(&api_url).await;
    //     let resp: reqwest::Response = _resp.unwrap_or_else(|error| {
    //         panic!(error)
    //     });

    //     println!("json_get_list - got response from server. decoding text.");
    //     let _rtext = resp.text();
    //     let _bodyx = _rtext.await;
    //     // let mut _bodyy: &'a str = _bodyx.unwrap_or(String::from("{\"error\": true}"));
    //     let _bodyy: String = _bodyx.unwrap_or(String::from("{\"error\": true}"));
    //     let mut _bodyz: String = String::new();
    //     _bodyy.as_str().clone_into(&mut _bodyz);
    //     let body: &'a str = _bodyz.as_str();

    //     // let _bodyx: &'a String = &_body.unwrap_or(String::from("{\"error\": true}"));
    //     // static bodyx: &'a String = _bodyx;
    //     // static body: str = bodyx.as_str();
    //     // {
    //     //     Ok(data) => data,
    //     //     Err(error) => match error.kind() {
    //     //         reqwest::Error
    //     //     }
    //     // };
    //     // let tbody: = 'a body;
    //     // let mut tbody = body.as_mut_str();
    //     // let tbody: 'a = body.as_str();
    //     println!("json_get_list - decoded text. parsing it with Serde...");
    //     // println!("json_get_list - response is: {:#?}", resp.text()?);
    //     // println!("json_get_list - response is: {}", body);
    //     // let xres: serde_json::Result<T> = serde_json::from_str(&_body.unwrap_or(String::from("{\"error\": true}")) as &'a str);
    //     // let xres: serde_json::Result<T> = serde_json::from_str(body).unwrap()?;
    //     let xres: T = serde_json::from_str(body).unwrap();
    //     return Ok(xres);
    //     // return xres;
    //     // Ok(result)
    // }
}

// #[async_trait]
// impl<'a> BaseExchangeAdapter<'a> {
//     const MARKET_API: &'static str = "";

//     fn build_uri(&self, uri: &str, endpoint: &str) -> String {
//         return adapter_core::build_uri(self.MARKET_API, uri, endpoint);
//     }
//     async fn json_get(&self, uri: &str, endpoint: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>> {
//         Ok(adapter_core::json_get(self.MARKET_API, uri, endpoint).await?)
//     }
//     async fn get_pairs(&self) -> Result<Pairs, Box<dyn std::error::Error + Send + Sync>> {
//         Ok(Vec::new())
//     }
// }

use serde::{Deserialize, Serialize};



mod Bittrex {
    use crate::{ExchangeAdapter, BaseExchangeAdapter, Pair, Pairs, ExchangeRate};
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
        associatedTermsOfService: Vec<String>,
        baseCurrencySymbol: String,
        createdAt: String,
        minTradeSize: String,
        precision: i16,
        prohibitedIn: Vec<String>,
        quoteCurrencySymbol: String,
        status: String,
        symbol: String,
        tags: Vec<String>,
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

    pub struct BittrexAdapter<'a> {
        pairs: Vec<Pair>,
        pair_map: HashMap<String, Pair>,
        market_api: &'a str
    }

    pub async fn get_bittrex_pairs() -> Result<Vec<BittrexPair>, Box<dyn std::error::Error + Send + Sync>> {
        let raw_res: String = adapter_core::json_get_str(BITTREX_API, "/", "").await?;
        let res: Vec<BittrexPair> = serde_json::from_str(raw_res.as_str()).unwrap();
        Ok(res)
    }

    #[async_trait]
    impl<'a> BaseExchangeAdapter<'a> for BittrexAdapter<'a> {
        // const MARKET_API: &'a str = BITTREX_API;

        fn build_uri(&self, uri: &str, endpoint: &str) -> String {
            return adapter_core::build_uri(self.market_api, uri, endpoint)
        }
        async fn json_get(&self, uri: &str, endpoint: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(adapter_core::json_get(self.market_api, uri, endpoint).await?)
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
            BittrexAdapter {
                market_api: BITTREX_API,
                pair_map: HashMap::new(),
                pairs: vec![]
            }
        }
        
    }

    pub fn new() -> BittrexAdapter<'static> {
        BittrexAdapter::new()
    }

}

#[tokio::main]
async fn main() {
    // let _res = Bittrex::get_bittrex_pairs("https://api.bittrex.com/v3/markets", "/", "").await;
    let mut btx: Bittrex::BittrexAdapter = Bittrex::new();
    let pairs: Pairs = btx.get_pairs().await.unwrap();
    // let _res = Bittrex::get_bittrex_pairs().await;
    // let res: Vec<Bittrex::BittrexPair> = _res.ok().unwrap();
    println!("Pair 1 is: {:#?}", pairs[1]);
    println!("BTC-USDT is: {:#?}", btx.get_rate("BTC", "USDT").await.unwrap());
    println!("LTC-BTC is: {:#?}", btx.get_rate("LTC", "BTC").await.unwrap());
    println!("DOGE-BTC is: {:#?}", btx.get_rate("DOGE", "BTC").await.unwrap());
}
