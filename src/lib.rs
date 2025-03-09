#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
pub mod tracing;

#[cfg(feature = "cache")]
#[cfg_attr(docsrs, doc(cfg(feature = "cache")))]
pub mod cache;

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub mod postgres;

mod config;
pub use config::*;

#[derive(Clone, bon::Builder)]
pub struct Services {
    #[cfg(feature = "postgres")]
    #[builder(setters(vis = "", name = pg_internal))]
    pub postgres: sqlx::PgPool,
    #[cfg(feature = "cache")]
    #[builder(setters(vis = "", name = cache_internal))]
    pub cache: cache::RedisManager,
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("service was not initialised")]
    NotInitialised,
    #[error("unknown data store error")]
    Unknown,
    #[error("invalid config `{0}`")]
    Configuration(String),
    #[cfg(feature = "postgres")]
    #[error(transparent)]
    /// Postgres error
    Postgres(#[from] sqlx::Error),
    #[cfg(feature = "cache")]
    #[error(transparent)]
    /// Redis error
    Cache(#[from] redis::RedisError),
    #[cfg(feature = "opentelemetry")]
    #[error(transparent)]
    /// When creating the tracing layer
    Opentelemetry(#[from] opentelemetry::trace::TraceError),
}
