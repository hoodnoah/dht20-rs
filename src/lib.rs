// no_std; intended for embedded environments
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
extern crate std; // to permit testing in non-embedded environment

pub mod dht20; // main dht20 module
pub mod sensor_reading; // type-safe sensor reading struct; prevent mixing up Celsius and Fahrenheit
mod utils; // utility functions for dht20

// re-export for convenience
pub use dht20::*;
