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
base62-uuid 2.0.0
```

```text
$ base62-uuid
44q7vJFEYzX5sn0GTYeZl7
```

```text
$ base62-uuid -c 5 |tee temp
477hvayLG40yBF5sYGRbNp
5WjZC3o8Lu8a1dCRSO7445
595ozvxDHiI92gWop9fX2E
oCLfbA0rGcG6fpkDnpQWb
8eUzHF5mRqDiiUwvHg3ww
```

```text
$ base62-uuid -d <temp |tee temp-decoded
8733f4fa-1e21-4a10-a065-aed1571eca41
b5a1d474-13eb-4e57-af00-9e9a19bce595
a91a84e9-8640-4685-8ab5-4c1b43a86e76
1a9a5fb2-3cd7-4eea-a365-b5e3a14ab225
0495f4f8-bfd6-4066-85c8-648fbb327572
```

```text
$ base62-uuid -e <temp-decoded
477hvayLG40yBF5sYGRbNp
5WjZC3o8Lu8a1dCRSO7445
595ozvxDHiI92gWop9fX2E
oCLfbA0rGcG6fpkDnpQWb
8eUzHF5mRqDiiUwvHg3ww
```

```text
$ base62-uuid -u
123181482011302909886179025827763753137
```

```text
$ base62-uuid -uc 5
1547323194265792858496136677828949708
152390338469044313744374123164830390474
40746094517207513339150042445972545861
311687206025234608037526505270147717055
237892160212180305199647675308449469415
```

```text
$ base62-uuid -puc 5
019732916859402206693754687592923949838
319602182278917338120780681235125858760
221822346344209788450828427055017358482
043273776146999055742651614111372982641
294244154071858359744938058964118637506
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

