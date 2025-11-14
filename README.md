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
base62-uuid 2.2.4
```

```text
$ base62-uuid
6Vmhqf7gpAN3ighl8XRP9C
```

```text
$ base62-uuid -c 5 |tee temp
K5oXT16y0MHpmRSrd8AwW
3itOqzul7nohwaLL0IzTGd
1xFvM6RIQKEWH2xSiKVG39
1s3mXfvtbeRsZTqMx1LJfM
4OzCiDZVuKXmEgdpQv64xe
```

```text
$ base62-uuid -d <temp |tee temp-decoded
0aa6109f-b59f-4a79-8c86-cff5bc924b64
7a5ca40f-75f9-4e4a-bd03-38605d377473
40428e56-256f-4a19-a496-6309f4c2224b
3d81a6d8-0cf4-455a-bc02-9d5cbfbfd4d8
90ab5b2a-ac82-4e04-83eb-84cfd0461be2
```

```text
$ base62-uuid -e <temp-decoded
K5oXT16y0MHpmRSrd8AwW
3itOqzul7nohwaLL0IzTGd
1xFvM6RIQKEWH2xSiKVG39
1s3mXfvtbeRsZTqMx1LJfM
4OzCiDZVuKXmEgdpQv64xe
```

```text
$ base62-uuid -u
157407396918793612345847058091214039179
```

```text
$ base62-uuid -uc 5
25726618875764647792772774522608328155
20738513410792187987615174959815028916
329452107775353825701765254106518541965
225916323110276552616847510420115463706
135922767871533736486893140031504287238
```

```text
$ base62-uuid -puc 5
312225004194716591315589648095127808750
282371425671130271663048757181686492574
189961662930602166833978530417449472272
003702940395260938853001456856541204600
167528746828106585828975203482742515586
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

