use sqlx::PgPool;
use std::env;
use dotenvy::dotenv;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;
    Ok(pool)
}
