pub mod charge_back;
pub mod deposit;
pub mod dispute;
pub mod error;
pub mod resolve;
pub mod withdrawal;
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
