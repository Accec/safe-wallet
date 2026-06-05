use super::ChainId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedPayment {
    pub chain: Option<ChainId>,
    pub address: String,
    pub amount: Option<String>,
    pub note: Option<String>,
}
