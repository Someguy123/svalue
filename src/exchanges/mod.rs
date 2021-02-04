
pub mod bittrex;
pub mod kraken;
pub mod huobi;
mod manager;
// pub mod manager;

pub use bittrex::BittrexAdapter;
pub use kraken::KrakenAdapter;
pub use huobi::HuobiAdapter;
pub use manager::{ExchangeManager};

use crate::exchange::{StdExAdapter, AdapterCombo};

// pub const ADAPTERS: Vec<Box<dyn AdapterCombo<'static>>> = vec![
//     Box::new(BittrexAdapter::new()),
//     Box::new(KrakenAdapter::new()),
//     Box::new(HuobiAdapter::new()),
// ];

