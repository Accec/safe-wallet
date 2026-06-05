use crate::error::WalletError;
use rusqlite::Connection;

mod activity;
mod assets;
mod executor;
mod indexes;
mod migrations;
mod multisig;
mod network;
mod preferences;
mod security;
mod wallets;

pub(super) fn create_schema(connection: &Connection) -> Result<(), WalletError> {
    executor::execute_schema(
        connection,
        &[
            migrations::SQL,
            security::SQL,
            wallets::SQL,
            network::SQL,
            assets::SQL,
            activity::SQL,
            multisig::SQL,
            preferences::SQL,
            indexes::SQL,
        ],
    )
}
