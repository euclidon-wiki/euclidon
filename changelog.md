# Euclidon changelog

## v0.1

### v0.1.0

#### API changes

##### Crate `euclidon`
* Created crate.
* Added `enum Error`
    * Defined in `mod euclidon::error`.
    * The generic error type used throughout `euclidon` crate.
    * Derives implementations for `trait Debug` and `trait thiserror::Error`.
    * Implements `trait axum::response::IntoResponse`, so it can be used in HTTP method handlers.
    * Wraps over various error types thrown by other modules or crates, and has `trait From<...>` implementations for all of them.
* Added alias `App = struct app::App`.
* Added alias `AppState = struct app::AppState`.
* Added `pub fn build_router(app: Arc<App>) -> axum::Router`
    * Defined in `mod euclidon::router`.
    * Constructs an axum router with the provided `app` instance as its state.
    * State can be extracted via the custom `AppState` extractor.

##### `pub mod euclidon::app`
* Created module.
* Added `struct App`
    * Represents the global data of a server instance.
    * `pub db: Db`
        * Manages connections to the database.
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

##### `pub mod euclidon::db`
* Created module.
* Added `struct Db`
    * Manages connections to the database through an `r2d2::Pool` instance.
    * `pub fn new(config: &Config) -> Result<Self, Error>`
        * Constructs an instance with the given configuration.
        * Specifically, the configuration decides the database URL via its `config.database_url` field.
    * `pub fn conn(&self) -> Result<PooledConnection<ConnMan>, Error>`
        * Returns a usable connection from the pool.
        * Could fail if all connections are occupied or else some problem occurs.
* Added `enum AnyConn`
    * A backend-agnostic version of a diesel connection.
    * Dervies `diesel::MultiConnection`, which propogates function calls down to the enum variants.
    * `Pg(PgConnection)`
        * Variant used for Postgres backend.
* Added `type ConnMan = ConnectionManager<AnyConn>`
    * A type alias used to shorten the name of the connection manager type, and doubles as a pun.

##### `pub mod euclidon::controllers`
* Created module.
* Contains all method handlers for different routes. Each route is mapped to a corresponding submodule, and each submodule contains controllers for the same route with different methods such as `GET` or `POST`.
* Route to controller mapping is as follows:
    * `/` to `root::get` with `GET`

#### Internal changes
* Set Rust compilation version at v1.84.0 and edition to 2021.
* Added the following crates as dependencies:
    * [`tokio`](https://docs.rs/tokio/1.43.0) v1.43.0 with features: `rt-multi-thread`, `macros`, `net`, `fs`
    * [`axum`](https://docs.rs/axum/0.8.1) v0.8.1 with features: `macros`, `tokio`, `http1`, `http2`, `multipart`, `query`, `form`
    * [`dotenvy`](https://docs.rs/dotenvy/0.15.7) v0.15.7
    * [`thiserror`](https://docs.rs/thiserror/2.0.11) v2.0.11
    * [`chrono`](https://docs.rs/chrono/0.4.39) v0.4.39 with features: `std`, `clock`, `serde`
    * [`diesel`](https://docs.rs/diesel/2.2.6) v2.2.6 with features: `with-deprecated`, `chrono`, `postgres`, `r2d2`
    * [`serde`](https://docs.rs/serde/1.0.217) v1.0.217 with features: `std` (default), `derive`
    * [`tera`](https://docs.rs/tera/1.20.0) v1.20.0 with features: `chrono`, `urlencode`
