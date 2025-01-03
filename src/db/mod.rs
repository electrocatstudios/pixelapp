use sqlx::{Pool, migrate::MigrateDatabase};
use sqlx_sqlite::{Sqlite,SqlitePool};
use warp::filters::BoxedFilter;
use warp::Filter;

use std::fmt;

pub mod models;
pub mod animation_models;
pub mod video_models;

pub mod queries;
pub mod animation_queries;
pub mod video_queries;

const DB_CONN_STRING: &str = "sqlite://pixel.db";

pub enum DBError {
    DatabaseError(String),
    UnknownError(String),
    AlreadyExists(String),
    NoneFound
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DBError::DatabaseError(err_str) => write!(f, "DatabaseError: {}", err_str),
            DBError::UnknownError(err_str) => write!(f, "UnknownError: {}", err_str),
            DBError::AlreadyExists(err_str) => write!(f, "Already Exists: {}", err_str),
            DBError::NoneFound => write!(f, "No result found")
        }
    }
}

pub async fn get_db_filter() -> Result<BoxedFilter<(Pool<Sqlite>,)>, Box<dyn std::error::Error>> {
    if !Sqlite::database_exists(&DB_CONN_STRING).await.unwrap_or(false) {
        Sqlite::create_database(&DB_CONN_STRING).await.unwrap();
        log::info!("Database created");
    } else {
        log::info!("Database exists, skipping creation");
    }

    let pool = SqlitePool::connect(DB_CONN_STRING).await?;
    
    let res = sqlx::migrate!("./migrations")
        .run(&pool)
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

    Ok(warp::any().map(move || pool.clone()).boxed())
}