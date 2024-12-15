use std::collections::{hash_map::Entry, HashMap};

use crate::{
    account::Account,
    client::Client,
    error::TransactionError,
    transaction::{Transaction, TransactionId},
    transaction_record::TransactionRecord,
};

/// Represents all accounts in the system and the transactions that have been processed.
/// The ledger does not keep transaction mutations but merely the current state of the transaction.
/// This keeps the ledger simple and reduces the overal size of the structure.
#[derive(Debug)]
pub struct TrialBalance {
    // Hashmaps are the recommended data structure for this task.
    // A Vec could be faster for accounts if max number of clients is known. A vec could also cause wasted space.
    accounts: HashMap<Client, Account>,
    ledger: HashMap<TransactionId, TransactionRecord>,
}

impl TrialBalance {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::with_capacity(1000),
            ledger: HashMap::with_capacity(100000),
        }
    }

    pub fn to_csv<W>(&self, w: &mut W)
    where
        W: std::io::Write,
    {
        let mut wtr = csv::WriterBuilder::new().has_headers(true).from_writer(w);
        for account in self.accounts.values() {
            wtr.serialize(account).unwrap();
        }
    }

    pub fn handle_transaction(&mut self, tx: Transaction) -> Result<(), TransactionError> {
        let account = self
            .accounts
            .entry(tx.client())
            .or_insert(Account::new(tx.client()));

        match tx {
            Transaction::Transfer(transfer) => {
                tracing::debug!("Handling transfer {:?}", transfer);
                if let Entry::Vacant(e) = self.ledger.entry(transfer.transaction_id()) {
                    let res = account.handle_transfer(&transfer);
                    // Stick the transaction into the ledger
                    // This might not be desired if you only want to keep track of succesful transactions.
                    // Alternatively, it is possible to keep track of success on the transaction in the ledger
                    e.insert(TransactionRecord::new(transfer));
                    // Return the result of the transfer handling
                    res?;
                } else {
                    return Err(TransactionError::DuplicateTransaction(
                        transfer.transaction_id(),
                    ));
                }
            }
            Transaction::Mutation(mutation) => {
                tracing::debug!("Handling mutation {:?}", mutation);
                if let Some(tx_record) = self.ledger.get_mut(&mutation.transaction_id()) {
                    tracing::debug!("Found transaction record {:?}", tx_record);
                    // Mutate the transaction record
                    tx_record.mutate(&mutation)?;
                    // update the account to reflect mutation
                    account.handle_mutation(&mutation, tx_record.tx().amount())?;
                } else {
                    return Err(TransactionError::MissingTransaction(
                        mutation.transaction_id(),
                    ));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use crate::{
        client::Client,
        transaction::{
            charge_back::ChargeBack, deposit::Deposit, dispute::Dispute, resolve::Resolve,
            withdrawal::Withdrawal, Mutation, Transaction, TransactionId, Transfer,
        },
    };

    #[test]
    fn test_trial_balance() {
        let transactions = vec![
            Transaction::Transfer(Transfer::Deposit(Deposit::new(
                Client::new(1),
                TransactionId::new(1),
                Decimal::new(100, 0),
            ))),
            Transaction::Transfer(Transfer::Withdrawal(Withdrawal::new(
                Client::new(1),
                TransactionId::new(2),
                Decimal::new(50, 0),
            ))),
            Transaction::Mutation(Mutation::Dispute(Dispute::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
            Transaction::Mutation(Mutation::Resolve(Resolve::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
            // This should fail because the dispute was resolved
            Transaction::Mutation(Mutation::ChargeBack(ChargeBack::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
            Transaction::Transfer(Transfer::Withdrawal(Withdrawal::new(
                Client::new(1),
                TransactionId::new(3),
                Decimal::new(50, 0),
            ))),
            // Duplicate transaction
            Transaction::Transfer(Transfer::Withdrawal(Withdrawal::new(
                Client::new(1),
                TransactionId::new(3),
                Decimal::new(50, 0),
            ))),
            // Reference not seen transaction
            Transaction::Mutation(Mutation::ChargeBack(ChargeBack::new(
                Client::new(1),
                TransactionId::new(100),
            ))),
        ];
        let results = vec![
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Err(crate::error::TransactionError::ChargeBackError),
            Ok(()),
            Err(crate::error::TransactionError::DuplicateTransaction(
                TransactionId::new(3),
            )),
            Err(crate::error::TransactionError::MissingTransaction(
                TransactionId::new(100),
            )),
        ];
        let mut trial_balance = super::TrialBalance::new();
        for (tx, expected_res) in transactions.into_iter().zip(results.into_iter()) {
            let res = trial_balance.handle_transaction(tx);
            assert_eq!(res, expected_res);
        }
    }

    #[test]
    fn test_trial_balance_2() {
        let transactions = vec![
            Transaction::Transfer(Transfer::Deposit(Deposit::new(
                Client::new(1),
                TransactionId::new(1),
                Decimal::new(100, 0),
            ))),
            Transaction::Transfer(Transfer::Withdrawal(Withdrawal::new(
                Client::new(1),
                TransactionId::new(2),
                Decimal::new(50, 0),
            ))),
            Transaction::Mutation(Mutation::Dispute(Dispute::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
            Transaction::Mutation(Mutation::Resolve(Resolve::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
            // This should fail because the dispute was resolved
            Transaction::Mutation(Mutation::Dispute(Dispute::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
            Transaction::Transfer(Transfer::Withdrawal(Withdrawal::new(
                Client::new(1),
                TransactionId::new(3),
                Decimal::new(50, 0),
            ))),
            // Duplicate transaction
            Transaction::Transfer(Transfer::Withdrawal(Withdrawal::new(
                Client::new(1),
                TransactionId::new(3),
                Decimal::new(50, 0),
            ))),
        ];
        let results = vec![
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Err(crate::error::TransactionError::InsufficientFunds),
            Err(crate::error::TransactionError::DuplicateTransaction(
                TransactionId::new(3),
            )),
        ];
        let mut trial_balance = super::TrialBalance::new();
        for (index, (tx, expected_res)) in transactions
            .into_iter()
            .zip(results.into_iter())
            .enumerate()
        {
            let res = trial_balance.handle_transaction(tx);
            assert_eq!(res, expected_res, "Failed on index {}", index);
        }
    }

    #[test]
    fn test_resolve_chargeback() {
        let transactions = vec![
            Transaction::Transfer(Transfer::Deposit(Deposit::new(
                Client::new(1),
                TransactionId::new(1),
                Decimal::new(100, 0),
            ))),
            Transaction::Transfer(Transfer::Withdrawal(Withdrawal::new(
                Client::new(1),
                TransactionId::new(2),
                Decimal::new(50, 0),
            ))),
            Transaction::Mutation(Mutation::Dispute(Dispute::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
            Transaction::Mutation(Mutation::ChargeBack(ChargeBack::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
            // This should fail because the dispute was charged back
            Transaction::Mutation(Mutation::Resolve(Resolve::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
        ];
        let results = vec![
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Err(crate::error::TransactionError::ResolveError),
        ];
        let mut trial_balance = super::TrialBalance::new();
        for (index, (tx, expected_res)) in transactions
            .into_iter()
            .zip(results.into_iter())
            .enumerate()
        {
            let res = trial_balance.handle_transaction(tx);
            assert_eq!(res, expected_res, "Failed on index {}", index);
        }
    }

    #[test]
    fn test_dispute_chargeback() {
        let transactions = vec![
            Transaction::Transfer(Transfer::Deposit(Deposit::new(
                Client::new(1),
                TransactionId::new(1),
                Decimal::new(100, 0),
            ))),
            Transaction::Mutation(Mutation::Dispute(Dispute::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
            Transaction::Mutation(Mutation::ChargeBack(ChargeBack::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
            // This should fail because the dispute was charged back
            Transaction::Mutation(Mutation::Dispute(Dispute::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
        ];
        let results = vec![
            Ok(()),
            Ok(()),
            Ok(()),
            Err(crate::error::TransactionError::DisputeError),
        ];
        let mut trial_balance = super::TrialBalance::new();
        for (index, (tx, expected_res)) in transactions
            .into_iter()
            .zip(results.into_iter())
            .enumerate()
        {
            let res = trial_balance.handle_transaction(tx);
            assert_eq!(res, expected_res, "Failed on index {}", index);
        }
    }

    #[test]
    fn test_withdrawal_locked() {
        let transactions = vec![
            Transaction::Transfer(Transfer::Deposit(Deposit::new(
                Client::new(1),
                TransactionId::new(1),
                Decimal::new(100, 0),
            ))),
            Transaction::Mutation(Mutation::Dispute(Dispute::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
            Transaction::Mutation(Mutation::ChargeBack(ChargeBack::new(
                Client::new(1),
                TransactionId::new(1),
            ))),
            Transaction::Transfer(Transfer::Withdrawal(Withdrawal::new(
                Client::new(1),
                TransactionId::new(2),
                Decimal::new(100, 0),
            ))),
            // This should work because money send via bank transfer etc will still come in after locking
            Transaction::Transfer(Transfer::Deposit(Deposit::new(
                Client::new(1),
                TransactionId::new(3),
                Decimal::new(50, 0),
            ))),
        ];
        let results = vec![
            Ok(()),
            Ok(()),
            Ok(()),
            Err(crate::error::TransactionError::AccountLocked),
            Ok(()),
        ];
        let mut trial_balance = super::TrialBalance::new();
        for (index, (tx, expected_res)) in transactions
            .into_iter()
            .zip(results.into_iter())
            .enumerate()
        {
            let res = trial_balance.handle_transaction(tx);
            assert_eq!(res, expected_res, "Failed on index {}", index);
        }
    }
}
