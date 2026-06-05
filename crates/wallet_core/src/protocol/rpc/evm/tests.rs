use super::{abi, balances, calldata};

#[test]
fn formats_evm_wei_balance() {
    assert_eq!(balances::format_evm_wei("0x0").unwrap(), "0");
    assert_eq!(
        balances::format_evm_wei("0x1158e460913d00000").unwrap(),
        "20"
    );
    assert_eq!(
        balances::format_evm_wei("0x1158e460913d0000").unwrap(),
        "1.25"
    );
}

#[test]
fn decodes_erc20_abi_values() {
    let dynamic_symbol = concat!(
        "0x",
        "0000000000000000000000000000000000000000000000000000000000000020",
        "0000000000000000000000000000000000000000000000000000000000000004",
        "5553444300000000000000000000000000000000000000000000000000000000"
    );
    let bytes32_symbol = "0x5553445400000000000000000000000000000000000000000000000000000000";
    let decimals = "0x0000000000000000000000000000000000000000000000000000000000000006";

    assert_eq!(abi::decode_abi_string(dynamic_symbol).unwrap(), "USDC");
    assert_eq!(abi::decode_abi_string(bytes32_symbol).unwrap(), "USDT");
    assert_eq!(abi::decode_abi_u8(decimals).unwrap(), 6);
    assert_eq!(
        calldata::encode_balance_of("0x0000000000000000000000000000000000000001").unwrap(),
        "0x70a082310000000000000000000000000000000000000000000000000000000000000001"
    );
}
