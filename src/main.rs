use svalue::Bittrex;
use svalue::Bittrex::BittrexAdapter;
use svalue::Huobi;
use svalue::Huobi::HuobiAdapter;
// use svalue::Bittrex::BaseExchangeAdapter;
use svalue::exchange::BaseExchangeAdapter;
use svalue::exchange::{Pair, Pairs};
// use svalue::huobi_test::get_ticker;
// use svalue::Bittrex::BittrexAdapter;

// use crate::Bittrex;

fn test_pairs() -> Pairs {
    let mut tpairs: Pairs = Vec::new();
    tpairs.push(Pair::from_str("doge", "btc"));
    tpairs.push(Pair::from_str("doge", "btc"));
    tpairs.push(Pair::from_str("btc", "usdt"));
    tpairs.push(Pair::from_str("ltc", "btc"));
    tpairs.push(Pair::from_str("ltc", "usdt"));
    tpairs
}

#[tokio::main]
async fn main() {
    // let _res = Bittrex::get_bittrex_pairs("https://api.bittrex.com/v3/markets", "/", "").await;
    let tpairs: Pairs = test_pairs();
    let mut hb = Huobi::new();
    let mut btx = Bittrex::new();
    let hpairs: Pairs = hb.get_pairs().await.unwrap();
    let bpairs: Pairs = btx.get_pairs().await.unwrap();
    println!("Huobi Pairs: {}", hpairs.len());
    println!("Bittrex Pairs: {}", bpairs.len());
    for p in tpairs {
        let hrate = hb.get_rate(p.from_coin.as_str(), p.to_coin.as_str()).await.unwrap();
        println!("{} exchange rate (huobi): {:#?}", p.symbol(), hrate);
        let brate = btx.get_rate(p.from_coin.as_str(), p.to_coin.as_str()).await.unwrap();
        println!("{} exchange rate (bittrex): {:#?}", p.symbol(), brate);
    }
    // let mut btx: BittrexAdapter = BittrexAdapter::new();
    // let pairs: Pairs = btx.get_pairs().await.unwrap();
    // let _res = Bittrex::get_bittrex_pairs().await;
    // let res: Vec<Bittrex::BittrexPair> = _res.ok().unwrap();
    // println!("Pair 1 is: {:#?}", pairs[1]);
    // println!("BTC-USDT is: {:#?}", btx.get_rate("BTC", "USDT").await.unwrap());
    // println!("LTC-BTC is: {:#?}", btx.get_rate("LTC", "BTC").await.unwrap());
    // println!("DOGE-BTC is: {:#?}", btx.get_rate("DOGE", "BTC").await.unwrap());
    // get_ticker("ltc", "usdt").await;
}
