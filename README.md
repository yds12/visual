# visual

Use the `Display` trait if it's satisfied, fallback to `Debug` otherwise, and
if neither are implemented use a default string value.

## Usage

```rust
use visual::{vis, Visual};

fn main() {
    // The `vis!` macro wraps your type in a such a way that it can decide which trait to
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

If neither traits are implemented, the string representation will be the one defined by the
constant `visual::NON_DISPLAYABLE`.

