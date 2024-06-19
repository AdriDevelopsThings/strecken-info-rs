//! # strecken-info-rs
//! A rust SDK to make requests to [strecken.info](https://strecken.info)
//!
//! Currently only these functions are implemented:
//! * get revisions - See [revision]
//! * get disruptions - See [disruptions]

pub mod error;
mod request;

pub use request::{disruptions, filter, revision};

#[cfg(test)]
mod tests;
