use crate::error::WalletError;
use crate::models::{ActivityRecord, ChainId};
use serde_json::Value;

mod common;
mod etherscan;
mod tronscan;

pub(super) fn parse_activity_body(
    chain: ChainId,
    body: &Value,
) -> Result<Vec<ActivityRecord>, WalletError> {
    let rows = common::activity_rows(body)?;
    let mut records = Vec::new();
    for row in rows {
        if let Some(record) = common::parse_summary_row(chain, row) {
            records.push(record);
            continue;
        }

        let record = if chain == ChainId::Tron {
            tronscan::parse_activity_row(chain, row)
        } else {
            etherscan::parse_activity_row(chain, row)
        };
        if let Some(record) = record {
            records.push(record);
        }
    }
    Ok(records)
}

#[cfg(test)]
mod tests;
