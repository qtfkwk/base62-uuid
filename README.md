# About

Minimalist crate to generate Base62 UUIDs

[GitHub](https://github.com/qtfkwk/base62-uuid)
[Crates.io](https://crates.io/crates/base62-uuid)
[Lib.rs](https://lib.rs/crates/base62-uuid)
[Docs.rs](https://docs.rs/base62-uuid/latest/base62_uuid/)

# Command line utility

```text
$ base62-uuid -h
Base62 UUID

Usage: base62-uuid [OPTIONS]

Options:
  -c, --count <COUNT>  Number of Base62 UUIDs to generate [default: 1]
  -d, --decode         Decode Base62 UUIDs from STDIN to standard UUIDs
  -e, --encode         Encode standard UUIDs from STDIN to Base62 UUIDs
  -p, --pad            Pad UUIDs with leading zeroes
  -u                   Generate/encode/decode u128 UUIDs instead of Base62 UUIDs
  -h, --help           Print help
  -V, --version        Print version
```

```text
$ base62-uuid -V
base62-uuid 2.2.5
```

```text
$ base62-uuid
1aUGn5qlPAXzhBWMytAWXO
```

```text
$ base62-uuid -c 5 |tee temp
2xupRLzaSX76TqA5ID3clG
1qnpMdepqA7RsbYhCTPowo
4YqYUoEXxecqUBggbxHzKT
iqTspGIsm1hKxZoywWJSr
6Vjt5CLLIj418VWAP1lJkS
```

```text
$ base62-uuid -d <temp |tee temp-decoded
61776ae5-74cf-40dd-aad3-dfa051ba43c2
3cd712cf-0253-4915-b52f-98c2f021864e
95e519fc-595a-4189-ac21-ab96128abaf1
17c4304e-50b2-4b3f-83f9-2e9d704bc169
d5f6384a-44ad-4d55-9c33-2d1bb65fd284
```

```text
$ base62-uuid -e <temp-decoded
2xupRLzaSX76TqA5ID3clG
1qnpMdepqA7RsbYhCTPowo
4YqYUoEXxecqUBggbxHzKT
iqTspGIsm1hKxZoywWJSr
6Vjt5CLLIj418VWAP1lJkS
```

```text
$ base62-uuid -u
101554627951797032970190861518671494473
```

```text
$ base62-uuid -uc 5
320321933757126548723643707359286408723
157295824476328908092048983571313750122
133110057582525554489809838180652883091
274168043867976659791770268987615338951
261763677303949830752343522991380248263
```

```text
$ base62-uuid -puc 5
114555319389950853767089051927318412958
240066976928020194054120994183064405148
110763369560043663778603143826153311109
202513619298374754753905232973245718736
107410098649442876268656714617057561833
```

# Library functions

```Rust
use base62_uuid::{base62_uuid, decode, encode};

let id = base62_uuid();

let id_decoded = decode(&id);
let id_encoded = encode(&id_decoded);

assert_eq!(id_encoded, id);
```

See also the [API documentation](https://docs.rs/base62-uuid/latest/base62_uuid/).

