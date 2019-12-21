use async_std::task::block_on;
use orbit_db_http_client::{Client, DatabaseType};
use structopt::StructOpt;
// Using `anyhow!` to show error descriptions for now
//
// TODO: remove use of anyhow when https://github.com/http-rs/surf/pull/113 is
// implemented, then `surf::Error` and `?` will be used instead
use anyhow::{anyhow, Error};

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
    /// Gets REST API identity information
    Identity,
    /// Gets counter database value
    CounterValue { dbname: String },
    /// Creates a database with the given name and type
    CreateDb {
        dbname: String,
        dbtype: DatabaseType,
    },
}

fn main() -> Result<(), Error> {
    // Get the arguments passed through the command line
    let args = CmdLine::from_args();
    // Create client instance with provided server url
    let client = Client::new(args.server_url.clone());
    // Make HTTP requests
    block_on(async {
        // Run corresponding client method based on provided subcommand
        match args.cmd {
            Command::Identity => {
                let identity = match client.get_identity().await {
                    Ok(id) => id,
                    Err(e) => return Err(anyhow!(e)),
                };
                dbg!(identity);
            }
            Command::CounterValue { dbname } => {
                let value = match client.get_counter_value(&dbname).await {
                    Ok(val) => val,
                    Err(e) => return Err(anyhow!(e)),
                };
                dbg!(value);
            }
            Command::CreateDb { dbname, dbtype } => {
                let value = match client.create_database(&dbname, dbtype).await {
                    Ok(val) => val,
                    Err(e) => return Err(anyhow!(e)),
                };
                dbg!(value);
            }
        }
        Ok(())
    })
}
