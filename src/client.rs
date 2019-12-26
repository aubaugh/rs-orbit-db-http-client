use super::{DatabaseType, RequestConfig, RequestType};
use serde::Deserialize;
use serde_json::Value;
use surf::Exception;
use url::Url;

/// The structure used for making requests to an OrbitDB REST API
pub struct Client {
    /// OrbitDB REST server url
    base_url: Url,
}

/// The information to uniquely identify the OrbitDB instance and sign its entries
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    id: String,
    public_key: String,
    signatures: Signatures,
    r#type: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Signatures {
    id: String,
    public_key: String,
}
/// The information pertaining to an OrbitDB database
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    address: Address,
    dbname: String,
    id: String,
    options: Options,
    can_append: bool,
    write: Vec<String>,
    r#type: String,
    capabilities: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct Address {
    root: String,
    path: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    create: bool,
    local_only: bool,
    max_history: i64,
    overwrite: bool,
    replicate: bool,
}

impl Client {
    /// The constructor
    pub fn new(base_url: Url) -> Self {
        Client { base_url }
    }

    /// Makes a GET request to `self.base_url/identity`,
    /// returning the identity structure on success
    pub async fn get_identity(&self) -> Result<Identity, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Get,
            path: "identity".into(),
            body: serde_json::json!({}),
        };
        api_request!(self, config)
    }

    /// Makes a POST request to `self.base_url/:dbname`,
    /// sending the specified db type and returning the
    /// database structure on success
    //
    // TODO: this method needs to take arbitrary parameters
    // for creating the database, specifying its type is the
    // minimum required
    pub async fn create_database(
        &self,
        dbname: &str,
        dbtype: DatabaseType,
    ) -> Result<Database, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Post,
            path: format!("db/{}", dbname),
            body: serde_json::json!({
                "create": true,
                "type": dbtype.to_string(),
            }),
        };
        api_request!(self, config)
    }

    /// Makes a GET request to `self.base_url/db/:dbname/value`,
    /// returning the counter's value on success
    pub async fn get_counter_value(&self, dbname: &str) -> Result<u64, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Get,
            path: format!("db/{}/value", &dbname),
            body: serde_json::json!({}),
        };
        api_request!(self, config)
    }
}
