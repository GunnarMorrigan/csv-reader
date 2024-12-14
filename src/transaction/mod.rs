use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
pub mod charge_back;
pub mod deposit;
pub mod dispute;
pub mod error;
pub mod resolve;
pub mod withdrawal;

pub fn transaction_reader<R>(reader: R) -> csv::DeserializeRecordsIntoIter<R, TransactionRow>
where
    R: std::io::Read,
{
    let rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(reader);
    rdr.into_deserialize()
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct TransactionRow {
    #[serde(rename = "type")]
    transaction_type: TransactionType,
    client: Client,
    #[serde(rename = "tx")]
    transaction_id: TransactionId,
    amount: Option<Decimal>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "chargeback")]
    ChargeBack,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, Hash)]
pub struct TransactionId(u32);

#[cfg(test)]
impl TransactionId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}
