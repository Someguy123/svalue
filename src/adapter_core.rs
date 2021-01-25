#![feature(proc_macro_hygiene, decl_macro)]
#![feature(toowned_clone_into)]

extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate serde_json;
extern crate async_trait;
extern crate serde;

// use rust_decimal::Decimal;
use std::str;
// use std::str::FromStr;
// use std::fmt;
use std::collections::HashMap;
// use async_trait::async_trait;
// use serde::{Deserialize, Serialize};


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

// use serde::de::Deserialize;

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