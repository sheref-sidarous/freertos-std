#![stable(feature = "rust1", since = "1.0.0")]

pub mod ffi;

#[stable(feature = "rust1", since = "1.0.0")]
pub mod services;

/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.
#[stable(feature = "rust1", since = "1.0.0")]
pub mod prelude {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::ffi::{OsStrExt, OsStringExt};
}
