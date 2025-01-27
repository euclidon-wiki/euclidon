# Euclidon changelog

## v0.1

### v0.1.0

#### API changes

##### `pub mod euclidon`
* Created module.
* Added `enum Error`
    * Defined in `mod euclidon::error`.
    * The generic error type used throughout `euclidon` crate.
    * Derives implementations for `trait Debug` and `trait thiserror::Error`.
    * Implements `trait axum::response::IntoResponse`, so it can be used in HTTP method handlers.
    * Wraps over various error types thrown by other modules or crates, and has `trait From<...>` implementations for all of them.
* Added alias `App = struct app::App`.
* Added alias `AppState = struct app::AppState`.

##### `pub mod euclidon::app`
* Created module.
* Added `struct App`
    * Represents the global data of a server instance.
    * `pub config: Config`
        * Contains server configuration.
    * `pub fn new(config: Config) -> Result<Self, Error>`
        * Constructs an app with the given configuration.
        * On failure returns `Err(euclidon::Error)`.
* Added `struct AppState`
    * Wrapper for an instance of `Arc<App>`.
    * Derives `trait Clone` and `trait axum::extract::FromRequestParts` via `struct axum::extact::State`, so it can be used directly as an extractor for axum method handlers rather than `State<Arc<App>>`.
    * Implements `trait Deref<Target = App>` so it can be seamlessly used as an `App` instance.
* Added `struct Config`
    * Contains the server configuration and options.
    * `pub server_url: String`
        * The URL of the server.
        * Default value is loaded from environmental variable `SERVER_URL` that can be changed via the .env file.
        * Alternatively, set using `.server_url(server_url: String)` function of builder.
    * `pub fn builder() -> ConfigBuilder`
        * Returns a builder. The builder type is hidden in the private module `app::detail`.
    * `pub fn load() -> Result<Self, Error>`
        * Equivalent to `Config::builder().build()`.


#### Internal changes
* Set Rust compilation version at v1.84.0 and edition to 2021.
* Added the following crates as dependencies:
    * [`tokio`](https://docs.rs/tokio) v1.43.0 with features: `rt-multi-thread`, `macros`, `net`, `fs`
    * [`axum`](https://docs.rs/axum) v0.8.1 with features: `macros`, `tokio`, `http1`, `http2`, `multipart`, `query`, `form`
    * [`dotenvy`](https://docs.rs/dotenvy) v0.15.7
    * [`thiserror`](https://docs.rs/thiserror) v2.0.11
