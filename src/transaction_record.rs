use crate::error::TransactionError;

use super::transaction::{Mutation, Transfer};

#[derive(Debug, PartialEq, Eq)]
pub struct TransactionRecord {
    tx: Transfer,
    under_dispute: bool,
    charge_backed: bool,
}

impl TransactionRecord {
    pub fn new(tx: Transfer) -> Self {
        Self {
            tx,
            under_dispute: false,
            charge_backed: false,
        }
    }
    pub fn tx(&self) -> &Transfer {
        &self.tx
    }

    /// Mutates the transaction record with the provided mutation type.
    ///
    /// Returns an error if the mutation is not allowed on the transaction
    pub fn mutate(&mut self, mutation: &Mutation) -> Result<(), TransactionError> {
        match mutation {
            Mutation::Dispute(_) => {
                if !self.charge_backed {
                    self.under_dispute = true;
                } else {
                    tracing::error!(
                        "Could not dispute transaction {:?} disputed: {}, charbacked: {}",
                        self.tx,
                        self.under_dispute,
                        self.charge_backed
                    );
                    return Err(TransactionError::DisputeError);
                }
            }
            Mutation::Resolve(_) => {
                if self.under_dispute && !self.charge_backed {
                    self.under_dispute = false;
                } else {
                    tracing::error!(
                        "Could not resolve transaction {:?} disputed: {}, charbacked: {}",
                        self.tx,
                        self.under_dispute,
                        self.charge_backed
                    );
                    return Err(TransactionError::ResolveError);
                }
            }
            Mutation::ChargeBack(_) => {
                if self.under_dispute {
                    self.under_dispute = false;
                    self.charge_backed = true;
                } else {
                    tracing::error!(
                        "Could not charge back transaction {:?} disputed: {}, charbacked: {}",
                        self.tx,
                        self.under_dispute,
                        self.charge_backed
                    );
                    return Err(TransactionError::ChargeBackError);
                }
            }
        }
        Ok(())
    }
}
