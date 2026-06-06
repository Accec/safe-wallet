use super::*;
use crate::models::{AssetKind, ChainId};

#[test]
fn parses_etherscan_token_holding_rows() {
    let body = serde_json::json!({
        "status": "1",
        "result": [{
            "TokenAddress": "0x1111111111111111111111111111111111111111",
            "TokenName": "Position",
            "TokenSymbol": "POSI",
            "TokenDivisor": "18",
            "TokenQuantity": "100000000000000"
        }]
    });

    let discovered = parse_discovery_body(ChainId::Bsc, &body).unwrap();

    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].kind, AssetKind::Erc20);
    assert_eq!(
        discovered[0].contract_address,
        "0x1111111111111111111111111111111111111111"
    );
    assert_eq!(discovered[0].symbol, "POSI");
    assert_eq!(discovered[0].name, "Position");
    assert_eq!(discovered[0].decimals, 18);
    assert_eq!(discovered[0].balance, "0.0001");
}

#[test]
fn parses_etherscan_token_transfer_rows_without_duplicate_contracts() {
    let body = serde_json::json!({
        "result": [
            {
                "contractAddress": "0x2222222222222222222222222222222222222222",
                "tokenName": "USD Coin",
                "tokenSymbol": "USDC",
                "tokenDecimal": "6",
                "value": "5000000"
            },
            {
                "contractAddress": "0x2222222222222222222222222222222222222222",
                "tokenName": "USD Coin",
                "tokenSymbol": "USDC",
                "tokenDecimal": "6",
                "value": "7000000"
            }
        ]
    });

    let discovered = parse_discovery_body(ChainId::Ethereum, &body).unwrap();

    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].symbol, "USDC");
    assert_eq!(discovered[0].balance, "5");
}

#[test]
fn parses_tronscan_account_token_rows() {
    let token_id = "TEST_TRON_TOKEN_ID_DO_NOT_USE";
    let body = serde_json::json!({
        "trc20token_balances": [{
            "tokenId": token_id,
            "tokenName": "Tether USD",
            "tokenAbbr": "USDT",
            "tokenDecimal": 6,
            "balance": "12500000"
        }]
    });

    let discovered = parse_discovery_body(ChainId::Tron, &body).unwrap();

    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].kind, AssetKind::Trc20);
    assert_eq!(discovered[0].contract_address, token_id);
    assert_eq!(discovered[0].symbol, "USDT");
    assert_eq!(discovered[0].balance, "12.5");
}

#[test]
fn parses_tronscan_trc20_transfer_rows_for_asset_discovery() {
    let body = serde_json::json!({
        "token_transfers": [{
            "transaction_id": "cc12dd6d9d06687ba29d810509cd8221305aaac9f98844f64102a6c114ae9b6d",
            "contract_address": "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
            "quant": "175054115",
            "tokenType2": "trc20",
            "tokenInfo": {
                "tokenId": "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
                "tokenAbbr": "USDT",
                "tokenName": "Tether USD",
                "tokenDecimal": 6,
                "tokenType": "trc20"
            }
        }]
    });

    let discovered = parse_discovery_body(ChainId::Tron, &body).unwrap();

    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].kind, AssetKind::Trc20);
    assert_eq!(
        discovered[0].contract_address,
        "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t"
    );
    assert_eq!(discovered[0].symbol, "USDT");
    assert_eq!(discovered[0].name, "Tether USD");
    assert_eq!(discovered[0].decimals, 6);
    assert_eq!(discovered[0].balance, "175.054115");
}
