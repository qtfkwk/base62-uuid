use base62_uuid::{base62_uuid, decode, decode_u128, encode, encode_u128, u128_uuid};
use clap::Parser;
use std::io::stdin;

/// Base62 UUID
#[derive(Parser)]
struct Args {
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
    let args = Args::parse();

    if args.decode && args.encode {
        return Err(String::from(
            "Decode and encode options are mutually exclusive.",
        ));
    }

    if args.decode {
        let mut line = String::new();
        while stdin().read_line(&mut line).is_ok() {
            let s = line.trim();
            if s.is_empty() {
                break;
            }
            if args.u {
                println!("{}", decode_u128(s));
            } else {
                println!("{}", decode(s));
            }
            line = String::new();
        }
    } else if args.encode {
        let mut line = String::new();
        while stdin().read_line(&mut line).is_ok() {
            let s = line.trim();
            if s.is_empty() {
                break;
            }
            if args.u {
                println!("{}", encode_u128(s, args.pad));
            } else {
                println!("{}", encode(s, args.pad));
            }
            line = String::new();
        }
    } else if args.u {
        for _ in 0..args.count {
            println!("{}", u128_uuid(args.pad));
        }
    } else {
        for _ in 0..args.count {
            println!("{}", base62_uuid(args.pad));
        }
    }

    Ok(())
}
