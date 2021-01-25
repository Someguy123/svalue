use svalue::Bittrex;
use svalue::Bittrex::BittrexAdapter;
// use svalue::Bittrex::BaseExchangeAdapter;
use svalue::exchange::BaseExchangeAdapter;
use svalue::exchange::{Pair, Pairs};
// use svalue::Bittrex::BittrexAdapter;

// use crate::Bittrex;


#[tokio::main]
async fn main() {
    // let _res = Bittrex::get_bittrex_pairs("https://api.bittrex.com/v3/markets", "/", "").await;
    unsafe {
        let mut btx: BittrexAdapter = BittrexAdapter::new();
        let pairs: Pairs = btx.get_pairs().await.unwrap();
        // let _res = Bittrex::get_bittrex_pairs().await;
        // let res: Vec<Bittrex::BittrexPair> = _res.ok().unwrap();
        println!("Pair 1 is: {:#?}", pairs[1]);
        println!("BTC-USDT is: {:#?}", btx.get_rate("BTC", "USDT").await.unwrap());
        println!("LTC-BTC is: {:#?}", btx.get_rate("LTC", "BTC").await.unwrap());
        println!("DOGE-BTC is: {:#?}", btx.get_rate("DOGE", "BTC").await.unwrap());
    }
}
