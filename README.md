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
base62-uuid 2.2.2
```

```text
$ base62-uuid
6GMTs0Ri8TbGxT1CDs4bta
```

```text
$ base62-uuid -c 5 |tee temp
47QgRgI8O9eQrUQsvPoPsZ
5bVvbFztRS7e3DbIB1XYrb
7idwxdNNsA3kuFN6avmUQN
4XYob0Ufdc9wUTAgSKtb03
3EOs7sT5iGFjg4SzpDKfuh
```

```text
$ base62-uuid -d <temp |tee temp-decoded
875d7ae1-0374-4098-b876-961414336bfb
b82a523f-ce76-4542-9f34-547f5a87f12b
fda84102-de12-41b7-a796-e2206807236b
95369d92-18d9-4526-94d1-b517a407031f
6a33d478-1623-4ec7-96da-b470a40bbc6f
```

```text
$ base62-uuid -e <temp-decoded
47QgRgI8O9eQrUQsvPoPsZ
5bVvbFztRS7e3DbIB1XYrb
7idwxdNNsA3kuFN6avmUQN
4XYob0Ufdc9wUTAgSKtb03
3EOs7sT5iGFjg4SzpDKfuh
```

```text
$ base62-uuid -u
268249471152177956793635011932537642945
```

```text
$ base62-uuid -uc 5
241434660040576739223539145393930293036
200001631279632594002330697595834007582
21566672162428949653686364174583895751
42705697181726104543318613845314642521
296070657833688421935982006384672283225
```

```text
$ base62-uuid -puc 5
018910419223512109115460924494095229811
289331023564481469176340286313807493402
029155980975695544420851152801243455341
093149930625528024458912222307367893902
255830530399136013876633733107953306757
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

