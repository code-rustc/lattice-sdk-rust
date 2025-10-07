pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ContentIdentifier {
    /// A valid path must not contain the following:
    /// - Spaces or Tabs
    /// - Special characters other than underscore (_), dash (-), period (.) and slash (/)
    /// - Non-ASCII characters such as accents or symbols
    /// Paths must not start with a leading space.
    pub path: String,
    /// The SHA-256 checksum of this object.
    pub checksum: String,
}
