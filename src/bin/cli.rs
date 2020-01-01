use async_std::task::block_on;
use orbit_db_http_client::{Client, DatabaseType, Query};
use structopt::StructOpt;

/// A client written in Rust for OrbitDB's REST server
#[derive(StructOpt)]
#[structopt(name = "OrbitDB HTTP Client", version = "0.1", author = "Austin Baugh")]
struct CmdLine {
    /// OrbitDB REST API url
    #[structopt(short, long)]
    server_url: url::Url,

    /// Command for client to communicate with REST API
    #[structopt(subcommand)]
    cmd: Command,
}

/// Commands corresponding to REST API requests
#[derive(StructOpt)]
enum Command {
    /// Gets the information of the different databases
    GetDbs,
    /// Gets the information of a specific database
    GetDb { dbname: String },
    /// Gets counter database value
    GetCounterValue { dbname: String },
    /// Gets the record identified by `item` within the given db
    GetDbItem { dbname: String, item: String },
    /// Gets a possibly limited number of items from an EventLog or Feed
    GetDbIterator { dbname: String, limit: Option<i64> },
    /// Gets the database information
    GetDbIndex { dbname: String },
    /// Gets REST API identity information
    GetIdentity,
    /// Creates a database with the given name and type
    /// TODO: add optional arguments
    CreateDb {
        dbname: String,
        dbtype: DatabaseType,
    },
    /// Adds a record to the specified database
    DbPut {
        dbname: String,
        record: serde_json::Value,
    },
    /// Applies a query to the specified database
    /// TODO: add query argument
    DbQuery { dbname: String },
}

fn main() -> Result<(), surf::Exception> {
    // Get the arguments passed through the command line
    let args = CmdLine::from_args();
    // Create client instance with provided server url
    let client = Client::new(args.server_url.clone());
    // Make HTTP requests
    block_on(async {
        // Run corresponding client method based on provided subcommand
        match args.cmd {
            Command::GetDbs => {
                let dbs = client.get_dbs().await?;
                dbg!(dbs);
            }
            Command::GetDb { dbname } => {
                let db = client.get_db(&dbname).await?;
                dbg!(db);
            }
            Command::GetCounterValue { dbname } => {
                let value = client.get_counter_value(&dbname).await?;
                dbg!(value);
            }
            Command::GetDbItem { dbname, item } => {
                let value = client.get_db_item(&dbname, &item).await?;
                dbg!(value);
            }
            Command::GetDbIterator { dbname, limit } => {
                let value = client.get_db_iterator(&dbname, limit).await?;
                dbg!(value);
            }
            Command::GetDbIndex { dbname } => {
                let value = client.get_db_index(&dbname).await?;
                dbg!(value);
            }
            Command::GetIdentity => {
                let identity = client.get_identity().await?;
                dbg!(identity);
            }
            Command::CreateDb { dbname, dbtype } => {
                let value = client.create_db(&dbname, dbtype, None, false).await?;
                dbg!(value);
            }
            Command::DbPut { dbname, record } => {
                let hash = client.db_put(&dbname, record).await?;
                dbg!(hash);
            }
            Command::DbQuery { dbname } => {
                let query = Query {
                    propname: None,
                    comp: None,
                    values: vec![]
                };

                let items = client.db_query(&dbname, query).await?;
                dbg!(items);
            }
        }
        Ok(())
    })
}
