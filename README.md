# dht20

A Rust library for interfacing with the DHT20 temperature and humidity sensor. This library provides an easy-to-use API for reading temperature and humidity data from the sensor.

## Features

- Read temperature and humidity data from the DHT20 sensor.
- Simple and lightweight API.
- Designed for embedded systems and microcontrollers.

## Installation

Add `dht20` to your `Cargo.toml`:

```toml
[dependencies]
dht20-rs = {version = "0.3.1"}
```

Then, include it in your project:

```rust
use dht20_rs;
```

## Usage

Here's a basic example of how to use the library:

```rust
#![no_std]
#![no_main]

// use a delay implementation which is compatible with the HAL `DelayNs` trait (HAL 1.0)
use esp_hal::{delay::Delay, main};

// simple `println!` macro to print messages to the serial console
use esp_println::println;

use dht20_rs::Dht20;

#[main]
fn main() -> ! {
    // Set up I2C
    println!("Setting up I2C...");
    let i2c = // Set up your I2C peripheral here
    println!("Done.");

    // Set up the DHT20 sensor
    println!("Setting up DHT20 sensor...");
    let mut delay = Delay::new(); // required dependency injection; sensor requires time delays
    let mut dht20 = Dht20::new(i2c); // pass your I2C peripheral

    // initialize the sensor
    if let Err(e) = dht20.init(&mut delay) {
        println!("Failed to initialize the DHT20 sensor: {:?}", e);
        loop {}
    }
    println!("Done.");

    loop {
        // Read temperature and humidity
        match dht20.take_reading(&mut delay) {
            Ok(reading) => {
                println!("Temperature: {}°F", reading.temperature_fahrenheit()); // also supports `temperature_celsius()`
                println!("Humidity: {}%", reading.humidity());
            }
            Err(e) => {
                println!("Failed to read from the DHT20 sensor: {:?}", e);
            }
        }

        // wait between readings
        delay.delay_millis(10000);
    }
}
```

## Documentation

For detailed documentation, visit [docs.rs/dht20-rs](https://docs.rs/dht20-rs).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/yourusername/dht20).
