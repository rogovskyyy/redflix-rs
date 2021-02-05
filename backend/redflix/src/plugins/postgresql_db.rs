extern crate tokio;
extern crate tokio_postgres;
extern crate postgres_query;

#[allow(unused_imports)]
use tokio::*;
#[warn(unused_imports)]
use tokio_postgres::{ NoTls, Error };
use std::result::{ Result };

pub struct PostgreSQLEngine { }

#[allow(dead_code)]
impl PostgreSQLEngine {
    
    pub async fn run() -> Result<tokio_postgres::Client, Error> {
        let (client, connection) =
            tokio_postgres::config::Config::new()
                        .host("127.0.0.1")
                        .user("postgresql")
                        .password("Admin123!")
                        .dbname("blueflixdb")
                        .port(5432)
                        .connect(NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(client)
    }

    pub async fn execute(query: &'static str, binding: Vec<String>) -> std::result::Result<tokio_postgres::RowStream, tokio_postgres::Error> {
        let client: tokio_postgres::Client = match PostgreSQLEngine::run().await {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        let result = client.query_raw(query, binding).await?;
        Ok(result)
    }
}