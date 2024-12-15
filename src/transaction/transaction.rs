use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::client::Client;

use super::{
    charge_back::ChargeBack, deposit::Deposit, dispute::Dispute, error::DeserializationError,
    resolve::Resolve, withdrawal::Withdrawal, TransactionId, TransactionRow, TransactionType,
};

/// Represents all possible transactions
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Transaction {
    /// Represents a deposit and withdrawal transaction.
    Transfer(Transfer),
    /// Represents a mutation to a Transfer transaction.
    Mutation(Mutation),
}

impl Transaction {
    pub fn client(&self) -> Client {
        match self {
            Transaction::Transfer(Transfer::Deposit(d)) => d.client(),
            Transaction::Transfer(Transfer::Withdrawal(w)) => w.client(),
            Transaction::Mutation(m) => match m {
                Mutation::Dispute(d) => d.client(),
                Mutation::Resolve(r) => r.client(),
                Mutation::ChargeBack(c) => c.client(),
            },
        }
    }
}

impl TryFrom<TransactionRow> for Transaction {
    type Error = DeserializationError;

    fn try_from(value: TransactionRow) -> Result<Self, Self::Error> {
        match (value.transaction_type, value.amount) {
            (TransactionType::Deposit, Some(amount)) => Ok(Transaction::Transfer(
                Transfer::Deposit(Deposit::new(value.client, value.transaction_id, amount)),
            )),
            (TransactionType::Withdrawal, Some(amount)) => Ok(Transaction::Transfer(
                Transfer::Withdrawal(Withdrawal::new(value.client, value.transaction_id, amount)),
            )),
            (TransactionType::Dispute, _) => Ok(Transaction::Mutation(Mutation::Dispute(
                Dispute::new(value.client, value.transaction_id),
            ))),
            (TransactionType::Resolve, _) => Ok(Transaction::Mutation(Mutation::Resolve(
                Resolve::new(value.client, value.transaction_id),
            ))),
            (TransactionType::ChargeBack, _) => Ok(Transaction::Mutation(Mutation::ChargeBack(
                ChargeBack::new(value.client, value.transaction_id),
            ))),
            _ => Err(DeserializationError::ParseError(value)),
        }
    }
}

/// Represents a category FIAT moving in or out of an account.
///
/// These transactions can be mutated by a [`Mutation`] transaction after being processed.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Transfer {
    Deposit(Deposit),
    Withdrawal(Withdrawal),
}

impl Transfer {
    pub fn transaction_id(&self) -> TransactionId {
        match self {
            Transfer::Deposit(d) => d.transaction_id(),
            Transfer::Withdrawal(w) => w.transaction_id(),
        }
    }

    pub fn amount(&self) -> Decimal {
        match self {
            Transfer::Deposit(d) => d.amount(),
            Transfer::Withdrawal(w) => w.amount(),
        }
    }
}

/// Mutations represent transactions that are dependent on a [`Transfer`] transaction.
/// They can adjust a transaction's state. Multiple mutations can be applied to a single transaction.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Mutation {
    Dispute(Dispute),
    Resolve(Resolve),
    ChargeBack(ChargeBack),
}

impl Mutation {
    pub fn transaction_id(&self) -> TransactionId {
        match self {
            Mutation::Dispute(d) => d.transaction_id(),
            Mutation::Resolve(r) => r.transaction_id(),
            Mutation::ChargeBack(c) => c.transaction_id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::transaction_reader;

    #[test]
    pub fn example_input_test() {
        let data = r"type, client, tx, amount
            deposit, 1, 1, 1.1234
            deposit, 2, 2, 2.0123
            deposit, 1, 3, 2.0000
            withdrawal, 1, 4, 1.5
            withdrawal, 2, 5, 3.0
            dispute, 1, 1
            dispute, 1, 1
            dispute, 2, 2,
            chargeback, 1, 1
            chargeback, 2, 2,
            resolve, 1, 1
            resolve, 2, 2";

        dbg!(data);

        let data = data.trim_matches(char::is_whitespace).as_bytes();

        let reader = transaction_reader(data);

        for tx in reader {
            assert!(tx.is_ok());
            let tx_row = tx.unwrap();
            let tx = super::Transaction::try_from(tx_row);
            assert!(tx.is_ok());
        }
    }
}
