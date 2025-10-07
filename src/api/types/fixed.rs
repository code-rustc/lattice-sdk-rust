pub use crate::prelude::*;

/// A fix of a signal. No extra fields but it is expected that location should be populated when using this report.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Fixed {}
