
use sqlx::{PgPool, prelude::FromRow};
#[derive(Debug, FromRow)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}
struct CreateTodo {
    title: String,
    done: bool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = dotenvy::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    let todo = CreateTodo {
        title: "learn sqlx4".to_string(),
        done: false,
    };

    sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (title, done)
        VALUES ($1, $2)
        RETURNING id, title, done, created_at
        "#,
        todo.title,
        todo.done,
    )
        .fetch_one(&pool)
        .await?;

    let all = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, done, created_at
        FROM todos
        "#,
    )
        .fetch_all(&pool)
        .await?;

    dbg!(all);

    Ok(())
}