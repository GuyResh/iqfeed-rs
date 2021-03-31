use std::str::Utf8Error;

use lexical::{Error, ErrorCode};
use thiserror::Error;
use tokio::io;

/// `ParsingError` is an error returned from anything having to do with parsing
/// data.
#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("error parsing number: {0}")]
    Number(String),
    #[error("error parsing time")]
    Timestamp(#[from] time::error::Parse),
    #[error("error converting to utf")]
    UTf8(#[from] Utf8Error),
}

impl From<lexical::Error> for ParsingError {
    fn from(e: Error) -> Self {
        match e.code {
            ErrorCode::Overflow => Self::Number("overflow".into()),
            ErrorCode::Underflow => Self::Number("underflow".into()),
            ErrorCode::InvalidDigit => Self::Number("invalid digit".into()),
            ErrorCode::Empty => Self::Number("empty".into()),
            ErrorCode::EmptyMantissa => Self::Number("empty mantissa".into()),
            ErrorCode::EmptyExponent => Self::Number("empty exponent".into()),
            ErrorCode::EmptyInteger => Self::Number("empty integer".into()),
            ErrorCode::EmptyFraction => Self::Number("empty fraction".into()),
            ErrorCode::InvalidPositiveMantissaSign => Self::Number("invalid positive mantissa sign".into()),
            ErrorCode::MissingMantissaSign => Self::Number("missing mantissa sign".into()),
            ErrorCode::InvalidExponent => Self::Number("invalid exponent".into()),
            ErrorCode::InvalidPositiveExponentSign => Self::Number("invalid positive exponent sign".into()),
            ErrorCode::MissingExponentSign => Self::Number("missing exponent sign".into()),
            ErrorCode::ExponentWithoutFraction => Self::Number("exponent without fraction".into()),
            ErrorCode::InvalidLeadingZeros => Self::Number("invalid leading zeros".into()),
            ErrorCode::__Nonexhaustive => Self::Number("__Nonexhaustive".into()),
        }
    }
}

/// `ClientError` is returned from anything having to do with processing data
/// from IQFeed's client to sending to the channel.
#[derive(Error, Debug)]
pub enum ClientError {
    #[error("error parsing number")]
    TcpError(#[from] io::Error),
    #[error("error sending msg over channel")]
    ChannelError(#[from] async_channel::SendError<Vec<u8>>),
}
