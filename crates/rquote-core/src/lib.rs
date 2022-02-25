//! # `rquote-core`
//!
//! This library provides multiple types used across all components on rquote source
//! and the Animechan API wrapper used to fetch data on rquote

pub use animechan::AnimechanQuote;

mod animechan;
pub mod wrapper;
