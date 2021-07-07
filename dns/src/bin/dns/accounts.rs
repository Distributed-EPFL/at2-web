use std::collections::HashMap;

use drop::crypto::sign;
use tokio::sync::{mpsc, oneshot};

#[derive(snafu::Snafu, Debug)]
pub enum Error {
    #[snafu(display("gone on send"))]
    GoneOnSend,
    #[snafu(display("gone on recv"))]
    GoneOnRecv,
}

type Name = String;
type Map = HashMap<sign::PublicKey, Name>;

type Response<T> = oneshot::Sender<T>;

enum Commands {
    Put {
        pubkey: Box<sign::PublicKey>,
        name: Name,
        resp: Response<()>,
    },
    GetAll {
        resp: Response<Map>,
    },
}

pub struct Accounts {
    agent: mpsc::Sender<Commands>,
}

impl Accounts {
    pub fn new() -> Self {
        Self {
            agent: AccountsHandler::new().spawn(),
        }
    }

    pub async fn put(&self, pubkey: sign::PublicKey, name: Name) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();

        self.agent
            .send(Commands::Put {
                pubkey: Box::new(pubkey),
                name,
                resp: tx,
            })
            .await
            .map_err(|_| Error::GoneOnSend)?;

        rx.await.map_err(|_| Error::GoneOnRecv)
    }

    pub async fn get_all(&self) -> Result<Map, Error> {
        let (tx, rx) = oneshot::channel();

        self.agent
            .send(Commands::GetAll { resp: tx })
            .await
            .map_err(|_| Error::GoneOnSend)?;

        rx.await.map_err(|_| Error::GoneOnRecv)
    }
}

struct AccountsHandler {
    map: Map,
}

impl AccountsHandler {
    fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    fn spawn(mut self) -> mpsc::Sender<Commands> {
        let (tx, mut rx) = mpsc::channel(32);

        tokio::spawn(async move {
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    Commands::Put { pubkey, name, resp } => {
                        self.map.insert(*pubkey, name);
                        let _ = resp.send(());
                    }
                    Commands::GetAll { resp } => {
                        let _ = resp.send(self.map.clone());
                    }
                }
            }
        });

        tx
    }
}
