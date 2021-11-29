use drop::crypto::sign;

use super::Contact;

/// Current user
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct User {
    /// Name of this user
    pub name: String,
    keypair: sign::KeyPair,
}

impl User {
    /// Create a new [`User`]
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
    pub fn to_thin(self) -> Contact {
        Contact::new(self.name, self.keypair.public())
    }
}
