use at2_ns::ThinUser;
use snafu::Snafu;

pub const DEFAULT_SEND_TRANSACTION_AMOUNT: usize = 12;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("missing field"))]
    MissingField,
}

pub struct SendTransactionBuilder {
    amount: usize,
    user: Option<ThinUser>,
}

impl Default for SendTransactionBuilder {
    fn default() -> Self {
        Self {
            amount: DEFAULT_SEND_TRANSACTION_AMOUNT,
            user: None,
        }
    }
}

impl SendTransactionBuilder {
    pub fn set_amount(&mut self, amount: usize) {
        self.amount = amount;
    }

    pub fn set_user(&mut self, user: ThinUser) {
        self.user = Some(user);
    }

    pub fn build(self) -> Result<(ThinUser, usize), Error> {
        match self.user {
            Some(user) => Ok((user, self.amount)),
            None => MissingField.fail(),
        }
    }
}
