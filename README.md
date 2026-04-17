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
base62-uuid 2.2.6
```

```text
$ base62-uuid
6jBPQHphUOIHkUf3c6krK5
```

```text
$ base62-uuid -c 5 |tee temp
4XAd9JkbCrPFUC8S5YftoM
15DaBHwIo9LF8OMQKG8Jiw
o74NZpZmw2TXn4N8d0vft
5s2VFXVPeVtQDtg1tHzzEe
oggJ8u6R11ODaYR9FjLGX
```

```text
$ base62-uuid -d <temp |tee temp-decoded
9501b1fc-7ad9-44b4-9b6e-87f2a1abcc46
239f69d6-0779-48e1-bed4-fa90b3ec75ee
1a8ed29b-4c4b-451b-8928-44e1e58c3239
c0ec45ff-cce1-4446-b008-71d60b538308
1adcbf5d-5c66-4a42-a786-d32ab866387d
```

```text
$ base62-uuid -e <temp-decoded
4XAd9JkbCrPFUC8S5YftoM
15DaBHwIo9LF8OMQKG8Jiw
o74NZpZmw2TXn4N8d0vft
5s2VFXVPeVtQDtg1tHzzEe
oggJ8u6R11ODaYR9FjLGX
```

```text
$ base62-uuid -u
173486006473405760866936702403733042076
```

```text
$ base62-uuid -uc 5
337012885788109882594929689051408649853
83381442489949285299222262381378785511
321904454328088423983258979670076213722
266526673478365653799107572431835602284
141826536554254091048512929624885674552
```

```text
$ base62-uuid -puc 5
168768175045614836074071965646814900034
006962970173046767957765724883577280814
282117893058402895209799202643482238562
121896480845731165582319399399260214528
110981028096359351826341735853103640821
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

