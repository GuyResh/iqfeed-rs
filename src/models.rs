use std::str::from_utf8;

use lexical::parse;
use rkyv::{Archive, Deserialize, Serialize};

use crate::errors::ParsingError;

pub enum Ops {
    Trade(Trade),
    Timestamp,
    None,
}

impl Ops {
    /// Parses a Vec<u8> into a valid `IQFeed` parsed message
    ///
    /// # Errors
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
    /// Parses comma delimited to `Trade`
    ///
    /// # Errors
    /// If there is an issue parsing the CSV a `ParsingError` is returned
    ///
    /// # Panics
    /// This will panic if there's an issue creating a format desc. Will be
    /// removed at some point.
    ///
    /// # Example
    /// ```
    /// # use iqfeed_rs::models::Ops;
    /// # let src = b"Q,GME,190.0000,1,16:40:18.814943,19,8346145,189.56,190.0000,300,197.500,199.4600.187.1102,O,8717";
    /// let trade = Ops::parse(src).unwrap();
    /// ```
    pub fn parse(msg: &[&str]) -> Result<Self, ParsingError> {
        // TODO: Move this to a oncecell maybe.
        let tod = time::format_description::parse("[year].[month].[day] [hour]:[minute]:[second].[subsecond digits:9]")
            .unwrap();
        let today = time::OffsetDateTime::now_utc();

        Ok(Self {
            symbol: msg[1].into(),
            most_recent_trade: parse(msg[2])?,
            most_recent_trade_size: parse(msg[3])?,
            most_recent_trade_time: time::OffsetDateTime::parse(
                &format!("{}.{}.{} {}000", today.year(), today.month(), today.day(), msg[4]),
                &tod,
            )
            .unwrap()
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
