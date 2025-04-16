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
base62-uuid 2.1.3
```

```text
$ base62-uuid
1vD7bVfThmprWisZ2WBXFk
```

```text
$ base62-uuid -c 5 |tee temp
15zRhGy1uI08DOvE2To3ee
1sPJ0IPskJ46LRyySaBFDa
6pbSiX5Sl8idfo7Fm6ab7W
4a7V7v7hgCfMEit3AJmkVV
1V5ld9JaQHq4KnnhtAFOHK
```

```text
$ base62-uuid -d <temp |tee temp-decoded
24082582-4d1f-4305-8338-82153f8cc1e4
3db0bfc7-a242-4c8d-b2b2-5e8134f37dee
e07d1fd0-6f3d-416c-9b20-b2764e535ee6
9691d940-2ccc-4b5c-a35e-1875375bbe09
3155a78e-e4f0-4032-b5e8-c6957f0f0a1a
```

```text
$ base62-uuid -e <temp-decoded
15zRhGy1uI08DOvE2To3ee
1sPJ0IPskJ46LRyySaBFDa
6pbSiX5Sl8idfo7Fm6ab7W
4a7V7v7hgCfMEit3AJmkVV
1V5ld9JaQHq4KnnhtAFOHK
```

```text
$ base62-uuid -u
195780859599194812538309117777803253298
```

```text
$ base62-uuid -uc 5
322311959642245423184227077805366235107
265516811764878654142132925478597631421
152642501104823714159104579109785730225
14883117085937578436974869787647098382
67987783208808059818718474367564554160
```

```text
$ base62-uuid -puc 5
191004200075747999183603018274424402321
234884124756555431401064403824103030636
332676006316932669987488181688305026883
215770725021832442542265288882336841622
235602559437145661596221636035013474584
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

