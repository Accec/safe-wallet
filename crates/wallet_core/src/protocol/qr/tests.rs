use super::*;
use crate::error::WalletError;
use crate::models::ChainId;

#[test]
fn parses_bare_evm_address() {
    let parsed = parse_payment_uri("0x0000000000000000000000000000000000000000").unwrap();
    assert_eq!(parsed.chain, None);
    assert_eq!(parsed.address, "0x0000000000000000000000000000000000000000");
}

#[test]
fn parses_bitcoin_uri_with_amount() {
    let parsed =
        parse_payment_uri("bitcoin:bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu?amount=0.001")
            .unwrap();
    assert_eq!(parsed.chain, Some(ChainId::Btc));
    assert_eq!(parsed.amount.as_deref(), Some("0.001"));
}

#[test]
fn parses_tron_address() {
    let parsed = parse_payment_uri("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7").unwrap();
    assert_eq!(parsed.chain, Some(ChainId::Tron));
}

#[test]
fn parses_bitcoin_uri_message_as_note() {
    let parsed =
        parse_payment_uri("bitcoin:bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu?message=Invoice123")
            .unwrap();

    assert_eq!(parsed.chain, Some(ChainId::Btc));
    assert_eq!(parsed.note.as_deref(), Some("Invoice123"));
}

#[test]
fn parses_ethereum_uri_value_and_message() {
    let parsed = parse_payment_uri(
        "ethereum:0x0000000000000000000000000000000000000000?value=1000000000000000&message=Rent",
    )
    .unwrap();

    assert_eq!(parsed.chain, Some(ChainId::Ethereum));
    assert_eq!(parsed.amount.as_deref(), Some("1000000000000000"));
    assert_eq!(parsed.note.as_deref(), Some("Rent"));
}

#[test]
fn rejects_unknown_or_invalid_payloads() {
    assert_eq!(
        parse_payment_uri("dogecoin:DExample").unwrap_err(),
        WalletError::InvalidAddress
    );
    assert_eq!(
        parse_payment_uri("ethereum:0xnot-an-address").unwrap_err(),
        WalletError::InvalidAddress
    );
}

#[test]
fn rejects_unsupported_required_bitcoin_parameters() {
    let error = parse_payment_uri(
        "bitcoin:bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu?amount=0.1&req-something=x",
    )
    .unwrap_err();

    assert_eq!(error, WalletError::InvalidAddress);
}

#[test]
fn decodes_query_values() {
    let parsed = parse_payment_uri(
        "ethereum:0x0000000000000000000000000000000000000000?value=1&message=June%20Rent%20%26%20Utilities",
    )
    .unwrap();

    assert_eq!(parsed.note.as_deref(), Some("June Rent & Utilities"));
}

#[test]
fn rejects_malformed_query_encoding() {
    let error =
        parse_payment_uri("ethereum:0x0000000000000000000000000000000000000000?message=Bad%ZZ")
            .unwrap_err();

    assert_eq!(error, WalletError::InvalidAddress);
}
