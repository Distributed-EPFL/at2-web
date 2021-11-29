use drop::crypto::sign;

/// Other users on the network
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Contact {
    /// Name of this user
    pub name: String,
    public_key: sign::PublicKey,
}

impl Contact {
    /// New contact
    pub fn new(name: String, public_key: sign::PublicKey) -> Self {
        Self { name, public_key }
    }

    /// Return the public key
    pub fn public_key(&self) -> &sign::PublicKey {
        &self.public_key
    }
}
