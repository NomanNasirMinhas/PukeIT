use clap::Parser;
use std::path::Path;
use pelite::{FileMap, Result};
use pelite::pe64::{Pe, PeFile};

/// Rust-Based Web Application Security Tester
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL of the web application to test
    #[arg(short, long)]
    url: String,

}

fn main()
{
    let args = Args::parse();
    println!("Scanning URL {}", args.url);

}