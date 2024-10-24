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
base62-uuid 2.1.0
```

```text
$ base62-uuid
tTO7UXTILC2AxQKVfhYzY
```

```text
$ base62-uuid -c 5 |tee temp
93a4HCalhVgXvozMQRDr8
25C7tS3B328zS834bz8uKV
6oTb2BzS78IkwZ5KoIFICl
qiqFIXOgI1kH1By16pEQ8
6iTib0KGX7T2Ufzy96sr09
```

```text
$ base62-uuid -d <temp |tee temp-decoded
04ccd736-750d-4267-9227-c775e4ff045a
447794f9-3c0b-4eef-adc3-458d0dd7f447
dfe43ed0-bb89-44c0-b014-30344d7ada47
1bf0cf03-bd86-40da-9011-f0e64645fcb4
dcb68298-8985-42ed-b569-6b308a943ced
```

```text
$ base62-uuid -e <temp-decoded
93a4HCalhVgXvozMQRDr8
25C7tS3B328zS834bz8uKV
6oTb2BzS78IkwZ5KoIFICl
qiqFIXOgI1kH1By16pEQ8
6iTib0KGX7T2Ufzy96sr09
```

```text
$ base62-uuid -u
334097669669744452419346251695304589440
```

```text
$ base62-uuid -uc 5
102389129306729026457559545738165059053
123439036126540706879880865806094135301
47167294684750739628554038748303803555
109767566098941365192457432125994458840
96887169780596865642054468111915816866
```

```text
$ base62-uuid -puc 5
128261614896343900523819685522305651328
263761526135930347639857319273958851714
303028256450039093740502152749362106721
190922064000881212597367172505050002457
148939742810114933100997314791914918665
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

