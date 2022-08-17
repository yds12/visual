# visual

Use the `Display` trait if it's satisfied, fallback to `Debug` otherwise, and
if neither are implemented use a default string value.

## Why

The typical "nice" way to display things is via the `Display` trait. However, sometimes
this trait is not available, but `Debug` is. `Debug` is easy to derive. In those cases
it would be nice to use `Debug` as a fallback.

## Usage

```rust
use visual::{vis, Visual};

fn main() {
    // The `vis!` macro wraps your type in such a way that it can decide which trait to
    // use: `Display`, `Debug` or neither
    printer(vis!("hello"));       // `&str` implements `Display`, so use it
    printer(vis!(vec![1, 2, 3])); // `Vec` does not, but it impls `Debug`, so we use that

    struct MyStruct;
    printer(vis!(MyStruct));      // `MyStruct` impls neither, so we use a default string value
}

fn printer<T>(t: Visual<T>) {        // Use the `Visual` wrapper around your type
    println!("{}", t.get_display()); // Use `get_display` to get a string representation of your type
}

```

If neither trait is implemented, the string representation will be the one defined by the
constant `visual::NON_DISPLAYABLE`.

## Credits

For the magic to work, I use the "autoderef hack" proposed by
[Lukas Kalbertodt](http://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html),
which in turn is based on
[David Tolnay's](https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md)
technique.

## Links

* Documentation: [docs.rs](https://docs.rs/visual/latest/)
* Crate: [crates.io](https://crates.io/crates/visual/)
* Repository: [github.com](https://github.com/yds12/visual)

