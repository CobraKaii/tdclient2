//! # tdameritradeclient 
//!
//! TDAClient is the main module that lets you build requests to TDAmeritrade's API
//! 
//! See [TD Ameritrade API](http://developer.tdameritrade.com) for API documentation
//! 
//! Create TDAClient with a valid Access Token - see [TD Ameritrade API](http://developer.tdameritrade.com) for info on creating token
//! 
//! Response output can be kept in text which comes out as JSON text or converted to a `serde_json::Value` object
//!
//! # Query parameters through Enum
//! 
//! Use the relevant associated Enums in param to add any parameters to the get function request on the TDAClient

mod tdaclient;
mod param;

pub use tdaclient::TDAClient as TDAClient;
pub use param::{Account, OptionChain, History, Order};

