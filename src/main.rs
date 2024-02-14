extern crate sqlx;
use web_view::*;
use tokio::sync::oneshot;

mod errors;
mod cli;
mod server;
mod db;
mod utils;
mod image; 


async fn run_server(port: u16) {

    let pool = match db::get_db_filter().await {
        Ok(db_pool) => db_pool,
        Err(err) => {
            log::error!("Error connecting to db: {}", err);
            
            return;
        }
    };

    server::start(([0,0,0,0], port), &mut pool.clone()).await;
    log::info!("Finishing server start");
}

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

    // let (tx, rx) = oneshot::channel<bool>();
    // thread::spawn(move || run_server(args.port));
    // tokio::spawn(run_server(rx, args.port));
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn(run_server(args.port));

    web_view::builder()
        .title("Pixel App")
        .content(Content::Url(format!("http://localhost:{}", args.port)))
        .size(600, 500)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
