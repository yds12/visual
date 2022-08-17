mod displayable;

pub use displayable::{Displayable, NON_DISPLAYABLE};

// Not supposed to be used by end-users: for use by the macro only
pub use displayable::{GetDisplayFn, Wrap};
