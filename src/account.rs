use rust_decimal::Decimal;
use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::{
    client::Client,
    error::TransactionError,
    transaction::{Mutation, Transfer},
};

/// Represents a Users account
#[derive(Debug)]
pub struct Account {
    client: Client,
    available: Decimal,
    held: Decimal,
    locked: bool,
}

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Account", 5)?;
        s.serialize_field("client", &self.client)?;
        s.serialize_field("available", &self.available)?;
        s.serialize_field("held", &self.held)?;
        s.serialize_field("total", &self.total())?;
        s.serialize_field("locked", &self.locked)?;
        s.end()
    }
}

impl Account {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            available: Decimal::new(0, 0),
            held: Decimal::new(0, 0),
            locked: false,
        }
    }

    /// Returns the computed property `total`
    pub fn total(&self) -> Decimal {
        self.available + self.held
    }

    /// Returns the computed property `held`
    pub fn locked(&self) -> bool {
        self.locked
    }
    /// Locks the account
    pub fn lock(&mut self) {
        self.locked = true;
    }

    /// Handles the transfer transactions
    pub fn handle_transfer(&mut self, tx: &Transfer) -> Result<(), TransactionError> {
        if self.locked {
            return Err(TransactionError::AccountLocked);
        }

        match tx {
            Transfer::Deposit(deposit) => {
                self.available += deposit.amount();
            }
            Transfer::Withdrawal(withdrawal) => {
                if self.available < withdrawal.amount() {
                    return Err(TransactionError::InsufficientFunds);
                }
                self.available -= withdrawal.amount();
            }
        }

        Ok(())
    }

    /// Handles the mutation on the account
    pub fn handle_mutation(
        &mut self,
        mutation: &Mutation,
        amount: Decimal,
    ) -> Result<(), TransactionError> {
        if self.locked {
            return Err(TransactionError::AccountLocked);
        }

        match mutation {
            Mutation::Dispute(_) => {
                self.available -= amount;
                self.held += amount;
            }
            Mutation::Resolve(_) => {
                self.held -= amount;
                self.available += amount;
            }
            Mutation::ChargeBack(_) => {
                self.lock();
                self.held -= amount;
            }
        }
        Ok(())
    }
}
