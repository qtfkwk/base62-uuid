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
base62-uuid 2.1.1
```

```text
$ base62-uuid
4IBP3SS4fd8ML5P71Cdlra
```

```text
$ base62-uuid -c 5 |tee temp
5LAIPC40yAz4V86pRxXDc0
79FDau3ZCOIJIn1pGWFW4S
53LjP8t5ptC3em3e8Fo0oX
3VIdCgsZHFmKYKF83vZtmZ
6Uai97JrjIzsYbVNW9R7o2
```

```text
$ base62-uuid -d <temp |tee temp-decoded
af805061-328a-4a3d-a673-6123dfc47e70
eae5ca7c-76d7-43ea-a610-affef874235c
a60f549d-0367-42fc-b11b-9d06c70fbd1d
73288376-0e9d-4f3b-a2fc-3a1e5ce83f37
d55a7928-6a04-417d-8a78-4ea9bbde2132
```

```text
$ base62-uuid -e <temp-decoded
5LAIPC40yAz4V86pRxXDc0
79FDau3ZCOIJIn1pGWFW4S
53LjP8t5ptC3em3e8Fo0oX
3VIdCgsZHFmKYKF83vZtmZ
6Uai97JrjIzsYbVNW9R7o2
```

```text
$ base62-uuid -u
110624821538500593819620445913626559316
```

```text
$ base62-uuid -uc 5
116939220672656048972068396193106757785
109213015501285043041552607393977977495
230524414364629592303131221695422262052
333334515472599187693213468807117921087
130930884786481010444928320641857593970
```

```text
$ base62-uuid -puc 5
136946287236238064293775326583002555415
333531098568090440423398388004355154486
178208015731409944103885891145577554119
251416759794276796052393647408489659663
093008692659840688075505936644117853866
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

