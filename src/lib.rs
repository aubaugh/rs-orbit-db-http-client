//! Client library used for communicating with (OrbitDB's REST API server)[https://github.com/orbitdb/orbit-db-http-api]

use std::convert::Into;
use std::str::FromStr;

mod client;
pub use client::Client;

/// The types of OrbitDB databases
pub enum DatabaseType {
    EventLog,
    Feed,
    DocStore,
    KeyValue,
    Counter,
}

/// Converting a &str to a DatabaseType enum
impl FromStr for DatabaseType {
    type Err = &'static str;

    fn from_str(dbtype: &str) -> Result<Self, Self::Err> {
        match dbtype {
            "eventlog" => Ok(Self::EventLog),
            "feed" => Ok(Self::Feed),
            "docstore" => Ok(Self::DocStore),
            "keyvalue" => Ok(Self::KeyValue),
            "counter" => Ok(Self::Counter),
            _ => Err("Invalid database type"),
        }
    }
}

/// Converting a DatabaseType enum to a String
impl Into<String> for DatabaseType {
    fn into(self) -> String {
        String::from(match self {
            Self::EventLog => "eventlog",
            Self::Feed => "feed",
            Self::DocStore => "docstore",
            Self::KeyValue => "keyvalue",
            Self::Counter => "counter",
        })
    }
}

/// Unit tests for the client methods
#[cfg(test)]
mod tests {
    use super::*;
    use surf::Exception;
    use url::Url;

    /// Tests `client.get_identity()`
    #[async_attributes::test]
    async fn test_get_identity() -> Result<(), Exception> {
        let url = Url::parse("https://localhost:3000")?;
        let client = Client::new(url);

        client.get_identity().await?;
        Ok(())
    }

    /// Tests `client.create_database()`
    #[async_attributes::test]
    async fn test_create_database() -> Result<(), Exception> {
        let url = Url::parse("https://localhost:3000")?;
        let client = Client::new(url);

        client
            .create_database("counter", DatabaseType::Counter)
            .await?;
        Ok(())
    }

    /// Tests success of `client.get_counter_value()`
    #[async_attributes::test]
    async fn test_ok_get_counter_value() -> Result<(), Exception> {
        let url = Url::parse("https://localhost:3000")?;
        let client = Client::new(url);

        client
            .create_database("counter", DatabaseType::Counter)
            .await?;

        assert_eq!(client.get_counter_value("counter").await?, 0);
        Ok(())
    }

    /// Tests failure of `client.get_counter_value()`
    #[async_attributes::test]
    #[should_panic(expected = "Expected valid counter database")]
    async fn test_err_get_counter_value() {
        let url = Url::parse("https://localhost:3000").unwrap();
        let client = Client::new(url);

        client
            .create_database("feed", DatabaseType::Feed)
            .await
            .unwrap();

        // This `.unwrap()` should panic at the expected error
        client.get_counter_value("feed").await.unwrap();
    }
}
