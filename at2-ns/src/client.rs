use http::Uri;
use snafu::{ResultExt, Snafu};

use crate::{
    proto::{name_service_client::NameServiceClient, *},
    FullUser, ThinUser,
};

#[derive(Debug, Snafu)]
pub enum Error {
    Serialize {
        source: bincode::Error,
    },
    Signature {
        source: drop::crypto::sign::SignError,
    },
    Rpc {
        source: tonic::Status,
    },
}

#[derive(Clone)]
pub struct Client(NameServiceClient<grpc_web_client::Client>);

impl Client {
    pub fn new(uri: &Uri) -> Self {
        let mut url_string = uri.to_string();
        if uri.path() == "/" {
            // TODO fix upstream handling
            url_string.pop();
        }

        Self(NameServiceClient::new(grpc_web_client::Client::new(
            url_string,
        )))
    }

    pub async fn put(&mut self, user: &FullUser) -> Result<(), Error> {
        self.0
            .put(PutRequest {
                account: Some(Account {
                    public_key: bincode::serialize(&user.public_key()).context(Serialize)?,
                    name: user.name().to_owned(),
                }),
                signature: bincode::serialize(&user.sign(&user.name()).context(Signature)?)
                    .context(Serialize)?,
            })
            .await
            .context(Rpc)
            .map(|_| {})
    }

    pub async fn get_all(&mut self) -> Result<Vec<ThinUser>, Error> {
        self.0
            .get_all(GetAllRequest {})
            .await
            .context(Rpc)
            .map(|reply| {
                reply
                    .into_inner()
                    .accounts
                    .iter()
                    .map(|account| ThinUser::new(account.name.clone()))
                    .collect()
            })
    }
}
