// This module will be in charge of interacting with a PostgreQSL database
use sqlx::PgPool;
use sqlx::pool::PoolOptions;
use sqlx::ConnectOptions;
use sqlx::postgres::PgConnectOptions;
use std::time::Duration;


pub async fn connect() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to Database");

    let mut connectOptions = PgConnectOptions::new();
    connectOptions.host(&"localhost");
    let mut pool : PgPool =  PoolOptions::new()
        .connect_timeout(Duration::from_secs(5))
        .connect("postgres://postgres:welcome@localhost")
        .await?;

    match pool {
        Ok(pool) => println!("Connected"),
        Err(e) => println!("Database connection failed: {e}")
    }
    Ok(())
}