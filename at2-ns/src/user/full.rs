use drop::crypto::sign;

#[derive(Debug, Clone, PartialEq)]
pub struct FullUser {
    pub name: String,
    keypair: sign::KeyPair,
}

impl FullUser {
    pub fn new(name: String, keypair: sign::KeyPair) -> Self {
        Self { name, keypair }
    }

    pub fn public_key(&self) -> sign::PublicKey {
        self.keypair.public()
    }

    pub fn keypair(&self) -> &sign::KeyPair {
        &self.keypair
    }
}
