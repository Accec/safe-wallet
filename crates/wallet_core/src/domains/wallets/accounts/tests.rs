use super::*;

const MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[test]
fn derives_one_account_per_supported_chain() {
    let wallet_id = Uuid::new_v4();
    let accounts = derive_default_accounts(wallet_id, MNEMONIC).unwrap();
    assert!(accounts.iter().any(|account| account.chain == ChainId::Btc));
    assert!(accounts
        .iter()
        .any(|account| account.chain == ChainId::Ethereum));
    assert!(accounts.iter().any(|account| account.chain == ChainId::Bsc));
    assert!(accounts
        .iter()
        .any(|account| account.chain == ChainId::Polygon));
    assert!(accounts
        .iter()
        .any(|account| account.chain == ChainId::Arbitrum));
    assert!(accounts
        .iter()
        .any(|account| account.chain == ChainId::Optimism));
    assert!(accounts
        .iter()
        .any(|account| account.chain == ChainId::Tron));
}

#[test]
fn derives_required_paths_and_address_shapes() {
    let wallet_id = Uuid::new_v4();
    let accounts = derive_default_accounts(wallet_id, MNEMONIC).unwrap();

    let btc = account_for(&accounts, ChainId::Btc);
    assert_eq!(btc.wallet_id, wallet_id);
    assert_eq!(btc.derivation_path, "m/84'/0'/0'/0/0");
    assert!(btc.address.starts_with("bc1"));

    let ethereum = account_for(&accounts, ChainId::Ethereum);
    assert_eq!(ethereum.derivation_path, "m/44'/60'/0'/0/0");
    assert!(ethereum.address.starts_with("0x"));

    for chain in [
        ChainId::Bsc,
        ChainId::Polygon,
        ChainId::Arbitrum,
        ChainId::Optimism,
    ] {
        let account = account_for(&accounts, chain);
        assert_eq!(account.derivation_path, "m/44'/60'/0'/0/0");
        assert_eq!(account.address, ethereum.address);
    }

    let tron = account_for(&accounts, ChainId::Tron);
    assert_eq!(tron.derivation_path, "m/44'/195'/0'/0/0");
    assert!(tron.address.starts_with('T'));
}

#[test]
fn derives_known_addresses_for_fixture_mnemonic() {
    let accounts = derive_default_accounts(Uuid::nil(), MNEMONIC).unwrap();

    assert_eq!(
        account_for(&accounts, ChainId::Btc).address,
        "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu"
    );
    assert_eq!(
        account_for(&accounts, ChainId::Ethereum).address,
        "0x9858effd232b4033e47d90003d41ec34ecaeda94"
    );
    assert_eq!(
        account_for(&accounts, ChainId::Tron).address,
        "TUEZSdKsoDHQMeZwihtdoBiN46zxhGWYdH"
    );
}

#[test]
fn derives_known_accounts_for_private_key() {
    let accounts = derive_private_key_accounts(
        Uuid::nil(),
        "0x0000000000000000000000000000000000000000000000000000000000000001",
    )
    .unwrap();

    assert_eq!(accounts.len(), 7);
    assert_eq!(
        account_for(&accounts, ChainId::Ethereum).address,
        "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf"
    );
    assert_eq!(
        account_for(&accounts, ChainId::Bsc).address,
        account_for(&accounts, ChainId::Ethereum).address
    );
    assert!(account_for(&accounts, ChainId::Btc)
        .address
        .starts_with("bc1"));
    assert!(account_for(&accounts, ChainId::Tron)
        .address
        .starts_with('T'));
}

fn account_for(accounts: &[Account], chain: ChainId) -> &Account {
    accounts
        .iter()
        .find(|account| account.chain == chain)
        .unwrap()
}
