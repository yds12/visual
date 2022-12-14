use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use std::sync::Once;

static INIT: Once = Once::new();
static mut NON_DISPLAYABLE: &'static str = "";

/// Set the `String` that should be used to display [`Visual`] objects that do
/// not implement neither [`Display`] nor [`Debug`].
pub fn set_non_displayable_string(value: &'static str) -> Result<(), String> {
    if INIT.is_completed() {
        return Err("Attempting to initialize static variable `NON_DISPLAYABLE` twice".to_owned());
    }

    INIT.call_once(|| {
        unsafe { NON_DISPLAYABLE = value; }
    });

    Ok(())
}

/// Get the `String` that should be used to display [`Visual`] objects that do
/// not implement neither [`Display`] nor [`Debug`].
pub fn get_non_displayable_string() -> &'static str {
    // SAFETY: we only allow mutation of this static once, using `Once`
    unsafe { NON_DISPLAYABLE }
}

// Only used by the macro
#[doc(hidden)]
pub struct Wrap<T>(pub T);

// Only used by the macro
#[doc(hidden)]
pub trait GetDisplayFn {
    type Target;
    fn get_display_fn(&self) -> Box<dyn Fn(&Self::Target) -> String>;
}

impl<T> GetDisplayFn for &Wrap<T> {
    type Target = T;
    fn get_display_fn(&self) -> Box<dyn Fn(&Self::Target) -> String> {
        Box::new(|_| {
            String::from(get_non_displayable_string())
        })
    }
}

impl<T: Debug> GetDisplayFn for &&Wrap<T> {
    type Target = T;
    fn get_display_fn(&self) -> Box<dyn Fn(&Self::Target) -> String> {
        Box::new(|val: &T| {
            format!("{:?}", val) 
        })
    }
}

impl<T: Display> GetDisplayFn for &&&Wrap<T> {
    type Target = T;
    fn get_display_fn(&self) -> Box<dyn Fn(&Self::Target) -> String> {
        Box::new(|val: &T| {
            format!("{}", val) 
        })
    }
}

/// A value that can be displayed either via `Display` or `Debug`, or with a
/// default representation.
pub struct Visual<T> {
    inner: T,
    display_fn: Box<dyn Fn(&T) -> String>
}

impl<T> Visual<T> {
    // Should only be used by the macro
    #[doc(hidden)]
    pub fn new(inner: T, display_fn: Box<dyn Fn(&T) -> String>) -> Self {
        Self {
            inner,
            display_fn
        }
    }

    /// Extract the inner value
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Get a `String` that represents this object
    pub fn get_display(&self) -> String {
        (self.display_fn)(&self.inner)
    }
}

impl<T> Deref for Visual<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for Visual<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

/// Wrap an object into the [`Visual`] type, which will choose a function to
/// obtain a visual representation of this object using these rules:
///
/// * if the object implements `Display`, use that;
/// * otherwise, if it implements `Debug`, use that;
/// * otherwise use a default representation.
///
/// The default representation can be changed with the function
/// [`set_non_displayable_string`].
#[macro_export]
macro_rules! vis {
    ($expr:expr) => {{
        use $crate::GetDisplayFn;
        let display_fn = (&&&&$crate::Wrap($expr)).get_display_fn();
        $crate::Visual::new($expr, display_fn)
    }}
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;
    use crate::{vis, Visual, get_non_displayable_string};

    struct Val;

    #[derive(Debug)]
    struct ValDebug;

    #[derive(Debug)]
    struct ValDisplayDebug;
    impl Display for ValDisplayDebug {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "ValDisplayDebug display")
        }
    }

    struct ValDisplay;
    impl Display for ValDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "ValDisplay display")
        }
    }

    fn assert_display_eq<T>(t: Visual<T>, other: &str) {
        assert_eq!(t.get_display(), other);
    }

    #[test]
    fn works_with_values() {
        assert_display_eq(vis!(6), "6");
        assert_display_eq(vis!(3.14), "3.14");
        assert_display_eq(vis!(vec![1, 2, 3]), "[1, 2, 3]");
        assert_display_eq(vis!(Val), get_non_displayable_string());
        assert_display_eq(vis!(ValDebug), "ValDebug");
        assert_display_eq(vis!(ValDisplayDebug), "ValDisplayDebug display");
        assert_display_eq(vis!(ValDisplay), "ValDisplay display");
    }

    #[test]
    fn works_with_refs() {
        assert_display_eq(vis!(&Val), get_non_displayable_string());
        assert_display_eq(vis!(&ValDebug), "ValDebug");
        assert_display_eq(vis!(&ValDisplayDebug), "ValDisplayDebug display");
        assert_display_eq(vis!(&ValDisplay), "ValDisplay display");
    }

    #[test]
    fn works_with_string_slice() {
        assert_display_eq(vis!("slice"), "slice");
    }

    #[test]
    fn works_with_string() {
        assert_display_eq(vis!(String::from("hello")), "hello");
    }
}

