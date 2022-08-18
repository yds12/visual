mod displayable;

pub use displayable::{Visual, get_non_displayable_string, set_non_displayable_string};

// Not supposed to be used by end-users: for use by the macro only
pub use displayable::{GetDisplayFn, Wrap};
