#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod client;
pub mod requests;
pub mod responses;
pub mod utils;

#[cfg(feature = "cli")]
pub mod cli;

pub use client::Client;
