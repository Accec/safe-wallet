use super::super::super::mappers::*;
use super::super::NetworkRepository;
use crate::error::WalletError;
use crate::models::NetworkPrivacySettings;
use chrono::Utc;
use rusqlite::{params, OptionalExtension};

impl NetworkRepository {
    pub fn network_privacy_settings(&self) -> Result<NetworkPrivacySettings, WalletError> {
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select proxy_enabled, proxy_mode, proxy_url
                from network_privacy_settings
                where id = 1",
                [],
                |row| {
                    let proxy_enabled: i64 = row.get(0)?;
                    let proxy_mode: String = row.get(1)?;
                    Ok(NetworkPrivacySettings {
                        proxy_enabled: proxy_enabled != 0,
                        proxy_mode: db_to_proxy_mode(&proxy_mode),
                        proxy_url: row.get(2)?,
                    })
                },
            )
            .optional()
            .map_err(|_| WalletError::Storage)
            .map(|settings| {
                settings.unwrap_or_else(crate::protocol::network::default_network_privacy_settings)
            })
    }

    pub fn save_network_privacy_settings(
        &self,
        settings: &NetworkPrivacySettings,
    ) -> Result<(), WalletError> {
        let connection = self.database.connect()?;
        let now = Utc::now().to_rfc3339();
        connection
            .execute(
                "insert into network_privacy_settings (
                    id, proxy_enabled, proxy_mode, proxy_url, updated_at
                ) values (1, ?1, ?2, ?3, ?4)
                on conflict(id) do update set
                    proxy_enabled = excluded.proxy_enabled,
                    proxy_mode = excluded.proxy_mode,
                    proxy_url = excluded.proxy_url,
                    updated_at = excluded.updated_at",
                params![
                    if settings.proxy_enabled { 1 } else { 0 },
                    proxy_mode_to_db(settings.proxy_mode),
                    settings.proxy_url.as_deref(),
                    now,
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        Ok(())
    }
}
