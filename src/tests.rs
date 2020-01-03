use super::*;
use serde_json::json;
use surf::Exception;

fn client() -> Result<Client, url::ParseError> {
    Ok(Client::new(url::Url::parse("https://localhost:3000")?))
}

/// Tests `client.get_dbs()`
#[async_attributes::test]
async fn get_dbs() -> Result<(), Exception> {
    let client = client()?;

    client.get_dbs().await?;
    Ok(())
}

/// Tests success of `client.get_db(:dbname)`
#[async_attributes::test]
async fn get_db_ok() -> Result<(), Exception> {
    let client = client()?;
    let dbname = String::from("feed");

    client
        .create_db(&dbname, DatabaseType::Feed, None, false)
        .await?;

    // Tested function
    client.get_db(&dbname).await?;

    client.delete_db(&dbname).await?;
    Ok(())
}

/// Tests failure of `client.get_db(:dbname)`
#[async_attributes::test]
#[should_panic]
async fn get_db_err() {
    let client = client().unwrap();

    client.get_db("fake").await.unwrap();
}

/// Tests success of `client.get_counter_value(:dbname)`
#[async_attributes::test]
async fn get_counter_value_ok() -> Result<(), Exception> {
    let client = client()?;
    let dbname = String::from("counter2");

    client
        .create_db(&dbname, DatabaseType::Counter, None, false)
        .await?;

    assert_eq!(client.get_counter_value(&dbname).await?, 0);

    client.delete_db(&dbname).await?;
    Ok(())
}

/// Tests failure of `client.get_counter_value(:dbname)`
#[async_attributes::test]
#[should_panic]
async fn get_counter_value_err() {
    let client = client().unwrap();

    client.get_counter_value("fake").await.unwrap();
}

/// Tests success of `client.get_db_item(:dbname, :item)`
#[async_attributes::test]
async fn get_db_item_ok() -> Result<(), Exception> {
    let client = client()?;
    let dbname = String::from("docstore");
    let record = json!({ "_id": 1, "value": "test" });

    client
        .create_db(
            &dbname,
            DatabaseType::DocStore { index_by: None },
            None,
            false,
        )
        .await?;

    client.db_put(&dbname, &record).await?;

    assert_eq!(client.get_db_item(&dbname, "1").await?, vec![record]);

    client.delete_db(&dbname).await?;
    Ok(())
}

/// Tests failure of `client.get_db_item(:dbname, :item)`
#[async_attributes::test]
#[should_panic]
async fn get_db_item_err() {
    let client = client().unwrap();

    client.get_db_item("fake", "item").await.unwrap();
}

/// Tests success of `client.get_db_iterator(:dbname)`
#[async_attributes::test]
async fn get_db_iterator_ok() -> Result<(), Exception> {
    let client = client()?;
    let dbname = String::from("feed");

    client
        .create_db(&dbname, DatabaseType::Feed, None, false)
        .await?;

    // Tested function
    client.get_db_iterator(&dbname, None).await?;

    client.delete_db(&dbname).await?;
    Ok(())
}

/// Tests failure of `client.get_db_iterator(:dbname)`
#[async_attributes::test]
#[should_panic]
async fn get_db_iterator_err() {
    let client = client().unwrap();

    client.get_db_iterator("fake", None).await.unwrap();
}

/// Tests `client.get_db_index(:dbname)`
// TODO add failure scenario
#[async_attributes::test]
async fn get_db_index() -> Result<(), Exception> {
    let client = client()?;
    let dbname = String::from("docstore");
    let record = json!({ "_id": 1, "value": "test" });

    client
        .create_db(
            &dbname,
            DatabaseType::DocStore { index_by: None },
            None,
            false,
        )
        .await?;
    client.db_put(&dbname, &record).await?;

    // Tested function
    client.get_db_index(&dbname).await?;

    client.delete_db(&dbname).await?;
    Ok(())
}

/// Tests `client.get_identity()`
#[async_attributes::test]
async fn get_identity() -> Result<(), Exception> {
    let client = client()?;

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
    let client = client()?;
    let dbname = String::from("docstore");

    // Tested function
    client
        .create_db(
            &dbname,
            DatabaseType::DocStore { index_by: None },
            None,
            false,
        )
        .await?;

    client.delete_db(&dbname).await?;
    Ok(())
}

