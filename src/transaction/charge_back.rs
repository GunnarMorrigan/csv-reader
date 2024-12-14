use serde::{Deserialize, Serialize};

use super::TransactionId;
use crate::client::Client;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ChargeBack {
    client: Client,
    tx: TransactionId,
}

impl ChargeBack {
    pub fn new(client: Client, tx: TransactionId) -> Self {
        Self { client, tx }
    }
    pub fn client(&self) -> Client {
        self.client
    }
    pub fn transaction_id(&self) -> TransactionId {
        self.tx
    }
}
