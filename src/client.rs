use super::*;
use serde::Deserialize;
use serde_json::{json, to_value, Value};
use std::collections::HashMap;
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
pub struct Hash {
    hash: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    // This field could be a string if the db was created with
    // `curl -X POST :base_url/db/:dbname -d "create=true" -d "type=:dbtype"`
    create: bool,
    index_by: Option<String>,
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

    /// Makes a GET request to `self.base_url/dbs`,
    /// returning a hashmap of databases
    pub async fn get_dbs(&self) -> Result<HashMap<String, Database>, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Get,
            path: "dbs".into(),
            body: Value::Null,
        };

        api_request!(self, config)
    }

    /// Makes a GET request to `self.base_url/db/:dbname`,
    /// returning the database structure on success
    pub async fn get_db(&self, dbname: &str) -> Result<Database, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Get,
            path: format!("db/{}", &dbname),
            body: Value::Null,
        };

        api_request!(self, config)
    }

    /// Makes a GET request to `self.base_url/db/:dbname/value`,
    /// returning the counter's value on success
    pub async fn get_counter_value(&self, dbname: &str) -> Result<u64, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Get,
            path: format!("db/{}/value", &dbname),
            body: Value::Null,
        };

        api_request!(self, config)
    }

    /// Makes a GET request to `self.base_url/db/:dbname/:item`,
    /// returning the database's record identified by `:item` on
    /// success
    pub async fn get_db_item(&self, dbname: &str, item: &str) -> Result<Vec<Value>, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Get,
            path: format!("db/{}/{}", &dbname, &item),
            body: Value::Null,
        };

        api_request!(self, config)
    }

    /// Makes a GET request to `self.base_url/db/:dbname/iterator`,
    /// returning a possibly limited number of items from an EventLog
    /// or Feed on success
    pub async fn get_db_iterator(
        &self,
        dbname: &str,
        limit: Option<i64>,
    ) -> Result<Vec<Value>, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Get,
            path: format!("db/{}/iterator", &dbname),
            body: json!({ "limit": limit.unwrap_or(-1) }),
        };

        api_request!(self, config)
    }

    /// Makes a GET request to `self.base_url/db/:dbname/value`,
    /// returning the database information on success
    pub async fn get_db_index(&self, dbname: &str) -> Result<Value, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Get,
            path: format!("db/{}/index", &dbname),
            body: Value::Null,
        };

        api_request!(self, config)
    }

    /// Makes a GET request to `self.base_url/identity`,
    /// returning the identity structure on success
    pub async fn get_identity(&self) -> Result<Identity, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Get,
            path: "identity".into(),
            body: Value::Null,
        };

        api_request!(self, config)
    }

    /// Makes a POST request to `self.base_url/db/:dbname`,
    /// sending the specified db type and returning the
    /// database structure on success
    //
    // TODO: The use of the api arguments might be able to be done better
    // And there may be additional parameters that should be added
    pub async fn create_db(
        &self,
        dbname: &str,
        dbtype: DatabaseType,
        ac: Option<AccessController>,
        overwrite: bool,
    ) -> Result<Database, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Post,
            path: format!("db/{}", dbname),
            body: json!({
                "create": true,
                "type": dbtype.to_string(),
                "indexBy": to_value(match dbtype {
                    DatabaseType::DocStore { index_by } => index_by,
                    _ => None,
                })?,
                "accessController": to_value(ac)?,
                "overwrite": to_value(overwrite)?,
            }),
        };

        api_request!(self, config)
    }

    /// Makes a POST request to `self.base_url/db/:dbname/query`,
    /// sending the query to be interpretted and processed returning
    /// the items on success
    pub async fn db_query(&self, dbname: &str, query: Query) -> Result<Vec<Value>, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Post,
            path: format!("db/{}/query", dbname),
            body: to_value(query)?,
        };

        api_request!(self, config)
    }

    /// Makes a POST request to `self.base_url/db/:dbname/add`,
    /// sending the entry to be added to the EventLog or Feed and returning
    /// the hash on success
    pub async fn db_add(&self, dbname: &str, entry: String) -> Result<Hash, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Post,
            path: format!("db/{}/add", dbname),
            body: to_value(entry)?,
        };

        api_request!(self, config)
    }

    /// Makes a POST request to `self.base_url/db/:dbname/put`,
    /// sending the record to be added to the database and returning
    /// the hash on success
    pub async fn db_put(&self, dbname: &str, record: Value) -> Result<Hash, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Post,
            path: format!("db/{}/put", dbname),
            body: record,
        };

        api_request!(self, config)
    }

    /// Makes a POST request to `self.base_url/db/:dbname/inc`,
    /// to increment the counter database by 1 and returning
    /// the hash on success
    pub async fn db_inc(&self, dbname: &str) -> Result<Hash, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Post,
            path: format!("db/{}/inc", dbname),
            body: Value::Null,
        };

        api_request!(self, config)
    }

    /// Makes a POST request to `self.base_url/db/:dbname/inc/:value`,
    /// to increment the counter database by `:value` and returning
    /// the hash on success
    pub async fn db_inc_val(&self, dbname: &str, value: u64) -> Result<Hash, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Post,
            path: format!("db/{}/inc/{}", dbname, value),
            body: Value::Null,
        };

        api_request!(self, config)
    }

    /// Makes a POST request to `self.base_url/db/:dbname/inc/:value`,
    /// to increment the counter database by `:value` and returning
    /// the hash on success
    pub async fn db_write_access(&self, dbname: &str, id: String) -> Result<Hash, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Post,
            path: format!("db/{}/access/write", dbname),
            body: json!({ "id": id }),
        };

        api_request!(self, config)
    }

    /// Makes a DELETE request to `self.base_url/db/:dbname`,
    /// to delete the specified database and returning
    /// the hash on success
    pub async fn delete_db(&self, dbname: &str) -> Result<HashMap<(), ()>, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Delete,
            path: format!("db/{}", dbname),
            body: Value::Null,
        };

        api_request!(self, config)
    }

    /// Makes a DELETE request to `self.base_url/db/:dbname/:item`,
    /// to delete the specified item from the database and returning
    /// the hash on success
    pub async fn delete_db_item(&self, dbname: &str, item: &str) -> Result<Hash, Exception> {
        let config = RequestConfig {
            rtype: RequestType::Delete,
            path: format!("db/{}/{}", dbname, item),
            body: Value::Null,
        };

        api_request!(self, config)
    }
}
