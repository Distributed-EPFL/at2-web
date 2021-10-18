use drop::crypto::sign;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThinUser {
    name: String,
    public_key: sign::PublicKey,
}

impl ThinUser {
    pub fn new(name: String, public_key: sign::PublicKey) -> Self {
        Self { name, public_key }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn public_key(&self) -> &sign::PublicKey {
        &self.public_key
    }
}
