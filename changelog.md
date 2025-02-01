# Euclidon changelog

## v0.1

### v0.1.0

#### API changes

##### Crate `euclidon`
* Created crate.
* Added type `enum Error`.
    * Defined in `mod euclidon::error`.
    * The generic error type used throughout `euclidon` crate.
    * Derives implementations for `trait Debug` and `trait thiserror::Error`.
    * Implements `trait axum::response::IntoResponse`, so it can be used in HTTP method handlers.
    * Wraps over various error types thrown by other modules or crates, and has `trait From<...>` implementations for all of them.
* Added alias `type App = struct app::App`.
* Added alias `type AppState = struct app::AppState`.
* Added function `fn build_router(app: Arc<App>) -> axum::Router`.
    * Defined in `mod euclidon::router`.
    * Constructs an axum router with the provided `app` instance as its state.
    * State can be extracted via the custom `AppState` extractor.
* Added function `fn spawn_tasks(app: Arc<App>) -> Vec<impl JoinHandle<()>>`.
    * Defined in `mod euclidon::tasks`.
    * Spawns the various background tasks required by Euclidon.

##### `mod euclidon::app`
* Created module.
* Added type `struct App`.
    * Represents the global data of a server instance.
    * Added member `config: Config`.
        * Contains server configuration.
    * Added member `assets: Assets`.
        * Manages server-side assets.
    * Added member `renderer: Renderer`
        * Manages rendering HTML responses.
    * Added member `db: Db`.
        * Manages connections to the database.
    * Added function `fn new(config: Config) -> Result<Self, Error>`.
        * Constructs an app with the given configuration.
        * On failure returns `Err(euclidon::Error)`.
* Added type `struct AppState`.
    * Wrapper for an instance of `Arc<App>`.
    * Derives `trait Clone` and `trait axum::extract::FromRequestParts` via `struct axum::extact::State`, so it can be used directly as an extractor for axum method handlers rather than `State<Arc<App>>`.
    * Implements `trait Deref<Target = App>` so it can be seamlessly used as an `App` instance.
* Added type `struct Config`.
    * Contains the server configuration and options.
    * Added member `server_url: String`.
        * The URL of the server.
        * Default value is loaded from environmental variable `SERVER_URL` provided through the .env file.
        * Alternatively, set using `.server_url(server_url: String)` function from the builder.
    * Added member `database_url: String`.
        * The URL of the database.
        * Default value is loaded from environmental variable `DATABASE_URL` provided through the .env file.
        * Alternatively, set using `.database_url(database_url: String)` function from the builder.
    * Added member `assets_dir: PathBuf`.
        * Path of the main assets directory.
        * Default value is `assets/euclidon`.
        * Alternatively, set using `.assets_dir(assets_dir: PathBuf)` function from the builder.
    * Added function `fn builder() -> ConfigBuilder`.
        * Returns a builder. The builder type is hidden in the private module `app::detail`.
    * Added function `fn load() -> Result<Self, Error>`.
        * Equivalent to `Config::builder().build()`.

##### `mod euclidon::db`
* Created module.
* Added type `struct Db`.
    * Manages connections to the database through an `r2d2::Pool` instance.
    * Added function `fn new(config: &Config) -> Result<Self, Error>`
        * Constructs an instance with the given configuration.
        * Specifically, the configuration decides the database URL via its `config.database_url` field.
    * Added function `fn conn(&self) -> Result<PooledConnection<ConnMan>, Error>`.
        * Returns a usable connection from the pool.
        * Could fail if all connections are occupied or else some problem occurs.
* Added type `enum AnyConn`.
    * A backend-agnostic version of a diesel connection.
    * Dervies `diesel::MultiConnection`, which propogates function calls down to the enum variants.
    * Added variant `Pg(PgConnection)`.
        * Variant used for Postgres backend.
* Added alias `type ConnMan = ConnectionManager<AnyConn>`.
    * A type alias used to shorten the name of the connection manager type, and doubles as a pun.

##### `mod euclidon::asset`
* Created module.
* Added type `struct Asset`.
    * Simple wrapper over a loaded asset, with the given specified type.
    * Cannot be manually constructed.
    * Added member `kind: AssetKind`.
    * Added member `data: Box<[u8]>`.
        * Data is stored as a boxed slice for its _minor_ memory efficiency.
