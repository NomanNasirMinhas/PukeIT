mod sql_injection;
use reqwest;
use clap::Parser;
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
    sql_injection::do_throttled_request(args.url.as_str());
}