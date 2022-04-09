use clap::Parser;
use base62_uuid::base62_uuid;

/// Base62 UUID
#[derive(Parser)]
#[clap(name = "base62-uuid")]
struct Args {
    /// Number of Base62 UUIDs to generate
    #[clap(short, long, default_value_t = 1)]
    count: usize,
}

/// Command line interface
fn main() -> Result<(), String> {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("{}", base62_uuid());
    }
    Ok(())
}
