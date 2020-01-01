use async_std::task::block_on;
use orbit_db_http_client::{Client, DatabaseType};

fn main() -> Result<(), surf::Exception> {
    femme::start(log::LevelFilter::Info)?;

    let url = url::Url::parse("https://localhost:3000")?;
    // Create client instance
    let client = Client::new(url);
    block_on(async {
        // Create docstore database
        client
            .create_db(
                "docstore-db",
                DatabaseType::DocStore { index_by: None },
                None,
                false,
            )
            .await?;
        // Create feed database
        client
            .create_db("feed-db", DatabaseType::Feed, None, false)
            .await?;
        // Get info for all databases
        let dbs = client.get_dbs().await?;
        dbg!(dbs);

        Ok(())
    })
}