/// Tests `client.db_query(:dbname, :query)`
// TODO: add failure scenario
#[async_attributes::test]
async fn db_query() -> Result<(), Exception> {
    let client = client()?;
    let dbname = String::from("docstore");
    let record = json!({ "_id": 1, "value": "test" });
    let query = Query {
        propname: None,
        comp: None,
        values: vec![],
    };

    client
        .create_db(
            &dbname,
            DatabaseType::DocStore { index_by: None },
            None,
            false,
        )
        .await?;

    client.db_put(&dbname, &record).await?;

    assert_eq!(client.db_query(&dbname, query).await?, vec![record]);

    client.delete_db(&dbname).await?;
    Ok(())
}

/// Tests success of `client.db_add(:dbname, :entry)`
#[async_attributes::test]
async fn db_add_ok() -> Result<(), Exception> {
    let client = client()?;
    let dbname = String::from("eventlog");

    client
        .create_db(&dbname, DatabaseType::EventLog, None, false)
        .await?;

    // Tested function
    client.db_add(&dbname, "entry").await?;

    client.delete_db(&dbname).await?;
    Ok(())
}

/// Tests failure of `client.db_add(:dbname, :entry)`
#[async_attributes::test]
#[should_panic]
async fn db_add_err() {
    let client = client().unwrap();

    client.db_add("fake", "entry").await.unwrap();
}

/// Tests success of `client.inc_counter_value(:dbname, :value)`
#[async_attributes::test]
async fn inc_counter_value_ok() -> Result<(), Exception> {
    let client = client()?;
    let dbname = String::from("counter");

    client
        .create_db(&dbname, DatabaseType::Counter, None, false)
        .await?;

    // Tested function
    client.inc_counter_value(&dbname, None).await?;

    client.delete_db(&dbname).await?;
    Ok(())
}

/// Tests failure of `client.inc_counter_value(:dbname, :value)`
#[async_attributes::test]
#[should_panic]
async fn inc_counter_value_err() {
    let client = client().unwrap();

    client.inc_counter_value("fake", None).await.unwrap();
}

/// Tests success of `client.db_put(:dbname, :record)`
#[async_attributes::test]
async fn db_put_ok() -> Result<(), Exception> {
    let client = client()?;
    let dbname = String::from("docstore");
    let record = json!({ "_id": 1, "value": "test" });

    client
        .create_db(
            &dbname,
            DatabaseType::DocStore { index_by: None },
            None,
            false,
        )
        .await?;

    // Tested function
    client.db_put(&dbname, &record).await?;

    client.delete_db(&dbname).await?;
    Ok(())
}

/// Tests failure of `client.db_put(:dbname, :record)`
#[async_attributes::test]
#[should_panic]
async fn db_put_err() {
    let client = client().unwrap();
    let record = json!({ "_id": 1, "value": "test" });

    client.db_put("fake", &record).await.unwrap();
}

/// Tests `client.delete_db(:dbname)`
#[async_attributes::test]
async fn delete_db() -> Result<(), Exception> {
    let client = client()?;
    let dbname = String::from("docstore");

    client
        .create_db(
            &dbname,
            DatabaseType::DocStore { index_by: None },
            None,
            false,
        )
        .await?;

    client.delete_db(&dbname).await?;
    Ok(())
}

/// Tests success of `client.delete_db_item(:dbname, :item)`
#[async_attributes::test]
async fn delete_db_item_ok() -> Result<(), Exception> {
    let client = client()?;
    let dbname = String::from("docstore");
    let record = json!({ "_id": 1, "value": "test" });

    client
        .create_db(
            &dbname,
            DatabaseType::DocStore { index_by: None },
            None,
            false,
        )
        .await?;
    client.db_put(&dbname, &record).await?;

    // Tested function
    client.delete_db_item(&dbname, "1").await?;

    client.delete_db(&dbname).await?;
    Ok(())
}

/// Tests failure of `client.delete_db_item(:dbname, :item)`
#[async_attributes::test]
#[should_panic]
async fn delete_db_item_err() {
    let client = client().unwrap();

    client.delete_db_item("fake", "item").await.unwrap();
}
