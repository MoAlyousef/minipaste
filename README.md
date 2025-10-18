# minipaste

A minimalist alternative to the [paste crate](https://crates.io/crates/paste), which is no longer maintained and may flag a `cargo audit/vet` warning. It does not handle case conversion!
Usage is similar to the `paste` crate:
```toml
minipaste = "0.1"
```

```rust
use minipaste::paste;
paste! {
    // Defines a const called `QRST`.
    const [<Q R S T>]: &str = "success!";
}

fn main() {
    assert_eq!(
        paste! { [<Q R S T>].len() },
        8,
    );
}
```