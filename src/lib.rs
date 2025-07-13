#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod chat;
pub mod client;
pub mod embed;
pub mod imagen;
pub mod messages;
pub mod models;
pub mod requests;
pub mod responses;
pub mod stream;
pub mod tokens;
pub mod traits;
pub mod tts;
pub mod utils;
pub mod vidgen;
pub mod vision;

#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "cli")]
pub mod tui;

pub use client::Client;
