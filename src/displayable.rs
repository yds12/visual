use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};

pub const NON_DISPLAYABLE: &'static str = "";

pub struct Wrap<T>(pub T);

pub trait GetDisplayFn {
    type Target;
    fn get_display_fn(&self) -> Box<dyn Fn(&Self::Target) -> String>;
}

impl<T> GetDisplayFn for &Wrap<T> {
    type Target = T;
    fn get_display_fn(&self) -> Box<dyn Fn(&Self::Target) -> String> {
        Box::new(|_| {
            String::from(NON_DISPLAYABLE)
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

pub struct Visual<T> {
    inner: T,
    display_fn: Box<dyn Fn(&T) -> String>
}

impl<T> Visual<T> {
    pub fn new(inner: T, display_fn: Box<dyn Fn(&T) -> String>) -> Self {
        Self {
            inner,
            display_fn
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

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
    use crate::{vis, Visual, NON_DISPLAYABLE};

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
        assert_display_eq(vis!(Val), NON_DISPLAYABLE);
        assert_display_eq(vis!(ValDebug), "ValDebug");
        assert_display_eq(vis!(ValDisplayDebug), "ValDisplayDebug display");
        assert_display_eq(vis!(ValDisplay), "ValDisplay display");
    }

    #[test]
    fn works_with_refs() {
        assert_display_eq(vis!(&Val), NON_DISPLAYABLE);
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

