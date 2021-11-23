use drop::crypto::sign;

use super::ThinUser;

/// User with a KeyPair
#[derive(Debug, Clone, PartialEq)]
pub struct FullUser {
    /// Name of this user
    pub name: String,
    keypair: sign::KeyPair,
}

impl FullUser {
    /// Create a new [`FullUser`]
    pub fn new(name: String, keypair: sign::KeyPair) -> Self {
        Self { name, keypair }
    }

    /// Return the public key
    pub fn public_key(&self) -> sign::PublicKey {
        self.keypair.public()
    }

    /// Return the keypair used
    pub fn keypair(&self) -> &sign::KeyPair {
        &self.keypair
    }

    /// Return a thin version of the user
    pub fn to_thin(self) -> ThinUser {
        ThinUser::new(self.name, self.keypair.public())
    }
}
