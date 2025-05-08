use std::path::PathBuf;

use clap::Parser;

//cargo run -- --help
#[derive(Parser, Debug)]
#[command(name = "Aginisi", version, about = "Fast JSON-Backed Mock API Server")]
pub struct Args {
    #[arg(
        long,
        default_value = ".",
        help = "Specify the path to serve files from"
    )]
    pub path: PathBuf,

    #[arg(
        short,
        long,
        default_value_t = 8090,
        help = "Port number to bind the server"
    )]
    pub port: u16,

    #[arg(short, long, default_value_t = false, help = "docs or how to use")]
    pub docs: bool,
}
