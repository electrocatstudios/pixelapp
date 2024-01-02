extern crate sqlx;

mod errors;
mod cli;
mod server;
mod db;
mod utils;

#[tokio::main]
async fn main() {
    let args = match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            args
        },
        Err(e) => {
            log::debug!("Error while parsing args:\n\t{}", e);
            panic!("Failed to launch web-server");
        }
    };
    
    pretty_env_logger::init();
    let pool = match db::get_db_filter().await {
        Ok(db_pool) => db_pool,
        Err(err) => {
            log::error!("Error connecting to db: {}", err);
            
            return;
        }
    };

    server::start(([0,0,0,0], args.port), &mut pool.clone()).await;
}
