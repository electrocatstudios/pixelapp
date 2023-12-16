use sqlx::{Pool,Error, migrate::MigrateDatabase};
use sqlx_sqlite::{Sqlite,SqlitePool};

const DB_CONN_STRING: &str = "sqlite://pixel.db";

pub async fn get_conn() -> Result<Pool<Sqlite>, Error> {
    if !Sqlite::database_exists(&DB_CONN_STRING).await.unwrap_or(false) {
        Sqlite::create_database(&DB_CONN_STRING).await.unwrap();
        log::info!("Database created");
    } else {
        log::info!("Database exists, skipping creation");
    }

    let db = SqlitePool::connect(DB_CONN_STRING).await?;
    
    let res = sqlx::migrate!("./migrations")
        .run(&db)
        .await;

    match res {
        Ok(_) => {
            log::info!("Migration complete");
        },
        Err(err) => {
            log::error!("{}", err.to_string());
            panic!("Failed during migration");
        }
    }

    Ok(db)
}