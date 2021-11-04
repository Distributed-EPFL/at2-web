use at2_ns::ThinUser;
use snafu::Snafu;

const DEFAULT_SEND_TRANSACTION_AMOUNT: usize = 12;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("missing field"))]
    MissingField,
}

pub struct TransactionBuilder {
    pub amount: usize,
    pub user: Option<ThinUser>,
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self {
            amount: DEFAULT_SEND_TRANSACTION_AMOUNT,
            user: None,
        }
    }
}

impl TransactionBuilder {
    pub fn build(self) -> Result<(ThinUser, usize), Error> {
        match self.user {
            Some(user) => Ok((user, self.amount)),
            None => MissingField.fail(),
        }
    }
}
