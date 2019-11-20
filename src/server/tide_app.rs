use crate::consts::msg;
use std::{
    fmt::{Debug, Error as FmtError, Formatter},
    ops::{Deref, DerefMut},
};

/// The `TideApp` newtype exists because `tide::App` i) does not derive `Debug`, which makes using
/// it or any struct which encapsulates it impossible in tests (i.e. cannot use with `assert`s) :(
/// and ii) does not derive `PartialEq`, which makes writing some tests more difficult.
pub struct TideApp(pub tide::App<()>);

impl Debug for TideApp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", msg::DBG_TIDE_APP)
    }
}

impl Deref for TideApp {
    type Target = tide::App<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TideApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Declare all instances of TideApp to be equivalent, enabling the rest of the containing struct to
/// be comparable for equality.
impl PartialEq for TideApp {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
