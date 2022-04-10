use clap::Parser;
use base62_uuid::{base62_uuid, decode, encode};
use std::io::stdin;

/// Base62 UUID
#[derive(Parser)]
#[clap(name = "base62-uuid")]
struct Args {
    /// Number of Base62 UUIDs to generate
    #[clap(short, long, default_value_t = 1)]
    count: usize,

    /// Decode Base62 UUIDs to standard UUIDs
    #[clap(short, long)]
    decode: bool,

    /// Encode standard UUIDs to Base62 UUIDs
    #[clap(short, long)]
    encode: bool,
}

/// Command line interface
fn main() -> Result<(), String> {
    let args = Args::parse();

    if args.decode && args.encode {
        return Err(String::from("Decode and encode options are mutually exclusive."));
    }

    if args.decode {
        let mut line = String::new();
        while stdin().read_line(&mut line).is_ok() {
            let s = line.trim();
            if s == "" {
                break;
            }
            println!("{}", decode(&s));
            line = String::new();
        }
    } else if args.encode {
        let mut line = String::new();
        while stdin().read_line(&mut line).is_ok() {
            let s = line.trim();
            if s == "" {
                break;
            }
            println!("{}", encode(&s));
            line = String::new();
        }
    } else {
        for _ in 0..args.count {
            println!("{}", base62_uuid());
        }
    }

    Ok(())
}
