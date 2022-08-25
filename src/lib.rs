//! This crate enables one to wrap an object so that in a generic context it
//! can use [`Display`] or [`Debug`], or, in case neither are available,
//! a default string, to be displayed.
//!
//! ```rust
//! use visual::{vis, Visual};
//! 
//! fn main() {
//!     // The `vis!` macro wraps your type in such a way that it can decide which trait to
//!     // use: `Display`, `Debug` or neither
//!     printer(vis!("hello"));       // `&str` implements `Display`, so use it
//!     printer(vis!(vec![1, 2, 3])); // `Vec` does not, but it impls `Debug`, so we use that
//! 
//!     struct MyStruct;
//!     printer(vis!(MyStruct));      // `MyStruct` impls neither, so we use a default string value
//! }
//! 
//! fn printer<T>(t: Visual<T>) {        // Use the `Visual` wrapper around your type
//!     println!("{}", t.get_display()); // Use `get_display` to get a string representation of your type
//! }
//! 
//! ```

mod displayable;

pub use displayable::{Visual, get_non_displayable_string, set_non_displayable_string};

// Not supposed to be used by end-users: for use by the macro only
pub use displayable::{GetDisplayFn, Wrap};
