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

    match &args.cmd.as_str() {
        &"version"  => println!("ProductSpring CLI version 0.0.1"),
        &"login"    => println!("Loggging you in..."),
        &"org"      => println!("Organization..."),
        &"projects" => pritnln!("Projects..."),
        &_ => println!("Unknown command.")
    }

    Ok(())
}
