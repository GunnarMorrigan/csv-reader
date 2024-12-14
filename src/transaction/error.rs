use super::TransactionRow;

#[derive(Debug, thiserror::Error)]
pub enum DeserializationError {
    #[error("Could not further parse: {0:?}")]
    ParseError(TransactionRow),
}
