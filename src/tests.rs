use super::*;
use serde_json::json;
use surf::Exception;
use url::Url;

/// Tests `client.get_dbs()`
#[async_attributes::test]
async fn get_dbs() -> Result<(), Exception> {
    let url = Url::parse("https://localhost:3000")?;
    let client = Client::new(url);

    let dbname = "feed";
    let dbtype = DatabaseType::Feed;

    client.create_db(dbname, dbtype, None, false).await?;

    client.get_dbs().await?;
    Ok(())
}

/// Tests success of `client.get_db(:dbname)`
#[async_attributes::test]
async fn get_db_ok() -> Result<(), Exception> {
    let url = Url::parse("https://localhost:3000")?;
    let client = Client::new(url);

    let dbname = String::from("feed");
    let dbtype = DatabaseType::Feed;

    client.create_db(&dbname, dbtype, None, false).await?;

    client.get_db(&dbname).await?;
    Ok(())
}

/// Tests failure of `client.get_db(:dbname)`
#[async_attributes::test]
#[should_panic(expected = "missing field `address`")]
async fn get_db_err() {
    let url = Url::parse("https://localhost:3000").unwrap();
    let client = Client::new(url);

    let dbname = "fake";

    // This `.unwrap()` should panic at the expected error
    client.get_db(dbname).await.unwrap();
}

/// Tests success of `client.get_counter_value(:dbname)`
#[async_attributes::test]
async fn get_counter_value_ok() -> Result<(), Exception> {
    let url = Url::parse("https://localhost:3000")?;
    let client = Client::new(url);

    let dbname = String::from("counter");
    let dbtype = DatabaseType::Counter;

    client.create_db(&dbname, dbtype, None, false).await?;

    assert_eq!(client.get_counter_value(&dbname).await?, 0);
    Ok(())
}

/// Tests failure of `client.get_counter_value(:dbname)`
#[async_attributes::test]
#[should_panic(expected = "Invalid API request arguments")]
async fn get_counter_value_err() {
    let url = Url::parse("https://localhost:3000").unwrap();
    let client = Client::new(url);

    let dbname = String::from("feed");
    let dbtype = DatabaseType::Feed;

    client
        .create_db(&dbname, dbtype, None, false)
        .await
        .unwrap();

    // This `.unwrap()` should panic at the expected error
    client.get_counter_value(&dbname).await.unwrap();
}

/// Tests success of `client.get_db_item(:dbname, :item)`
#[async_attributes::test]
async fn get_db_item_ok() -> Result<(), Exception> {
    let url = Url::parse("https://localhost:3000")?;
    let client = Client::new(url);

    let dbname = String::from("docstore");
    let dbtype = DatabaseType::DocStore { index_by: None };
    let record = json!({ "_id": 1, "value": "test" });

    client.create_db(&dbname, dbtype, None, false).await?;

    client.db_put(&dbname, record.clone()).await?;

    assert_eq!(client.get_db_item(&dbname, "1").await?, vec![record]);
    Ok(())
}

/// Tests failure of `client.get_db_item(:dbname, :item)`
#[async_attributes::test]
#[should_panic(expected = "Invalid API request arguments")]
async fn get_db_item_err() {
    let url = Url::parse("https://localhost:3000").unwrap();
    let client = Client::new(url);

    // This `.unwrap()` should panic at the expected error
    client.get_db_item("fake", "item").await.unwrap();
}

/// Tests success of `client.get_db_iterator(:dbname)`
#[async_attributes::test]
async fn get_db_iterator_ok() -> Result<(), Exception> {
    let url = Url::parse("https://localhost:3000")?;
    let client = Client::new(url);

    let dbname = String::from("feed");
    let dbtype = DatabaseType::Feed;

    client.create_db(&dbname, dbtype, None, false).await?;

    client.get_db_iterator(&dbname, None).await?;
    Ok(())
}

/// Tests failure of `client.get_db_iterator(:dbname)`
#[async_attributes::test]
#[should_panic(expected = "Invalid API request arguments")]
async fn get_db_iterator_err() {
    let url = Url::parse("https://localhost:3000").unwrap();
    let client = Client::new(url);

    let dbname = String::from("docstore");
    let dbtype = DatabaseType::DocStore { index_by: None };

    client
        .create_db(&dbname, dbtype, None, false)
        .await
        .unwrap();

    // This `.unwrap()` should panic at the expected error
    client.get_db_iterator(&dbname, None).await.unwrap();
}

/// Tests `client.get_db_index(:dbname)`
#[async_attributes::test]
async fn get_db_index() -> Result<(), Exception> {
    let url = Url::parse("https://localhost:3000")?;
    let client = Client::new(url);

    let dbname = String::from("docstore");
    let dbtype = DatabaseType::DocStore { index_by: None };
    let record = json!({ "_id": 1, "value": "test" });

    client.create_db(&dbname, dbtype, None, false).await?;

    client.db_put(&dbname, record).await?;

    client.get_db_index(&dbname).await?;
    Ok(())
}

/// Tests `client.get_identity()`
#[async_attributes::test]
async fn get_identity() -> Result<(), Exception> {
    let url = Url::parse("https://localhost:3000")?;
    let client = Client::new(url);

    client.get_identity().await?;
    Ok(())
}

/// Tests `client.create_db(
///            :dbname,
///            :dbtype,
///            :access-controller,
///            :overwrite
///        )`
#[async_attributes::test]
async fn create_db() -> Result<(), Exception> {
    let url = Url::parse("https://localhost:3000")?;
    let client = Client::new(url);

    let dbname = "docstore";
    let dbtype = DatabaseType::DocStore { index_by: None };

    client.create_db(dbname, dbtype, None, false).await?;
    Ok(())
}

/// Tests `client.db_query(:dbname, :query)`
#[async_attributes::test]
async fn db_query() -> Result<(), Exception> {
    let url = Url::parse("https://localhost:3000")?;
    let client = Client::new(url);

    let dbname = String::from("docstore");
    let dbtype = DatabaseType::DocStore { index_by: None };
    let record = json!({ "_id": 1, "value": "test" });
    let query = Query {
        propname: None,
        comp: None,
        values: vec![]
    };

    client.create_db(&dbname, dbtype, None, false).await?;

    client.db_put(&dbname, record.clone()).await?;

    assert_eq!(
        client.db_query(&dbname, query).await?,
        vec![record]
    );

    Ok(())
}

/// Tests `client.db_put(:dbname, :record)`
#[async_attributes::test]
async fn db_put() -> Result<(), Exception> {
    let url = Url::parse("https://localhost:3000")?;
    let client = Client::new(url);

    let dbname = String::from("docstore");
    let dbtype = DatabaseType::DocStore { index_by: None };
    let record = json!({ "_id": 1, "value": "test" });

    client.create_db(&dbname, dbtype, None, false).await?;

    client.db_put(&dbname, record).await?;

    Ok(())
}

/// Tests `client.delete_db(:dbname)`
#[async_attributes::test]
async fn delete_db() -> Result<(), Exception> {
    let url = Url::parse("https://localhost:3000")?;
    let client = Client::new(url);

    let dbname = String::from("docstore");
    let dbtype = DatabaseType::DocStore { index_by: None };

    client.create_db(&dbname, dbtype, None, false).await?;

    client.delete_db(&dbname).await?;

    Ok(())
}
