use super::*;
use crate::models::{ActivityKind, ActivityStatus, ChainId};

#[test]
fn parses_etherscan_compatible_token_transfer_results() {
    let body = serde_json::json!({
        "status": "1",
        "message": "OK",
        "result": [{
            "hash": "0xabc",
            "to": "0xreceiver",
            "value": "12500000",
            "tokenSymbol": "USDT",
            "tokenDecimal": "6"
        }]
    });

    let records = parse_activity_body(ChainId::Ethereum, &body).unwrap();

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].tx_hash, "0xabc");
    assert_eq!(records[0].kind, ActivityKind::TokenTransfer);
    assert_eq!(records[0].summary, "Transfer 12.5 USDT to 0xreceiver");
}

#[test]
fn parses_tronscan_native_transfer_results() {
    let body = serde_json::json!({
        "data": [{
            "transactionHash": "abc123",
            "amount": 1234567,
            "transferToAddress": "TReceiver",
            "confirmed": true,
            "contractRet": "SUCCESS",
            "tokenInfo": {
                "tokenAbbr": "TRX",
                "tokenDecimal": 6
            }
        }]
    });

    let records = parse_activity_body(ChainId::Tron, &body).unwrap();

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].tx_hash, "abc123");
    assert_eq!(records[0].kind, ActivityKind::NativeTransfer);
    assert_eq!(records[0].status, ActivityStatus::Confirmed);
    assert_eq!(records[0].summary, "Transfer 1.234567 TRX to TReceiver");
}

#[test]
fn parses_tronscan_trc20_transfer_results() {
    let body = serde_json::json!({
        "token_transfers": [{
            "transaction_id": "def456",
            "quant": "2500000",
            "to_address": "TReceiver",
            "confirmed": true,
            "contractRet": "SUCCESS",
            "tokenInfo": {
                "tokenAbbr": "USDT",
                "tokenDecimal": 6
            }
        }]
    });

    let records = parse_activity_body(ChainId::Tron, &body).unwrap();

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].tx_hash, "def456");
    assert_eq!(records[0].kind, ActivityKind::TokenTransfer);
    assert_eq!(records[0].status, ActivityStatus::Confirmed);
    assert_eq!(records[0].summary, "Transfer 2.5 USDT to TReceiver");
}
