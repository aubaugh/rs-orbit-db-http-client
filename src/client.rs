use super::DatabaseType;
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

    /// Makes an arbitrary GET request to `self.base_url/:path`,
    /// returning a JSON object on success
    async fn get(&self, path: &str) -> Result<Value, Exception> {
        let endpoint_url = self.base_url.join(path)?;

        surf::get(&endpoint_url).recv_json().await
    }

    /// Makes an arbitrary POST request to `self.base_url/:path`,
    /// sending a JSON body object and returning a JSON object
    /// on success
    async fn post(&self, path: &str, body: Value) -> Result<Value, Exception> {
        let endpoint_url = self.base_url.join(path)?;

        let response = surf::post(&endpoint_url)
            .body_json(&body)?
            .await?
            .body_json()
            .await?;

        Ok(response)
    }

    /// Makes a GET request to `self.base_url/identity`,
    /// returning the identity structure on success
    pub async fn get_identity(&self) -> Result<Identity, Exception> {
        let response = self.get("identity").await?;

        let identity = serde_json::from_value(response)?;
        Ok(identity)
    }

    /// Makes a POST request to `self.base_url/:dbname`,
    /// sending the specified db type and returning the
    /// database structure on success
    pub async fn create_database(
        &self,
        dbname: &str,
        dbtype: DatabaseType,
    ) -> Result<Database, Exception> {
        let path = format!("db/{}", dbname);
        let dbtype: String = dbtype.into();
        let body = serde_json::json!({
            "create": true,
            "type": dbtype,
        });
        let response = self.post(&path, body).await?;

        let database = serde_json::from_value(response)?;
        Ok(database)
    }

    /// Makes a GET request to `self.base_url/db/:dbname/value`,
    /// returning the counter's value on success
    pub async fn get_counter_value(&self, dbname: &str) -> Result<u64, Exception> {
        let path = format!("db/{}/value", &dbname);
        let response = self.get(&path).await?;

        if response["statusCode"] == 500 {
            Err("Expected valid counter database")?
        }

        let value = serde_json::from_value(response)?;
        Ok(value)
    }
}
