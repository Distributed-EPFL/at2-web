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

                        let _ = resp.send(match self.pubkey_to_name.entry(*pubkey) {
                            // nobody claimed the name
                            Entry::Vacant(entry) if !self.names.contains(&name) => {
                                entry.insert(name.clone());
                                self.names.insert(name.clone());

                                Ok(())
                            }
                            // same association already existing
                            Entry::Occupied(existing) if existing.get() == &name => Ok(()),

                            // someone already claimed the name
                            Entry::Occupied(_) if self.names.contains(&name) => {
                                AlreadyExisting.fail()
                            }
                            Entry::Vacant(_) => AlreadyExisting.fail(),

                            // changing its name
                            Entry::Occupied(mut entry) => {
                                self.names.remove(entry.get());
                                self.names.insert(name.clone());
                                entry.insert(name.clone());

                                Ok(())
                            }
                        });

                        debug_assert!({
                            let mut names = self.names.clone();
                            self.pubkey_to_name.values().all(|name| names.remove(name))
                        })
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

#[cfg(test)]
mod tests {
    use at2_ns::User;
    use drop::crypto::sign::KeyPair;

    use super::Accounts;

    #[tokio::test]
    async fn put_once_returns_it_in_get_all() {
        let accounts = Accounts::new();
        let user = User::new("user".to_owned(), KeyPair::random());

        accounts
            .put(user.public_key(), user.name.clone())
            .await
            .expect("put user");

        assert_eq!(
            accounts
                .get_all()
                .await
                .expect("get all")
                .into_iter()
                .collect::<Vec<_>>(),
            vec![(user.public_key(), "user".to_owned())],
        );
    }

    #[tokio::test]
    async fn put_twice_update_name() {
        let accounts = Accounts::new();
        let user = User::new("user".to_owned(), KeyPair::random());

        accounts
            .put(user.public_key(), "first".to_owned())
            .await
            .expect("first put");
        accounts
            .put(user.public_key(), "second".to_owned())
            .await
            .expect("second put");

        assert_eq!(
            accounts
                .get_all()
                .await
                .expect("get all")
                .into_iter()
                .collect::<Vec<_>>(),
            vec![(user.public_key(), "second".to_owned())],
        );
    }

    #[tokio::test]
    async fn put_for_same_name_with_different_pubkey_fails() {
        let accounts = Accounts::new();

        let first_user = User::new("user".to_owned(), KeyPair::random());
        let second_user = User::new("user".to_owned(), KeyPair::random());

        accounts
            .put(first_user.public_key(), first_user.name.clone())
            .await
            .expect("put first user");
        accounts
            .put(second_user.public_key(), second_user.name.clone())
            .await
            .expect_err("fail to put second user");
    }

    #[tokio::test]
    async fn update_name_for_another_already_existing() {
        let accounts = Accounts::new();

        let first_user = User::new("user".to_owned(), KeyPair::random());
        let second_user = User::new("usr".to_owned(), KeyPair::random());

        accounts
            .put(first_user.public_key(), first_user.name.clone())
            .await
            .expect("put first user");
        accounts
            .put(second_user.public_key(), second_user.name.clone())
            .await
            .expect("put second user");

        accounts
            .put(second_user.public_key(), "user".to_owned())
            .await
            .expect_err("fail to update name for second user");
    }
}