* Added type `enum AssetKind`.
    * Represents the asset type.
    * Derives `trait Default`, with default variant `Self::None`.
    * Added function `fn from_extension(extension: Option<&str>) -> Self`.
        * Detects the type from file extension.
    * Added function `fn mime_type(&self) -> &'static str`.
        * Returns the MIME type of the asset as a static string.
    * Currently supported types are:
        * `Json` for `.json` files and has MIME type `application/json`.
        * `Css` for `.css` files and has MIME type `text/css`.
        * `JavaScript` for `.js` files and has MIME type `text/javascript`.
        * `None` for unknown files and MIME type `application/octet-stream`.
* Added type `struct Assets`.
    * Manages assets and related functionalities.
    * Added function `fn new(config: &Config) -> Self`.
        * Constructs an instance and loads the path for the assets directory root from `config.assets_dir`.
    * Added function `fn load_transient(&self, loc: Loc) -> Result<Arc<Asset>, Error>`.
        * Loads an asset from the server hard drive without caching it.
        * Useful for when it is known that the asset will not be continually used; otherwise use the following.
    * Added function `fn load(&self, loc: Loc) -> Result<Arc<Asset>, Error>`.
        * Loads a cached asset, and if not present will look for it on the filesystem.
        * Useful for when it is known the asset is used continually and needs to be kept in memory; otherwise, use the transient variant.
    * Added function `fn reload(&self, loc: Loc) -> Result<Arc<Asset>, Error>`.
        * Clears the cached asset, and then loads it again from the filesystem.
    * Added function `fn clear_cache(&self)`.
        * Clears all cached assets.
    * Added function `fn path_of(&self, loc: &Loc) -> Result<PathBuf, Error>`.
        * Takes in a `loc` and provides the associated path.
* Added type `struct Loc`.
    * Defined in `mod euclidon::asset::loc`.
    * Represents the location of an asset.
    * Added member `path: String`
        * The path of the asset, relative to its assets root directory.
    * Added function `fn new(path: String) -> Self`
        * Constructs a new instance with the given path.

##### `mod euclidon::render`
* Created module.
* Added type `struct Renderer`.
    * Uses `tera` to render HTML templates.
    * `fn new() -> Result<Self, Error>`
        * Constructs a new instance with templates already loaded.
        * Currently hard-coded but soon should dynamically detect templates instead.
    * `fn render(&self, name: &str, context: &Context) -> Result<String, Error>`
        * Renders the registered template `name` with the provided `context`.


##### `mod euclidon::controllers`
* Created module.
* Contains all method handlers for different routes. Each route is mapped to a corresponding submodule, and each submodule contains controllers for the same route with different methods such as `GET` or `POST`.
* Route to controller mapping is as follows:
    * `/` to `root` with `GET`.
    * `/assets/{*path}` to `assets` with `GET`.

#### UI changes

##### Templates
* Euclidon uses the `tera` crate for rendering templates.
* Currently in early development stages.

#### Internal changes
* Set Rust compilation version at v1.84.0 and edition to 2021.
* Added the following crates as dependencies:
    * [`tokio`](https://docs.rs/tokio/1.43.0) v1.43.0 with features: `rt-multi-thread`, `macros`, `time`, `net`, `fs`
    * [`axum`](https://docs.rs/axum/0.8.1) v0.8.1 with features: `macros`, `tokio`, `http1`, `http2`, `multipart`, `query`, `form`
    * [`dotenvy`](https://docs.rs/dotenvy/0.15.7) v0.15.7
    * [`thiserror`](https://docs.rs/thiserror/2.0.11) v2.0.11
    * [`chrono`](https://docs.rs/chrono/0.4.39) v0.4.39 with features: `std`, `clock`, `serde`
    * [`diesel`](https://docs.rs/diesel/2.2.6) v2.2.6 with features: `with-deprecated`, `chrono`, `postgres`, `r2d2`
    * [`serde`](https://docs.rs/serde/1.0.217) v1.0.217 with features: `std` (default), `derive`
    * [`serde_json`](https://docs.rs/serde_json/1.0.137) v1.0.137 with features: `std` (default)
    * [`tera`](https://docs.rs/tera/1.20.0) v1.20.0 with features: `chrono`, `urlencode`
    * [`base64`](https://docs.rs/base64/0.22.1) v0.22.1
    * [`sha3`](https://docs.rs/sha3/0.10.8) v0.10.8
    * [`hmac`](https://docs.rs/hmac/0.12.1) v0.12.1
    * [`pbkdf2`](https://docs.rs/pbkdf2/0.12.2) v0.12.2
    * [`rand_core`](https://docs.rs/rand_core/0.9.0) v0.9.0 with features: default, `os_rng`
    * [`rand_chacha`](https://docs.rs/rand_chacha/0.9.0) v0.9.0 with default features
