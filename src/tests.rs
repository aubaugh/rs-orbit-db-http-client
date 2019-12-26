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
#[should_panic(expected = "Invalid API request arguments")]
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
