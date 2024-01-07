use clap::Parser;

use crate::errors::Errcode;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Activate debug mode
    // short and long flags, (-d, --debug) derived from the field name
    #[arg(short, long)]
    debug: bool,

    /// The port to run on
    #[arg(short, long, default_value_t = 8888)]
    pub port: u16,
}

pub fn parse_args() -> Result<Args, Errcode> {
    let args = Args::parse();
    Ok(args)
}
