use drop::crypto::sign;

#[derive(Debug)]
pub struct FullUser {
    name: String,
    keypair: sign::KeyPair,
}

impl FullUser {
    pub fn new(name: String, keypair: sign::KeyPair) -> Self {
        Self { name, keypair }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn public_key(&self) -> sign::PublicKey {
        self.keypair.public()
    }

    pub fn sign(
        &self,
        message: &impl serde::Serialize,
    ) -> Result<sign::Signature, sign::SignError> {
        self.keypair.sign(message)
    }
}
