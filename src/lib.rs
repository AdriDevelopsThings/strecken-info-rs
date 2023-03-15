//! # strecken-info-rs
//! A rust SDK to make requests to [strecken.info](https://strecken.info)
//!
//! Currently only these functions are implemented:
//! * [`HimGeoPos`] - See [geo_pos]
//! * [`HimDetails`] - See [details]

pub mod error;
mod request;

pub use request::{details, geo_pos};

#[cfg(test)]
mod tests;
