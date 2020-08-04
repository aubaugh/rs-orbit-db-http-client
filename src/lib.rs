//! Client library used for communicating with (OrbitDB's REST API server)[https://github.com/orbitdb/orbit-db-http-api]

use serde::Serialize;
use serde_json::Value;

pub use client::Client;

extern crate strum;
#[macro_use]
extern crate strum_macros;

/// The types of API requests
pub enum RequestType {
    Get,
    Post,
    Delete,
}

/// Settings for an API request
struct RequestConfig<'a> {
    /// The type of request
    rtype: RequestType,
    /// The path to be concatenated to the client's base url
    path: String,
    /// The body json value
    body: &'a Value,
}

/// Makes an arbitrary API request based on the provided `Client` and `RequestConfig`
macro_rules! api_request {
    ($client:ident, $config:ident) => {{
        let uri = $client.base_url.join(&$config.path)?;

        let response: Value = match $config.rtype {
            RequestType::Get => {
                surf::get(&uri)
                    .body_json($config.body)?
                    .await?
                    .body_json()
                    .await?
            }
            RequestType::Post => {
                surf::post(&uri)
                    .body_json($config.body)?
                    .await?
                    .body_json()
                    .await?
            }
            RequestType::Delete => surf::delete(&uri).recv_json().await?,
        };

        let value = serde_json::from_value(response)?;
        Ok(value)
    }};
}

mod client;

/// The types of OrbitDB databases
#[derive(Debug, ToString, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum DatabaseType {
    EventLog,
    Feed,
    DocStore { index_by: Option<String> },
    KeyValue,
    Counter,
}
#[derive(Debug, Serialize)]
pub struct AccessController {
    pub r#type: String,
    pub write: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct Query {
    pub propname: Option<String>,
    pub comp: Option<Comparison>,
    pub values: Vec<i64>,
}
#[derive(Debug, ToString, Serialize)]
#[strum(serialize_all = "lowercase")]
pub enum Comparison {
    /// ==
    EQ,
    /// !=
    NE,
    /// >
    GT,
    /// <
    LT,
    /// >=
    GTE,
    /// <=
    LTE,
    /// %
    Mod,
    /// Those between min and max
    Range,
    /// *
    All,
}
