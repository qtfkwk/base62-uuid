# About

Minimalist crate to generate Base62 UUIDs

[GitHub](https://github.com/qtfkwk/base62-uuid)
[Crates.io](https://crates.io/crates/base62-uuid)
[Lib.rs](https://lib.rs/crates/base62-uuid)
[Docs.rs](https://docs.rs/base62-uuid/latest/base62_uuid/)

# Command line utility

```text
$ base62-uuid -h
!run:../target/release/base62-uuid -h
```

```text
$ base62-uuid -V
!run:../target/release/base62-uuid -V
```

```text
$ base62-uuid
!run:../target/release/base62-uuid
```

```text
$ base62-uuid -c 5 |tee temp
!run:../target/release/base62-uuid -c 5 |tee temp
```

```text
$ base62-uuid -d <temp |tee temp-decoded
!run:../target/release/base62-uuid -d <temp |tee temp-decoded
```

```text
$ base62-uuid -e <temp-decoded
!run:../target/release/base62-uuid -e <temp-decoded
!run:rm -f temp temp-decoded
```

```text
$ base62-uuid -u
!run:../target/release/base62-uuid -u
```

```text
$ base62-uuid -uc 5
!run:../target/release/base62-uuid -uc 5
```

```text
$ base62-uuid -puc 5
!run:../target/release/base62-uuid -puc 5
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

