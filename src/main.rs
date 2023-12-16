mod errors;
mod cli;
mod server;

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

    server::start(([0,0,0,0], args.port)).await;
}
