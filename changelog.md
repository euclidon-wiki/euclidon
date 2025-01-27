# Euclidon changelog

## v0.1

### v0.1.0

#### Internal changes
* Set Rust compilation version at v1.84.0 and edition to 2021.
* Added the following crates as dependencies:
    * [`tokio`](https://docs.rs/tokio) v1.43.0 with features: `rt-multi-thread`, `macros`, `net`, `fs`
    * [`axum`](https://docs.rs/axum) v0.8.1 with features: `macros`, `tokio`, `http2`, `multipart`, `query`, `form`
    * [`dotenvy`](https://docs.rs/dotenvy) v0.15.7
    * [`thiserror`](https://docs.rs/thiserror) v2.0.11
