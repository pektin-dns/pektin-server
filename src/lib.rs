use std::env;
use thiserror::Error;

pub mod persistence;
pub use trust_dns_proto as proto;

#[derive(Debug, Error)]
pub enum PektinError {
    #[error("redis error")]
    RedisError(#[from] redis::RedisError),
    #[error("could not (de)serialize JSON: `{0}`")]
    JsonError(#[from] serde_json::Error),
    #[error("invalid DNS data")]
    ProtoError(#[from] trust_dns_proto::error::ProtoError),
    #[error("data in redis invalid")]
    InvalidRedisData,
    #[error("redis key does not exist")]
    RedisKeyNonexistent,
    #[error("requested redis key had an unexpected type")]
    WickedRedisValue,
}
pub type PektinResult<T> = Result<T, PektinError>;

pub fn load_env(default_parameter: &str, parameter_name: &str) -> String {
    let mut p = String::from(default_parameter);
    if let Ok(param) = env::var(parameter_name) {
        if param.len() > 0 {
            p = param;
        }
    };
    println!("{}: {}", parameter_name, p);
    return p;
}
