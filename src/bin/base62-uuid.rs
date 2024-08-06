use base62_uuid::{base62_uuid, decode, decode_u128, encode, encode_u128, u128_uuid};
use clap::Parser;
use std::io::stdin;

/// Base62 UUID
#[derive(Parser)]
#[command(version, term_width = 80)]
struct Cli {
    /// Number of Base62 UUIDs to generate
    #[clap(short, long, default_value_t = 1)]
    count: usize,

    /// Decode Base62 UUIDs from STDIN to standard UUIDs
    #[clap(short, long)]
    decode: bool,

    /// Encode standard UUIDs from STDIN to Base62 UUIDs
    #[clap(short, long)]
    encode: bool,

    /// Pad UUIDs with leading zeroes
    #[clap(short, long)]
    pad: bool,

    /// Generate/encode/decode u128 UUIDs instead of Base62 UUIDs
    #[clap(short)]
    u: bool,
}

/// Command line interface
fn main() -> Result<(), String> {
    let cli = Cli::parse();

    if cli.decode && cli.encode {
        return Err(String::from(
            "Decode and encode options are mutually exclusive.",
        ));
    }

    if cli.decode {
        let mut line = String::new();
        while stdin().read_line(&mut line).is_ok() {
            let s = line.trim();
            if s.is_empty() {
                break;
            }
            if cli.u {
                println!("{}", decode_u128(s));
            } else {
                println!("{}", decode(s));
            }
            line = String::new();
        }
    } else if cli.encode {
        let mut line = String::new();
        while stdin().read_line(&mut line).is_ok() {
            let s = line.trim();
            if s.is_empty() {
                break;
            }
            if cli.u {
                println!("{}", encode_u128(s, cli.pad));
            } else {
                println!("{}", encode(s, cli.pad));
            }
            line = String::new();
        }
    } else if cli.u {
        for _ in 0..cli.count {
            println!("{}", u128_uuid(cli.pad));
        }
    } else {
        for _ in 0..cli.count {
            println!("{}", base62_uuid(cli.pad));
        }
    }

    Ok(())
}
