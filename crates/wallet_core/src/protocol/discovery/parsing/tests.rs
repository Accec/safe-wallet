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
fn parses_bscscan_token_transfer_html_without_running_javascript() {
    let html = r#"
        <html>
            <body>
                <table>
                    <tr>
                        <td>
                            <a class="d-flex" href="/token/0x0e63b9c287e32a05e6b9ab8ee8df88a2760225a9?a=0x12b17178502c5b24d01d9a2089d2625f165acb2c">
                                <div title="Pieverse Token (PIEVERSE)">Pieverse Token</div>
                            </a>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <a class="d-flex" href="/token/0x0e63b9c287e32a05e6b9ab8ee8df88a2760225a9?a=0x12b17178502c5b24d01d9a2089d2625f165acb2c">
                                <div title="Pieverse Token (PIEVERSE)">Pieverse Token</div>
                            </a>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <a class="d-flex" href="/token/0x10278d2d3b0c795faeed5d86acec8aa06f0e7777?a=0x12b17178502c5b24d01d9a2089d2625f165acb2c">
                                <div title="BEP-20: Miracle (MIRACLE)">Miracle</div>
                            </a>
                        </td>
                    </tr>
                </table>
                <script>
                    const quickExportTokentxnsData = '[{"Amount":"6.249975","Token":"Pieverse Token(PIEVERSE)"},{"Amount":"2.49999","Token":"Pieverse Token(PIEVERSE)"},{"Amount":"5","Token":"BEP-20: Miracle(MIRACLE)"}]';
                </script>
            </body>
        </html>
    "#;

    let discovered = parse_discovery_response(ChainId::Bsc, html).unwrap();

    assert_eq!(discovered.len(), 2);
    assert_eq!(discovered[0].kind, AssetKind::Erc20);
    assert_eq!(
        discovered[0].contract_address,
        "0x0e63b9c287e32a05e6b9ab8ee8df88a2760225a9"
    );
    assert_eq!(discovered[0].symbol, "PIEVERSE");
    assert_eq!(discovered[0].name, "Pieverse Token");
    assert_eq!(discovered[0].decimals, 18);
    assert_eq!(discovered[0].balance, "6.249975");
    assert_eq!(
        discovered[1].contract_address,
        "0x10278d2d3b0c795faeed5d86acec8aa06f0e7777"
    );
    assert_eq!(discovered[1].symbol, "MIRACLE");
    assert_eq!(discovered[1].name, "Miracle");
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
