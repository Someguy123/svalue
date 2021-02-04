#![feature(allocator_api)]
#[macro_use] extern crate rocket;
// #![feature(proc_macro_hygiene, decl_macro)]
// #![feature(toowned_clone_into)]

// #[macro_use] extern crate rocket;
// extern crate futures;
// extern crate tokio_core;
// extern crate tokio;
// extern crate serde_json;
// extern crate async_trait;
// extern crate serde;

// use rust_decimal::Decimal;
// use std::str;
// use std::str::FromStr;
// use std::fmt;
// use std::collections::HashMap;
// use async_trait::async_trait;
// use serde::{Deserialize, Serialize};


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
// use crate::adapter_core;
// use crate::exchange;
// use crate::Bittrex;
// use crate::exchanges;
pub mod adapter_core;
pub mod exchange;
pub mod exchanges;
// pub mod exchanges;
// pub mod huobi_test;
// pub mod Huobi;
// pub mod Kraken;
// pub mod main;

