# About

Minimalist crate to generate Base62 UUIDs

[GitHub](https://github.com/qtfkwk/base62-uuid)
[Crates.io](https://crates.io/crates/base62-uuid)
[Lib.rs](https://lib.rs/crates/base62-uuid)
[Docs.rs](https://docs.rs/base62-uuid/latest/base62_uuid/)

# Command line utility

```text
$ base62-uuid -h
base62-uuid 
Base62 UUID

USAGE:
    base62-uuid [OPTIONS]

OPTIONS:
    -c, --count <COUNT>    Number of Base62 UUIDs to generate [default: 1]
    -d, --decode           Decode Base62 UUIDs from STDIN to standard UUIDs
    -e, --encode           Encode standard UUIDs from STDIN to Base62 UUIDs
    -p, --pad              Pad UUIDs with leading zeroes
    -u                     Generate/encode/decode u128 UUIDs instead of Base62 UUIDs
    -h, --help             Print help information

$ base62-uuid
39IZVIfyOJ6H2PNswyWqlF

$ base62-uuid -c 5 |tee temp
25mciqn6Uj2Y1fQv8Fj63m
1xFonVhl6BAY08Wxfbywao
75iEHpyZYYeQaAeAuY9Cab
47uxUuSrZYUZTM5NnIHjmk
4EzikvWmOBBrmVbQnHMocK

$ cat temp |base62-uuid -d |tee temp-decoded
44c771c2-d628-4d84-a5ed-3c53025848ca
40425315-ca7c-4711-9249-28ccf86189e2
e9069abf-adb7-4e02-ae6a-ff3885badb65
879fba26-6ba4-48f8-af5e-0cfa32155c3a
8b5fd11e-bbe2-41ea-9f3a-52f85fcdef50

$ cat temp-decoded |base62-uuid -e |tee temp-encoded
25mciqn6Uj2Y1fQv8Fj63m
1xFonVhl6BAY08Wxfbywao
75iEHpyZYYeQaAeAuY9Cab
47uxUuSrZYUZTM5NnIHjmk
4EzikvWmOBBrmVbQnHMocK

$ b3sum temp temp-encoded
8a22bc391bed5d0ee9c3b326d3251602df5ee1d52ab5f62f6538bc2e0531e5bb  temp
8a22bc391bed5d0ee9c3b326d3251602df5ee1d52ab5f62f6538bc2e0531e5bb  temp-encoded

$ base62-uuid -u
322307542572217969934229690286784727300

$ base62-uuid -uc 5
14982542445497376152403689592424402944
172687757666081737260525696030870372042
43167745396421733066649374535242967096
286929222040204797401597000791353515341
317248292745988805767278411123975748262

$ base62-uuid -puc 5
145208551251690730667864324716102711897
147013240575534556220474725701279222198
098510246896224898774321662663208667390
083581755085478548623485238303158515246
168393287624526834515206282224381425158
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

