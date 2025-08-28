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
base62-uuid 2.2.0
```

```text
$ base62-uuid
55YfT7LT6wvJgdA4AbFNIY
```

```text
$ base62-uuid -c 5 |tee temp
4mZhgkgLMAEPNlsSaXpvTF
6asXJXlDyk25abdUtaYDe1
2faJqyOI4xClOWMgvMsr2
6Gcn9gOATMgMOcDt0RMYMr
53lpU6YAOzvivyBxAFMl8L
```

```text
$ base62-uuid -d <temp |tee temp-decoded
9d2b90ae-48c0-4996-891f-bd9ca5ec3cf1
d8af7af3-d007-4529-b167-a45c8788fe35
016a54c7-f220-4be9-b47d-4fe73d8cded0
cdf3b02c-8f4a-4605-bdbb-51c8a8af5451
a648700e-b9fd-4b47-9020-e60cfd1aa001
```

```text
$ base62-uuid -e <temp-decoded
4mZhgkgLMAEPNlsSaXpvTF
6asXJXlDyk25abdUtaYDe1
2faJqyOI4xClOWMgvMsr2
6Gcn9gOATMgMOcDt0RMYMr
53lpU6YAOzvivyBxAFMl8L
```

```text
$ base62-uuid -u
232480195457168546014132775532846169924
```

```text
$ base62-uuid -uc 5
140471281257249528817589598838243267128
304477159215772330344667714450555089277
322508131350300957191390669337804649978
302348384318230714402993982731679280540
143173130970864163417362776299252244858
```

```text
$ base62-uuid -puc 5
147589676647192900593384419997360289756
285608226344560402994333615690752191802
242150064069015677856668869430180393980
295607583736598908898530096890709874498
292677704844092062836148043322963624272
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

