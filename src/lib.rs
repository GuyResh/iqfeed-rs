//! A IQFeed client written in Rust.
//!
//! This crate was originally written for use at Oxide Financial and thus there
//! may not be 100% coverage of what data IQFeed provides. However overtime and
//! on request we will attempt to add any features/data that are not already
//! implemented.
//!
//! By default iqfeed-rs pulls in `rkyv` for use on all [`models::Ops`]. This is
//! because we heavily use `rkyv` in production however in the future we will
//! make this either opt-in or opt-out.
//!
//! All timestamps in iqfeed-rs are formatted as a unix-nanosecond timestamp in
//! UTC time. Timestamps originate in UTC(-4/-5) depending on if DST is in
//! effect.
//!
//! # Features
//! There are not currently any features however we do plan on making `rkyv`
//! along with a choice of what channel provider you would like to use in the
//! future.

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]
#![allow(clippy::unit_arg)]

pub mod client;
pub mod errors;
pub mod models;
