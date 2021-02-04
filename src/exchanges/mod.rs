
pub mod bittrex;
pub mod kraken;
pub mod huobi;
pub mod manager;

pub use bittrex::BittrexAdapter;
pub use kraken::KrakenAdapter;
pub use huobi::HuobiAdapter;
pub use manager::{AdapterBox, ExchangeManager};

use crate::exchange::{StdExAdapter, AdapterCombo};

pub const ADAPTERS: Vec<Box<AdapterCombo>> = vec![
    Box::new(BittrexAdapter::new()),
    Box::new(KrakenAdapter::new()),
    Box::new(HuobiAdapter::new()),
];

