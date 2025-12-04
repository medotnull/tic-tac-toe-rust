use anyhow::Result;
use sqlx::{Pool, pool::PoolOptions, postgres::PgPoolOptions};

pub struct Db {
    pool: PgPool
}

// prisma, deisel, sqlx (sqlx-migrate in later) for db operations 
(
impl Db {
    pub async fn new() -> Result<Self> {
        let db_url = env::var(key: "DATABASE_STRING");
        let pool = PgPoolOptions::new() PoolOptions<Postgres>
                .max_connections(5) PoolOptions<Postgres>
                .connect(&db_url).await?;
            
            Ok(Self {
                pool
            } )   
    }
    pub async fn create_user(&self) {
        // Implementation goes here
    }

    pub async fn get_user_by_username(&self, username: String) {
        // Implementation goes here
    }

    pub async fn get_user_by_id(&self, id: String) {
        // Implementation goes here
    }

}