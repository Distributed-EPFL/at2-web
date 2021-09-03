use at2_ns::proto;
use drop::crypto::sign;
use snafu::{OptionExt, ResultExt};

use super::accounts::{self, Accounts};

#[derive(snafu::Snafu, Debug)]
pub enum ProtoError {
    #[snafu(display("missing required field"))]
    MissingRequiredField,
    #[snafu(display("invalid serialization: {}", source))]
    InvalidSerialization { source: bincode::Error },
    #[snafu(display("invalid signature: {}", source))]
    InvalidSignature { source: sign::VerifyError },
}

pub struct Service {
    accounts: Accounts,
}

impl Service {
    pub fn new() -> Self {
        Self {
            accounts: Accounts::new(),
        }
    }
}

impl From<ProtoError> for tonic::Status {
    fn from(err: ProtoError) -> Self {
        Self::invalid_argument(err.to_string())
    }
}
impl From<accounts::Error> for tonic::Status {
    fn from(err: accounts::Error) -> Self {
        Self::invalid_argument(err.to_string())
    }
}

#[tonic::async_trait]
impl proto::name_service_server::NameService for Service {
    async fn put(
        &self,
        request: tonic::Request<proto::PutRequest>,
    ) -> Result<tonic::Response<proto::PutReply>, tonic::Status> {
        let message = request.into_inner();
        let account = message.account.context(MissingRequiredField)?;

        let pubkey: sign::PublicKey =
            bincode::deserialize(&account.public_key).context(InvalidSerialization)?;
        let signature: sign::Signature =
            bincode::deserialize(&message.signature).context(InvalidSerialization)?;

        signature
            .verify(&account.name, &pubkey)
            .context(InvalidSignature)?;

        self.accounts.put(pubkey, account.name).await?;

        Ok(tonic::Response::new(proto::PutReply {}))
    }

    async fn get_all(
        &self,
        _: tonic::Request<proto::GetAllRequest>,
    ) -> Result<tonic::Response<proto::GetAllReply>, tonic::Status> {
        let mut accounts = Vec::default();
        for (public_key, name) in self.accounts.get_all().await?.drain() {
            accounts.push(proto::Account {
                public_key: bincode::serialize(&public_key).context(InvalidSerialization)?,
                name,
            })
        }

        Ok(tonic::Response::new(proto::GetAllReply { accounts }))
    }
}
