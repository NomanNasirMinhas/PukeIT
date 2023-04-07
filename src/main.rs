use clap::Parser;

/// Rust-Based Reverse Engineering Tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Binary to Reverse Engineer
    #[arg(short, long)]
    binary: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

fn main() {
    let args = Args::parse();


        println!("Hello {}!", args.binary)

}