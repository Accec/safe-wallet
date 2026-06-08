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
fn parses_evm_scan_token_transfer_html_without_running_javascript() {
    let html = r#"
        <html>
            <body>
                <script>
                    const quickExportTokentxnsData = '[{"Txhash":"0xa0f204aedeca37f9f16dde3a9efa096387ff938327f7c98a6bf688c6254898c4","Status":"Success","Method":"0x810c705b","BlockNo":"102516332","DateTime":"2026-06-05 19:59:44","Sender":"0x12b17178502c5b24d01d9a2089d2625f165acb2c","Receiver":"0xb300000b72deaeb607a12d5f54773d1c19c7028d","Amount":"3,090.10611993","Value":"$100.75","Token":"BNB Attestation(BAS)"}]';
                </script>
            </body>
        </html>
    "#;

    let records = parse_activity_response(ChainId::Bsc, html).unwrap();

    assert_eq!(records.len(), 1);
    assert_eq!(
        records[0].tx_hash,
        "0xa0f204aedeca37f9f16dde3a9efa096387ff938327f7c98a6bf688c6254898c4"
    );
    assert_eq!(records[0].kind, ActivityKind::TokenTransfer);
    assert_eq!(records[0].status, ActivityStatus::Confirmed);
    assert_eq!(
        records[0].summary,
        "Transfer 3090.10611993 BAS to 0xb300000b72deaeb607a12d5f54773d1c19c7028d"
    );
}

#[test]
fn parses_evm_scan_native_transaction_html_without_running_javascript() {
    let html = r#"
        <html>
            <body>
                <script>
                    const quickExportTransactionListData = '[{"Txhash":"0x3ed0bede558468a6410db8c5cb1fc247b914a2910a7892f6635f4174bb7f44b5","Status":"Success","Method":"0x810c705b","Blockno":"102660519","DateTime":"2026-06-06 14:02:49","Sender":"0x12b17178502c5b24d01d9a2089d2625f165acb2c","Receiver":"0xb300000b72deaeb607a12d5f54773d1c19c7028d","Amount":"0.6 BNB","Value":"$361.97","TxnFee":"0.00004844"}]';
                </script>
            </body>
        </html>
    "#;

    let records = parse_activity_response(ChainId::Bsc, html).unwrap();

    assert_eq!(records.len(), 1);
    assert_eq!(
        records[0].tx_hash,
        "0x3ed0bede558468a6410db8c5cb1fc247b914a2910a7892f6635f4174bb7f44b5"
    );
    assert_eq!(records[0].kind, ActivityKind::NativeTransfer);
    assert_eq!(records[0].status, ActivityStatus::Confirmed);
    assert_eq!(
        records[0].summary,
        "Transfer 0.6 BNB to 0xb300000b72deaeb607a12d5f54773d1c19c7028d"
    );
}

#[test]
fn parses_evm_scan_zero_value_transaction_html_as_contract_activity() {
    let html = r#"
        <html>
            <body>
                <script>
                    const quickExportTransactionListData = '[{"Txhash":"0x33910b23e16a1a2091b411596d190d22eb47ce68262bce0f833a46b34da538dc","Status":"Success","Method":"Approve","Blockno":"102660682","DateTime":"2026-06-06 14:04:02","Sender":"0x12b17178502c5b24d01d9a2089d2625f165acb2c","Receiver":"0x55d398326f99059ff775485246999027b3197955","Amount":"0 BNB","Value":"$0.00","TxnFee":"0.00000246"}]';
                </script>
            </body>
        </html>
    "#;

    let records = parse_activity_response(ChainId::Bsc, html).unwrap();

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].kind, ActivityKind::Approval);
    assert_eq!(
        records[0].summary,
        "Approve to 0x55d398326f99059ff775485246999027b3197955"
    );
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
fn parses_tronscan_transaction_api_transfer_rows() {
    let body = serde_json::json!({
        "data": [{
            "hash": "38b0e5b3d1db75907adba09fbd17be04ee395546973c7a9d4086604466afc69e",
            "toAddress": "TUxJcEDX8Srz3kYsv7oC4h3RWERhBk4QjJ",
            "ownerAddress": "TBvk3YvmPKCRNx4mbuHazZhXURodqFyfUb",
            "contractType": 1,
            "confirmed": true,
            "revert": false,
            "contractData": {
                "amount": 1,
                "owner_address": "TBvk3YvmPKCRNx4mbuHazZhXURodqFyfUb",
                "to_address": "TUxJcEDX8Srz3kYsv7oC4h3RWERhBk4QjJ"
            },
            "contractRet": "SUCCESS",
            "result": "SUCCESS",
            "amount": "1",
            "tokenInfo": {
                "tokenId": "_",
                "tokenAbbr": "trx",
                "tokenName": "trx",
                "tokenDecimal": 6,
                "tokenType": "trc10"
            }
        }]
    });

    let records = parse_activity_body(ChainId::Tron, &body).unwrap();

    assert_eq!(records.len(), 1);
    assert_eq!(
        records[0].tx_hash,
        "38b0e5b3d1db75907adba09fbd17be04ee395546973c7a9d4086604466afc69e"
    );
    assert_eq!(records[0].kind, ActivityKind::NativeTransfer);
    assert_eq!(
        records[0].summary,
        "Transfer 0.000001 TRX to TUxJcEDX8Srz3kYsv7oC4h3RWERhBk4QjJ"
    );
}

#[test]
fn skips_tronscan_transaction_api_non_transfer_rows() {
    let body = serde_json::json!({
        "data": [{
            "hash": "30ee479c215bd24124f55bdd9a4e1c76d9c09c0c51783917ae840dc66269a0a6",
            "toAddress": "TUxJcEDX8Srz3kYsv7oC4h3RWERhBk4QjJ",
            "contractType": 58,
            "confirmed": true,
            "revert": false,
            "contractData": {
                "balance": "7044000000",
                "resource": "ENERGY",
                "receiver_address": "TUxJcEDX8Srz3kYsv7oC4h3RWERhBk4QjJ"
            },
            "contractRet": "SUCCESS",
            "result": "SUCCESS",
            "amount": "0",
            "tokenInfo": {
                "tokenId": "_",
                "tokenAbbr": "trx",
                "tokenName": "trx",
                "tokenDecimal": 6
            }
        }]
    });

    let records = parse_activity_body(ChainId::Tron, &body).unwrap();

    assert!(records.is_empty());
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
