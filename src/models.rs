use std::str::from_utf8;

use lexical::parse;
use once_cell::sync::Lazy;
use rkyv::{Archive, Deserialize, Serialize};
use time::{format_description, OffsetDateTime, Time, UtcOffset};

use crate::errors::ParsingError;

pub enum Ops {
    Trade(Trade),
    Timestamp(Timestamp),
    ServerMessage,
    None,
}

static NANO_PARSE: Lazy<Vec<format_description::FormatItem>> =
    Lazy::new(|| format_description::parse("[hour]:[minute]:[second].[subsecond digits:6]").unwrap());

static PARSE_TIMESTAMP: Lazy<Vec<format_description::FormatItem>> =
    Lazy::new(|| format_description::parse("[year][month][day] [hour]:[minute]:[second]").unwrap());

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
        let msg: Vec<&str> = from_utf8(src)?.split(',').collect();

        match msg[0] {
            "Q" => Ok(Self::Trade(Trade::parse(&msg)?)),
            "T" => Ok(Self::Timestamp(Timestamp::parse(&msg)?)),
            "O" => Ok(Self::ServerMessage),
            _ => Ok(Self::None),
        }
    }
}

/// Trade is the Op for any trade returned.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Trade {
    pub symbol: String,
    pub most_recent_trade: f32,
    pub most_recent_trade_size: i32,
    pub most_recent_trade_time: i128,
    pub most_recent_trade_market_center: i32,
    pub total_volume: i32,
    pub bid: Option<f32>,
    pub bid_size: Option<f32>,
    pub ask: Option<f32>,
    pub ask_size: Option<i32>,
    pub open: Option<f32>,
    pub high: Option<f32>,
    pub low: Option<f32>,
    pub close: Option<f32>,
    pub message_contents: String,
    pub most_recent_trade_conditions: String,
}

impl Trade {
    fn parse(msg: &[&str]) -> Result<Self, ParsingError> {
        Ok(Self {
            symbol: msg[1].into(),
            most_recent_trade: parse(msg[2])?,
            most_recent_trade_size: parse(msg[3])?,
            most_recent_trade_time: OffsetDateTime::now_utc()
                .replace_time(Time::parse(msg[4], &NANO_PARSE.as_ref())?)
                .to_offset(UtcOffset::UTC)
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

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Timestamp {
    timestamp: i128,
}

impl Timestamp {
    fn parse(msg: &[&str]) -> Result<Self, ParsingError> {
        Ok(Self {
            timestamp: OffsetDateTime::now_utc()
                .replace_time(Time::parse(msg[1], &PARSE_TIMESTAMP.as_ref())?)
                .to_offset(UtcOffset::UTC)
                .unix_timestamp_nanos(),
        })
    }
}
