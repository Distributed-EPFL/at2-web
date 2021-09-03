use drop::crypto::sign;

#[derive(Debug)]
pub struct User {
    name: String,
    keypair: sign::KeyPair,
}

impl User {
    pub fn new(name: String) -> Self {
        let keypair = sign::KeyPair::random();

        // TODO push on nameserver

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
