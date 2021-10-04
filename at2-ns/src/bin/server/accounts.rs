use std::collections::{HashMap, HashSet};

use drop::crypto::sign;
use tokio::sync::{mpsc, oneshot};

#[derive(snafu::Snafu, Debug)]
pub enum Error {
    #[snafu(display("name already registered"))]
    AlreadyExisting,
    #[snafu(display("gone on send"))]
    GoneOnSend,
    #[snafu(display("gone on recv"))]
    GoneOnRecv,
}

type Name = String;

type Response<T> = oneshot::Sender<T>;

enum Commands {
    Put {
        pubkey: Box<sign::PublicKey>,
        name: Name,
        resp: Response<Result<(), Error>>,
    },
    GetAll {
        resp: Response<HashMap<sign::PublicKey, Name>>,
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

        rx.await.map_err(|_| Error::GoneOnRecv)?
    }

    pub async fn get_all(&self) -> Result<HashMap<sign::PublicKey, Name>, Error> {
        let (tx, rx) = oneshot::channel();

        self.agent
            .send(Commands::GetAll { resp: tx })
            .await
            .map_err(|_| Error::GoneOnSend)?;

        rx.await.map_err(|_| Error::GoneOnRecv)
    }
}

struct AccountsHandler {
    pubkey_to_name: HashMap<sign::PublicKey, Name>,
    names: HashSet<Name>,
}

impl AccountsHandler {
    fn new() -> Self {
        Self {
            pubkey_to_name: Default::default(),
            names: Default::default(),
        }
    }

    fn spawn(mut self) -> mpsc::Sender<Commands> {
        let (tx, mut rx) = mpsc::channel(32);

        tokio::spawn(async move {
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    Commands::Put { pubkey, name, resp } => {
                        use std::collections::hash_map::Entry;

                        let _ = match self.pubkey_to_name.entry(*pubkey) {
                            // nobody claimed the name
                            Entry::Vacant(entry) => {
                                entry.insert(name.clone());
                                self.names.insert(name.clone());

                                resp.send(Ok(()))
                            }
                            // same association already existing
                            Entry::Occupied(existing) if existing.get() == &name => {
                                resp.send(Ok(()))
                            }
                            // trying to put an already existing association
                            Entry::Occupied(_) if self.names.get(&name).is_some() => {
                                resp.send(AlreadyExisting.fail())
                            }
                            // changing its name
                            Entry::Occupied(mut entry) => {
                                self.names.remove(entry.get());
                                self.names.insert(name.clone());
                                entry.insert(name.clone());

                                resp.send(Ok(()))
                            }
                        };

                        debug_assert_eq!(
                            self.names,
                            self.pubkey_to_name.values().cloned().collect(),
                        )
                    }
                    Commands::GetAll { resp } => {
                        let _ = resp.send(self.pubkey_to_name.clone());
                    }
                }
            }
        });

        tx
    }
}
