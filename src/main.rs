use sqlx::sqlite::SqliteConnectOptions;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    id: i64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let mut opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
    opts = opts.shared_cache(false);

    let pool = sqlx::SqlitePool::connect_with(opts).await.unwrap();

    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS users(id int, name varchar(128), email varchar(128));",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    // Comment in to see that no db is created.
    // let id = 42;
    // let name = "ferris";
    // let email = "foo@bar.com";

    // let _ = sqlx::query("INSERT INTO users(id, name, email) VALUES (?, ?, ?);")
    //     .bind(id)
    //     .bind(name)
    //     .bind(email)
    //     .execute(&pool)
    //     .await
    //     .expect("Failed to insert user");

    // let ferris = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
    //     .bind(id)
    //     .fetch_one(&pool)
    //     .await
    //     .unwrap();

    // let res = format!("Created and retrieved {:?}", ferris);
    // println!("{res}");
}
