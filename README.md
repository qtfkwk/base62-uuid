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
base62-uuid 2.2.1
```

```text
$ base62-uuid
247RV8c2ZgxivfFBsE2W8S
```

```text
$ base62-uuid -c 5 |tee temp
1DC94xjMQb6qOxA7btFsDG
61qMFokPmmHZUgXH4EIgvP
2EuKhAAlFfO9M3o1fk6ZMw
33bnNBTmR7wnuSaFDBgEMq
3J7loH5ohggA7bdoVecydy
```

```text
$ base62-uuid -d <temp |tee temp-decoded
27d99ad8-41a2-4261-8b88-95537a5b2d26
c61e5e34-a0f3-468b-8694-57ccd372c01f
499d5114-3765-4f53-84f8-ee64ce7a68ea
647bc575-d0db-4460-9680-70bb8370d140
6cb4be3c-00af-4b69-b7e2-53841fb9820e
```

```text
$ base62-uuid -e <temp-decoded
1DC94xjMQb6qOxA7btFsDG
61qMFokPmmHZUgXH4EIgvP
2EuKhAAlFfO9M3o1fk6ZMw
33bnNBTmR7wnuSaFDBgEMq
3J7loH5ohggA7bdoVecydy
```

```text
$ base62-uuid -u
69089617929897512512571285136830376967
```

```text
$ base62-uuid -uc 5
182026059137579074983402627691910764318
31306162247594004845708377933453306332
301501013578469709782775885669662216506
236456772086291494159534087538288742503
197901093192396415446677969986934925063
```

```text
$ base62-uuid -puc 5
051717229333849816523356379951611840876
230952278713017380170141044763118660298
144946706414763915261299590366465688011
283586503043110628650755270459387800391
260252246961985818225573018669773436157
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

