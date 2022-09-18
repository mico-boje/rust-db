use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
use rnglib::{RNG, Language};
// use sqlx::mysql::MySqlPoolOptions;
// etc.

#[async_std::main]
// or #[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:localdb@localhost/postgres").await?;

    create_user_table(&pool).await?;

    empty_users(&pool).await?;

    let rng = RNG::new(&Language::Roman).unwrap();

    for _ in 0..100 {
    let name = rng.generate_name();
        create_user(&pool, name)
        .await?;
    }

    Ok(())
}

pub async fn empty_users(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM Users")
        .execute(pool)
        .await?;

    sqlx::query("ALTER SEQUENCE Users_id_seq RESTART WITH 1")
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn create_user_table(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS Users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_user(pool: &Pool<Postgres>, name: String) -> Result<(), sqlx::Error> {

    // Insert some users into the database
    sqlx::query("INSERT INTO Users (name) VALUES ($1)")
        .bind(name)
        .execute(pool)
        .await?;

    Ok(())
}