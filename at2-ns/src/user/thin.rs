use drop::crypto::sign;

/// User on the network
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThinUser {
    /// Name of this user
    pub name: String,
    public_key: sign::PublicKey,
}

impl ThinUser {
    /// New user
    pub fn new(name: String, public_key: sign::PublicKey) -> Self {
        Self { name, public_key }
    }

    /// Return the public key
    pub fn public_key(&self) -> &sign::PublicKey {
        &self.public_key
    }
}
