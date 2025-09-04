use axum::Router;
use sqlx::PgPool;
mod todo;
mod share_mod;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = dotenvy::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    // 合并多个模块的路由
    let app = Router::new()
        .nest("/todos", todo::todo_routes())
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}