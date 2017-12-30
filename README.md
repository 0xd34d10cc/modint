# modint
Modular arithmetic integer type

# Example
```rust
use typenum::U17;

type M17 = Unsigned<u8, U17>;

assert_eq!(M17::from(16) + 14.into(), 13.into());
assert_eq!(M17::from(11) - 15.into(), 13.into());
assert_eq!(M17::from(11) * 15.into(), 12.into());
assert_eq!(M17::from(13) / 4 .into(),  3.into());
```