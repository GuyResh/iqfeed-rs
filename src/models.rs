use std::str::from_utf8;

use lexical::parse;
use once_cell::sync::Lazy;
use rkyv::{Archive, Deserialize, Serialize};

use crate::errors::ParsingError;

pub enum Ops {
    Trade(Trade),
    Timestamp,
    None,
}

static TIME_PARSER: Lazy<Vec<time::format_description::FormatItem>> =
    Lazy::new(|| time::format_description::parse("[hour]:[minute]:[second].[subsecond digits:6]").unwrap());

impl Ops {
    /// Parses a Vec<u8> into a valid `IQFeed` parsed message
    ///
    /// # Errors
    ///
    /// # Example
    /// ```
    /// # use iqfeed_rs::models::Ops;
    ///
    /// let src = b"Q,GME,190.0000,1,16:40:18.814943,19,8346145,189.56,190,300,197,199.4600,187.1102,0.0,8717,O,";
    /// let op = Ops::parse(src).unwrap();
    ///
    /// match op {
    ///     Ops::Trade(trade) => {
    ///         assert_eq!(trade.symbol, "GME")
    ///     },
    ///     _ => {},
    /// }
    /// ```
    #[allow(clippy::match_on_vec_items)]
    pub fn parse(src: &[u8]) -> Result<Self, ParsingError> {
        let msg = from_utf8(src)?.split(',').collect::<Vec<_>>();

        match msg[0] {
            "Q" => Ok(Self::Trade(Trade::parse(&msg)?)),
            _ => Ok(Self::None),
        }
    }
}

/// Trade is the Op for any trade returned.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Trade {
    pub symbol: String,
    pub most_recent_trade: f64,
    pub most_recent_trade_size: i32,
    pub most_recent_trade_time: i128,
    pub most_recent_trade_market_center: i32,
    pub total_volume: i32,
    pub bid: Option<f64>,
    pub bid_size: Option<i32>,
    pub ask: Option<f64>,
    pub ask_size: Option<i32>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub message_contents: String,
    pub most_recent_trade_conditions: String,
}

impl Trade {
    fn parse(msg: &[&str]) -> Result<Self, ParsingError> {
        Ok(Self {
            symbol: msg[1].into(),
            most_recent_trade: parse(msg[2])?,
            most_recent_trade_size: parse(msg[3])?,
            most_recent_trade_time: time::OffsetDateTime::now_utc()
                .replace_time(time::Time::parse(msg[4], &TIME_PARSER.as_ref())?)
                .to_offset(time::UtcOffset::UTC)
                .unix_timestamp_nanos(),
            most_recent_trade_market_center: parse(msg[5])?,
            total_volume: parse(msg[6])?,
            bid: match msg[7] {
                "" => None,
                _ => Some(parse(msg[7])?),
            },
            bid_size: match msg[8] {
                "" => None,
                _ => Some(parse(msg[8])?),
            },
            ask: match msg[9] {
                "" => None,
                _ => Some(parse(msg[9])?),
            },
            ask_size: match msg[10] {
                "" => None,
                _ => Some(parse(msg[10])?),
            },
            open: match msg[11] {
                "" => None,
                _ => Some(parse(msg[11])?),
            },
            high: match msg[12] {
                "" => None,
                _ => Some(parse(msg[12])?),
            },
            low: match msg[13] {
                "" => None,
                _ => Some(parse(msg[13])?),
            },
            close: match msg[14] {
                "" => None,
                _ => Some(parse(msg[14])?),
            },
            message_contents: msg[15].into(),
            most_recent_trade_conditions: msg[16].into(),
        })
    }
}
