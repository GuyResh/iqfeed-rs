use std::str::Utf8Error;

use lexical::{Error, ErrorCode};
use thiserror::Error;
use tokio::io;

/// `ParsingError` is an error returned from anything having to do with parsing
/// data.
#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("error parsing number")]
    Number(ErrorCode),
    #[error("error parsing time")]
    Timestamp(#[from] time::error::Parse),
    #[error("error converting to utf")]
    UTf8(#[from] Utf8Error),
    #[error("error parsing float")]
    Float(#[from] fast_float::Error),
}

impl From<lexical::Error> for ParsingError {
    fn from(e: Error) -> Self { Self::Number(e.code) }
}

/// `ClientError` is returned from anything having to do with processing data
/// from `IQFeed`'s client to sending to the channel.
#[derive(Error, Debug)]
pub enum ClientError {
    #[error("error parsing number")]
    TcpError(#[from] io::Error),
    #[error("error sending msg over channel")]
    ChannelError(#[from] async_channel::SendError<Vec<u8>>),
}
