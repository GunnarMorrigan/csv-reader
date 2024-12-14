use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::TransactionId;
use crate::client::Client;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Withdrawal {
    client: Client,
    tx: TransactionId,
    amount: Decimal,
}

impl Withdrawal {
    pub fn new(client: Client, tx: TransactionId, amount: Decimal) -> Self {
        Self { client, tx, amount }
    }
    pub fn client(&self) -> Client {
        self.client
    }
    pub fn transaction_id(&self) -> TransactionId {
        self.tx
    }
    pub fn amount(&self) -> Decimal {
        self.amount
    }
}
