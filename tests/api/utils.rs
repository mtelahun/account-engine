use std::time::{SystemTime, UNIX_EPOCH};

use chrono::NaiveDateTime;
use tokio_postgres::NoTls;

pub fn timestamp() -> NaiveDateTime {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before Unix epoch")
        .as_micros();
    let naive = NaiveDateTime::from_timestamp_micros(now.try_into().unwrap()).unwrap();

    naive
}

pub async fn connect_to_db(db_name: &str) -> tokio_postgres::Client {
    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host=localhost user=postgres password=password dbname={}",
            db_name
        ),
        NoTls,
    )
    .await
    .expect("Unable to connect to test database");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e)
        }
    });

    client
}
