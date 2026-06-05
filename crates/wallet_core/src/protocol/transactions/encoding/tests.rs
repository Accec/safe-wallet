use super::encode_erc20_transfer;

#[test]
fn encodes_erc20_transfer_calldata() {
    let data = encode_erc20_transfer("0x0000000000000000000000000000000000000001", "1", 6).unwrap();

    assert_eq!(&hex::encode(&data)[..8], "a9059cbb");
    assert!(hex::encode(data)
        .ends_with("00000000000000000000000000000000000000000000000000000000000f4240"));
}
