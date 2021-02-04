// use svalue::Bittrex;
// use svalue::Bittrex::BittrexAdapter;
// use svalue::Huobi;
// use svalue::Huobi::HuobiAdapter;
// use svalue::Kraken;
// use svalue::Kraken::{KrakenAdapter};
// use svalue::Bittrex::BaseExchangeAdapter;
use svalue::exchange::{Pair, Pairs, StdExAdapter, AdapterCombo};
use svalue::exchanges;
use svalue::exchanges::{ BittrexAdapter, KrakenAdapter, HuobiAdapter, ExchangeManager };
use log::{info, trace, warn};
use svalue::adapter_core::BoxErrGlobal;

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
async fn main() -> Result<(), std::io::Error> {
    // let _res = Bittrex::get_bittrex_pairs("https://api.bittrex.com/v3/markets", "/", "").await;
    let adapters: &'static mut Vec<Box<dyn AdapterCombo<'static>>> = &mut vec![
        Box::new(BittrexAdapter::new()),
        Box::new(KrakenAdapter::new()),
        Box::new(HuobiAdapter::new()),
    ];



    let tpairs: Pairs = test_pairs();
    // type xadapters = impl KrakenAdapter + HuobiAdapter + BittrexAdapter;
    // unsafe {
    let mut exm = ExchangeManager::new(adapters);
    // let newadapters = exm.exchanges.iter_mut().into_slice();

    // for mut a in newadapters.iter_mut() {
    //     // unsafe {
    //         // let _adp: *mut Option<dyn AdapterCombo> = Box::into_raw(a);
    //         // if let Some(adp) = _adp {
    //     // exm.register(a.clone().as_mut())
    //         // }
    //     // }
    //     exm.register( a);
    //
    //     // let mut adp = a;
    //
    //
    // }
    for p in &tpairs {
        println!();
        let lp = exm.get_rate(
            p.from_coin.as_str(), p.to_coin.as_str()
        ).await;
        if lp.is_err() {
            warn!("Failed to get exchange rate for pair: {}", p);
            warn!("Error for pair {} is: {}", p, lp.unwrap_err().to_string());
            continue;
        }
        let lres = lp.unwrap();
        println!("Exchange rates for pair {} are: {:#?}", p, lres);
    }
    // }
    // exm.register(&mut KrakenAdapter::new()).await?;
    // exm.register(&mut HuobiAdapter::new()).await?;
    // exm.register(&mut BittrexAdapter::new()).await?;

    // for p in tpairs {
    //     println!();
    //     let lp = exm.get_rate(
    //         p.from_coin.as_str(), p.to_coin.as_str()
    //     ).await;
    //     if lp.is_err() {
    //         warn!("Failed to get exchange rate for pair: {}", p);
    //         warn!("Error for pair {} is: {}", p, lp.unwrap_err().to_string());
    //         continue;
    //     }
    //     let lres = lp.unwrap();
    //     println!("Exchange rates for pair {} are: {:#?}", p, lres);
    // }
    Ok(())
    /*
    let mut kc: KrakenCore = KrakenCore::new();

    // let lp = kc.load_pairs().await.unwrap();
    let lp = kc.get_ticker_main("DOGE", "BTC").await.unwrap();
    println!("Kraken pairs: {:#?}", lp);
    */

    /*
    let mut hb = Huobi::new();
    let mut btx = Bittrex::new();
    let hpairs: Pairs = hb.get_pairs().await.unwrap();
    let bpairs: Pairs = btx.get_pairs().await.unwrap();
    println!("Huobi Pairs: {}", hpairs.len());
    println!("Bittrex Pairs: {}", bpairs.len());
    println!("Huobi Adapter - Name: {} || Code: {}", hb.name(), hb.code());
    println!("Bittrex Adapter - Name: {} || Code: {}", btx.name(), btx.code());
    println!("Bittrex Pairs: {}", bpairs.len());
    for p in tpairs {
        let hrate = hb.get_rate(p.from_coin.as_str(), p.to_coin.as_str()).await.unwrap();
        println!("{} exchange rate (huobi): {:#?}", p.symbol(), hrate);
        let brate = btx.get_rate(p.from_coin.as_str(), p.to_coin.as_str()).await.unwrap();
        println!("{} exchange rate (bittrex): {:#?}", p.symbol(), brate);
    }
    */


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
