mod activity;
mod app;
mod assets;
mod auth;
mod command;
mod multisig;
mod network;
mod support;
mod transfers;
mod wallets;

pub(crate) use command::WalletCommand;

use crate::response::WalletResponse;

pub(crate) fn handle_command(command: WalletCommand) -> WalletResponse {
    match command {
        WalletCommand::AppStatus { db_path } => app::status(db_path),
        WalletCommand::GenerateMnemonic => wallets::generate_mnemonic(),
        WalletCommand::SetMasterPassword { db_path, password } => {
            auth::set_master_password(db_path, password)
        }
        WalletCommand::UnlockApp { db_path, password } => auth::unlock_app(db_path, password),
        WalletCommand::SetDuressPassword {
            db_path,
            master_password,
            duress_password,
        } => auth::set_duress_password(db_path, master_password, duress_password),
        WalletCommand::UpdateBiometricUnlock {
            db_path,
            password,
            enabled,
        } => auth::update_biometric_unlock(db_path, password, enabled),
        WalletCommand::LockApp { db_path } => auth::lock_app(db_path),
        WalletCommand::ListWallets { db_path } => wallets::list_wallets(db_path),
        WalletCommand::ListAccounts { db_path, wallet_id } => {
            wallets::list_accounts(db_path, wallet_id)
        }
        WalletCommand::CreateWallet {
            db_path,
            label,
            mnemonic,
            password,
        }
        | WalletCommand::ImportWallet {
            db_path,
            label,
            mnemonic,
            password,
        } => wallets::create_wallet(db_path, label, mnemonic, password),
        WalletCommand::ImportPrivateKey {
            db_path,
            label,
            private_key,
            password,
        } => wallets::import_private_key(db_path, label, private_key, password),
        WalletCommand::ImportKeystore {
            db_path,
            label,
            keystore_json,
            keystore_password,
            password,
        } => wallets::import_keystore(db_path, label, keystore_json, keystore_password, password),
        WalletCommand::RevealMnemonic {
            db_path,
            wallet_id,
            password,
        } => wallets::reveal_mnemonic(db_path, wallet_id, password),
        WalletCommand::ExportKeystore {
            db_path,
            wallet_id,
            password,
        } => wallets::export_keystore(db_path, wallet_id, password),
        WalletCommand::DeleteWallet {
            db_path,
            wallet_id,
            password,
        } => wallets::delete_wallet(db_path, wallet_id, password),
        WalletCommand::ListAssets { db_path, wallet_id } => assets::list_assets(db_path, wallet_id),
        WalletCommand::RefreshAssets {
            db_path,
            wallet_id,
            chain,
        } => assets::refresh_assets(db_path, wallet_id, chain),
        WalletCommand::DiscoverAssets {
            db_path,
            wallet_id,
            chain,
        } => assets::discover_assets(db_path, wallet_id, chain),
        WalletCommand::AddCustomToken {
            db_path,
            chain,
            contract_address,
            token_name,
        } => assets::add_custom_token(db_path, chain, contract_address, token_name),
        WalletCommand::RemoveCustomToken { db_path, asset_id } => {
            assets::remove_custom_token(db_path, asset_id)
        }
        WalletCommand::GetNetworkPrivacySettings { db_path } => {
            network::get_privacy_settings(db_path)
        }
        WalletCommand::SaveNetworkPrivacySettings {
            db_path,
            proxy_enabled,
            proxy_mode,
            proxy_url,
        } => network::save_privacy_settings(db_path, proxy_enabled, proxy_mode, proxy_url),
        WalletCommand::TestProxyConnection {
            db_path,
            proxy_enabled,
            proxy_mode,
            proxy_url,
        } => network::test_proxy(db_path, proxy_enabled, proxy_mode, proxy_url),
        WalletCommand::ListNetworkSettings { db_path } => network::list_settings(db_path),
        WalletCommand::SaveNetworkSettings {
            db_path,
            network_name,
            rpc_url,
            chain_id,
            currency_symbol,
            block_explorer_url,
            indexer_endpoint,
        } => network::save_settings(
            db_path,
            network_name,
            rpc_url,
            chain_id,
            currency_symbol,
            block_explorer_url,
            indexer_endpoint,
        ),
        WalletCommand::UpdateChainRpc {
            db_path,
            chain,
            rpc_url,
        } => network::update_chain_rpc(db_path, chain, rpc_url),
        WalletCommand::UpdateIndexerSettings {
            db_path,
            chain,
            endpoint,
            api_key,
        } => network::update_indexer_settings(db_path, chain, endpoint, api_key),
        WalletCommand::ParsePaymentUri { payload } => transfers::parse_payment_uri(payload),
        WalletCommand::PreviewTransfer {
            db_path,
            request_json,
        } => transfers::preview_transfer(db_path, request_json),
        WalletCommand::SendTransfer {
            db_path,
            request_json,
            password,
        } => transfers::send_transfer(db_path, request_json, password),
        WalletCommand::ListActivity { db_path, wallet_id } => {
            activity::list_activity(db_path, wallet_id)
        }
        WalletCommand::SyncActivity {
            db_path,
            wallet_id,
            chain,
        } => activity::sync_activity(db_path, wallet_id, chain),
        WalletCommand::ImportMultisigAccount {
            db_path,
            label,
            chain,
            kind,
            address,
            threshold,
            permission_id,
            owners,
        } => multisig::import_account(
            db_path,
            label,
            chain,
            kind,
            address,
            threshold,
            permission_id,
            owners,
        ),
        WalletCommand::ListMultisigAccounts { db_path } => multisig::list_accounts(db_path),
        WalletCommand::CreateMultisigProposal {
            db_path,
            multisig_account_id,
            to_address,
            asset_symbol,
            amount,
        } => multisig::create_proposal(
            db_path,
            multisig_account_id,
            to_address,
            asset_symbol,
            amount,
        ),
        WalletCommand::ListMultisigProposals {
            db_path,
            multisig_account_id,
        } => multisig::list_proposals(db_path, multisig_account_id),
        WalletCommand::AddMultisigSignature {
            db_path,
            proposal_id,
            owner_address,
            signature,
        } => multisig::add_signature(db_path, proposal_id, owner_address, signature),
    }
}
