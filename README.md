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
base62-uuid 2.2.3
```

```text
$ base62-uuid
7W3bcywYYdHXTVGu2PFDI9
```

```text
$ base62-uuid -c 5 |tee temp
4GoxRZFtDTjdj4BdKIWgQl
4iOeXXs5ZE5zuTxAvhvnRd
1bxFkPJJKdK7GEE0rf23q3
2cihE1LT0i2yM17y0LVLDA
4AH4v9kO9pkranqLnemHMR
```

```text
$ base62-uuid -d <temp |tee temp-decoded
8c57996d-d32e-4b06-9c2c-ff6b489eed03
9af4b74c-e169-4fff-906b-14c47f1dd53d
34f8b166-807c-49d7-8bbb-162a85cb8a87
563bdcb5-51a6-4dd4-88f9-0056fc02645c
88df768c-b882-4a9e-8e11-34c0844b3bd3
```

```text
$ base62-uuid -e <temp-decoded
4GoxRZFtDTjdj4BdKIWgQl
4iOeXXs5ZE5zuTxAvhvnRd
1bxFkPJJKdK7GEE0rf23q3
2cihE1LT0i2yM17y0LVLDA
4AH4v9kO9pkranqLnemHMR
```

```text
$ base62-uuid -u
110698649437569065737455570577095525029
```

```text
$ base62-uuid -uc 5
238981546592484542129939676544627163213
92596523074198712145824399895334562337
200608507005442482622814881611402837488
70578660453805861253225006625419089649
234034593625296001670957240782579348338
```

```text
$ base62-uuid -puc 5
013629594431662970953936786076512722817
058711434539551638323807936785206326275
275816263869174499895391681088580423596
126088002708095562018558866447833246417
077706646678184456579948564052954064991
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

