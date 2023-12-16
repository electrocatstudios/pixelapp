extern crate sqlx;

mod errors;
mod cli;
mod server;
mod db;

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
    let mut pool = db::get_conn().await.unwrap();

    server::start(([0,0,0,0], args.port), &mut pool).await;
}
