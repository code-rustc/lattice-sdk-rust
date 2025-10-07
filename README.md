# Lattice SDK Rust Library

![](https://www.anduril.com/lattice-sdk/)

[![fern shield](https://img.shields.io/badge/%F0%9F%8C%BF-Built%20with%20Fern-brightgreen)](https://buildwithfern.com?utm_source=github&utm_medium=github&utm_campaign=readme&utm_source=https%3A%2F%2Fgithub.com%2Fcode-rustc%2Flattice-sdk-rust)
[![crates.io shield](https://img.shields.io/crates/v/lattice)](https://crates.io/crates/lattice)

The Lattice SDK Rust library provides convenient access to the Lattice SDK APIs from Rust.

## Documentation

API reference documentation is available [here](https://developer.anduril.com/).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lattice = "0.1.0"
```

Or install via cargo:

```sh
cargo add lattice
```

## Reference

A full reference for this library is available [here](https://github.com/code-rustc/lattice-sdk-rust/blob/HEAD/./reference.md).

## Usage

Instantiate and use the client with the following:

```rust
use lattice::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client
        .entities
        .long_poll_entity_events(
            &EntityEventRequest {
                session_token: "sessionToken".to_string(),
            },
            None,
        )
        .await;
}
```

## Errors

When the API returns a non-success status code (4xx or 5xx response), an error will be returned.

```rust
use lattice::prelude::{*};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let config = ClientConfig {
        base_url: " ".to_string(),
        api_key: Some("your-api-key".to_string())
    };
    let client = LatticeClient::new(config)?;
    match client.some_method().await {
        Ok(response) => {
            println!("Success: {:?}", response);
        },
        Err(ApiError::HTTP { status, message }) => {
            println!("API Error {}: {:?}", status, message);
        },
        Err(e) => {
            println!("Other error: {:?}", e);
        }
    }
    return Ok(());
}
```

## Advanced

### Retries

The SDK is instrumented with automatic retries with exponential backoff. A request will be retried as long
as the request is deemed retryable and the number of retry attempts has not grown larger than the configured
retry limit (default: 2).

A request is deemed retryable when any of the following HTTP status codes is returned:

- [408](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/408) (Timeout)
- [429](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/429) (Too Many Requests)
- [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500) (Internal Server Errors)

Use the `max_retries` method to configure this behavior.

```rust
use lattice::prelude::{*};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        base_url: " ".to_string(),
        api_key: Some("your-api-key".to_string()),
        max_retries: 3
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
}
```

### Timeouts

The SDK defaults to a 30 second timeout. Use the `timeout` method to configure this behavior.

```rust
use lattice::prelude::{*};
use std::time::{Duration};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        base_url: " ".to_string(),
        api_key: Some("your-api-key".to_string()),
        timeout: Duration::from_secs(30)
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
}
```
