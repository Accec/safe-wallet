use serde::Deserialize;
use wallet_core::models::MultisigOwnerDraft;

#[derive(Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub(crate) enum WalletCommand {
    AppStatus {
        db_path: Option<String>,
    },
    GenerateMnemonic,
    SetMasterPassword {
        db_path: String,
        password: String,
    },
    UnlockApp {
        db_path: String,
        password: String,
    },
    SetDuressPassword {
        db_path: String,
        master_password: String,
        duress_password: String,
    },
    UpdateBiometricUnlock {
        db_path: String,
        password: String,
        enabled: bool,
    },
    LockApp {
        db_path: String,
    },
    ListWallets {
        db_path: String,
    },
    ListAccounts {
        db_path: String,
        wallet_id: String,
    },
    CreateWallet {
        db_path: String,
        label: String,
        mnemonic: String,
        password: String,
    },
    ImportWallet {
        db_path: String,
        label: String,
        mnemonic: String,
        password: String,
    },
    ImportPrivateKey {
        db_path: String,
        label: String,
        private_key: String,
        password: String,
    },
    ImportKeystore {
        db_path: String,
        label: String,
        keystore_json: String,
        keystore_password: String,
        password: String,
    },
    RevealMnemonic {
        db_path: String,
        wallet_id: String,
        password: String,
    },
    ExportKeystore {
        db_path: String,
        wallet_id: String,
        password: String,
    },
    DeleteWallet {
        db_path: String,
        wallet_id: String,
        password: String,
    },
    ListAssets {
        db_path: String,
        wallet_id: String,
    },
    RefreshAssets {
        db_path: String,
        wallet_id: String,
        chain: Option<String>,
    },
    DiscoverAssets {
        db_path: String,
        wallet_id: String,
        chain: Option<String>,
    },
    AddCustomToken {
        db_path: String,
        chain: String,
        contract_address: String,
        token_name: Option<String>,
    },
    RemoveCustomToken {
        db_path: String,
        asset_id: String,
    },
    GetNetworkPrivacySettings {
        db_path: String,
    },
    SaveNetworkPrivacySettings {
        db_path: String,
        proxy_enabled: bool,
        proxy_mode: String,
        proxy_url: Option<String>,
    },
    TestProxyConnection {
        #[allow(dead_code)]
        db_path: String,
        proxy_enabled: bool,
        proxy_mode: String,
        proxy_url: Option<String>,
    },
    ListNetworkSettings {
        db_path: String,
    },
    SaveNetworkSettings {
        db_path: String,
        network_name: String,
        rpc_url: String,
        chain_id: String,
        currency_symbol: String,
        block_explorer_url: Option<String>,
        indexer_endpoint: Option<String>,
    },
    ParsePaymentUri {
        payload: String,
    },
    PreviewTransfer {
        db_path: String,
        request_json: String,
    },
    SendTransfer {
        db_path: String,
        request_json: String,
        password: String,
    },
    ListActivity {
        db_path: String,
        wallet_id: String,
    },
    SyncActivity {
        db_path: String,
        wallet_id: String,
        chain: Option<String>,
    },
    ImportMultisigAccount {
        db_path: String,
        label: String,
        chain: String,
        kind: String,
        address: String,
        threshold: u32,
        permission_id: Option<u32>,
        owners: Vec<MultisigOwnerDraft>,
    },
    ListMultisigAccounts {
        db_path: String,
    },
    CreateMultisigProposal {
        db_path: String,
        multisig_account_id: String,
        to_address: String,
        asset_symbol: String,
        amount: String,
    },
    ListMultisigProposals {
        db_path: String,
        multisig_account_id: Option<String>,
    },
    AddMultisigSignature {
        db_path: String,
        proposal_id: String,
        owner_address: String,
        signature: String,
    },
    UpdateChainRpc {
        db_path: String,
        chain: String,
        rpc_url: String,
    },
    UpdateIndexerSettings {
        db_path: String,
        chain: String,
        endpoint: String,
        api_key: Option<String>,
    },
}
