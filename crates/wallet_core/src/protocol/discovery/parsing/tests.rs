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
