#![deny(missing_docs)]

//! Name service for AT2, making a link between usernames and public keys

/// `tonic-build` generated files
#[allow(missing_docs)]
pub mod proto;

pub mod client;

mod user;
pub use user::User;

mod contact;
pub use contact::Contact;
