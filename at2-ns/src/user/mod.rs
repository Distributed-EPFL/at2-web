//! There is two types of user,
//! one having a KeyPair which is usually the current user,
//! and one having only a PublicKey, representing the other users on the network.

mod full;
mod thin;

pub use full::FullUser;
pub use thin::ThinUser;
