use super::codecs::{db_to_asset_kind, db_to_chain, db_to_multisig_kind, db_to_multisig_status};
use super::errors::sql_conversion_error;
use super::primitives::{parse_datetime_sql, parse_uuid_sql};
use crate::models::{
    Asset, ChainSettings, MultisigAccount, MultisigOwner, MultisigProposal, WalletSummary,
};
use uuid::Uuid;

pub(crate) fn multisig_account_from_row(
    row: &rusqlite::Row<'_>,
) -> rusqlite::Result<MultisigAccount> {
    let id: String = row.get(0)?;
    let chain: String = row.get(2)?;
    let kind: String = row.get(3)?;
    let created_at: String = row.get(7)?;
    Ok(MultisigAccount {
        id: parse_uuid_sql(&id, 0)?,
        label: row.get(1)?,
        chain: db_to_chain(&chain).map_err(sql_conversion_error)?,
        kind: db_to_multisig_kind(&kind).map_err(sql_conversion_error)?,
        address: row.get(4)?,
        threshold: row.get::<_, i64>(5)? as u32,
        permission_id: row
            .get::<_, Option<i64>>(6)?
            .map(|permission_id| permission_id as u32),
        created_at: parse_datetime_sql(&created_at, 7)?,
    })
}

pub(crate) fn multisig_owner_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<MultisigOwner> {
    let multisig_account_id: String = row.get(0)?;
    Ok(MultisigOwner {
        multisig_account_id: parse_uuid_sql(&multisig_account_id, 0)?,
        address: row.get(1)?,
        weight: row.get::<_, i64>(2)? as u32,
    })
}

pub(crate) fn multisig_proposal_from_row(
    row: &rusqlite::Row<'_>,
) -> rusqlite::Result<MultisigProposal> {
    let id: String = row.get(0)?;
    let multisig_account_id: String = row.get(1)?;
    let chain: String = row.get(2)?;
    let status: String = row.get(7)?;
    let created_at: String = row.get(10)?;
    Ok(MultisigProposal {
        id: parse_uuid_sql(&id, 0)?,
        multisig_account_id: parse_uuid_sql(&multisig_account_id, 1)?,
        chain: db_to_chain(&chain).map_err(sql_conversion_error)?,
        to_address: row.get(3)?,
        asset_symbol: row.get(4)?,
        amount: row.get(5)?,
        payload_json: row.get(6)?,
        status: db_to_multisig_status(&status).map_err(sql_conversion_error)?,
        threshold: row.get::<_, i64>(8)? as u32,
        signature_weight: row.get::<_, i64>(9)? as u32,
        created_at: parse_datetime_sql(&created_at, 10)?,
    })
}

pub(crate) fn asset_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Asset> {
    let id: String = row.get(0)?;
    let chain: String = row.get(1)?;
    let kind: String = row.get(2)?;
    Ok(Asset {
        id: Uuid::parse_str(&id).map_err(|error| {
            rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(error),
            )
        })?,
        chain: db_to_chain(&chain).map_err(sql_conversion_error)?,
        kind: db_to_asset_kind(&kind).map_err(sql_conversion_error)?,
        symbol: row.get(3)?,
        name: row.get(4)?,
        decimals: row.get::<_, i64>(5)? as u8,
        contract_address: row.get(6)?,
        visible: row.get::<_, i64>(7)? == 1,
        balance: row.get(8)?,
    })
}

pub(crate) fn chain_settings_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ChainSettings> {
    let chain: String = row.get(0)?;
    Ok(ChainSettings {
        chain: db_to_chain(&chain).map_err(sql_conversion_error)?,
        network_name: row.get(1)?,
        chain_id: row.get(2)?,
        enabled: row.get::<_, i64>(3)? == 1,
        default_rpc_url: row.get(4)?,
        user_rpc_url: row.get(5)?,
        indexer_endpoint: row.get(6)?,
        explorer_url: row.get(7)?,
        native_symbol: row.get(8)?,
        native_decimals: row.get::<_, i64>(9)? as u8,
    })
}

pub(crate) fn wallet_summary_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<WalletSummary> {
    let id: String = row.get(0)?;
    let created_at: String = row.get(2)?;
    Ok(WalletSummary {
        id: parse_uuid_sql(&id, 0)?,
        label: row.get(1)?,
        created_at: parse_datetime_sql(&created_at, 2)?,
    })
}
