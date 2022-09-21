use sqlx::{query_file_as, PgPool, FromRow};
use std::fmt::{Display, Formatter};

#[derive(FromRow)]
struct Users {
    id: i32,
    name: String,
}

impl Display for Users {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
            id: {},
            name: {},
        "#,
            self.id, self.name
        )
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?)
        .await?;


    // we can also use `query_file_as!()` similarly to `query_as!()` to map our database models
    let users = query_file_as!(Users, "queries/select_all.sql")
        .fetch_all(&pool)
        .await?;

    for user in users {
        println!("{}", user);
    }

    Ok(())
}