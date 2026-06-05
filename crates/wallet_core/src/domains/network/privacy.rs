use crate::error::WalletError;
use crate::models::NetworkPrivacySettings;
use crate::storage::WalletDatabase;

pub(super) fn privacy_settings(
    database: &WalletDatabase,
) -> Result<NetworkPrivacySettings, WalletError> {
    database.network().network_privacy_settings()
}

pub(super) fn save_privacy_settings(
    database: &WalletDatabase,
    settings: NetworkPrivacySettings,
) -> Result<(), WalletError> {
    let settings = crate::protocol::network::normalize_network_privacy_settings(settings)?;
    database.network().save_network_privacy_settings(&settings)
}
