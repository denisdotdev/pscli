#![allow(unused)]

use clap::Parser;
use hyper::Client;

#[derive(Parser)]
struct Cli {
    cmd: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Cli::parse();

    if (&args.cmd == "version") {
        println!("ProductSpring CLI version 0.0.1"); // todo: reference the current git tag release
    }

    Ok(())
}
