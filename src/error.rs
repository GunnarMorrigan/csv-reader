use crate::transaction::TransactionId;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum TransactionError {
    #[error("Error: Account is locked")]
    AccountLocked,
    #[error("Error: Insufficient funds")]
    InsufficientFunds,
    #[error("Error: Dispute could not be processed on transaction")]
    DisputeError,
    #[error("Error: Resolve could not be processed on transaction")]
    ResolveError,
    #[error("Error: Charge back could not be processed on transaction")]
    ChargeBackError,
    #[error("Error: Duplicate transaction {0:?}")]
    DuplicateTransaction(TransactionId),
    #[error("Error: Missing transaction {0:?}")]
    MissingTransaction(TransactionId),
}
