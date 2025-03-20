use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn create_pool() -> Result<ConnectionPool, Box<dyn std::error::Error>> {
    // Create the connection manager
    let manager = PostgresConnectionManager::new_from_stringlike(
        load_db_config()?,
        NoTls)?;

    let pool = Pool::builder().build(manager).await?;
    Ok(pool)
}

pub async fn get_conn(
    pool: &ConnectionPool,
) -> Result<PooledConnection<'_, PostgresConnectionManager<NoTls>>, Box<dyn std::error::Error>> {
    let conn = pool.get().await?;
    Ok(conn)
}

fn load_db_config() -> Result<String, Box<dyn std::error::Error>> {
    let host = std::env::var("DATABASE_HOST")?;
    let port = std::env::var("DATABASE_PORT")?;
    let dbname = std::env::var("DATABASE_NAME")?;
    let user = std::env::var("DATABASE_USER")?;
    let password = std::env::var("DATABASE_PASSWORD")?;

    Ok(format!(
        "host={} port={} dbname={} user={} password={}",
        host, port, dbname, user, password
    ))
}