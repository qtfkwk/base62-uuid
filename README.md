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
base62-uuid 2.1.2
```

```text
$ base62-uuid
3laM5kPICgyP7F1768yCaT
```

```text
$ base62-uuid -c 5 |tee temp
6NufJxvYb12RZMqwvx85aV
71HGe9h1dXZRiXm1HHe5xI
1D8H5DnL5ndJRAu70slwhS
4Wd4A1pT6ZgrLcj5HH5Q9R
1Nh8S7crLM05QqXrmcQepz
```

```text
$ base62-uuid -d <temp |tee temp-decoded
d1d077cf-248a-430e-a00d-f3d66289b4bb
e6acf05c-985c-4103-8a30-ea11bdd2e060
27d12278-1098-48ba-8f4d-fb7af76f9e16
94b84023-433f-48fa-a4f8-2d4266f69d39
2d6a1620-4405-4440-ab67-790c1be38f87
```

```text
$ base62-uuid -e <temp-decoded
6NufJxvYb12RZMqwvx85aV
71HGe9h1dXZRiXm1HHe5xI
1D8H5DnL5ndJRAu70slwhS
4Wd4A1pT6ZgrLcj5HH5Q9R
1Nh8S7crLM05QqXrmcQepz
```

```text
$ base62-uuid -u
334192065423925568041916124721190537442
```

```text
$ base62-uuid -uc 5
153037776052740855318205720407277046638
142310672101693990170550766039982545299
35776981637915368734461781696356706436
133118973068825747634739805701820992127
223576874598144120164987935439909393048
```

```text
$ base62-uuid -puc 5
253426329562873622538263703838148629133
168936522868221790808201843416965754237
152722512667824118025585819570923031743
137445247870291291923632416101302920106
052819853576699207033140099703351549973
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

