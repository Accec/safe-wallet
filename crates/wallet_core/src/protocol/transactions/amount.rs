use crate::error::WalletError;

pub(super) fn decimal_amount_to_u64(amount: &str, decimals: u8) -> Result<u64, WalletError> {
    let bytes = decimal_amount_to_be_bytes(amount, decimals)?;
    let mut value = 0_u64;
    for byte in bytes {
        value = value
            .checked_mul(256)
            .and_then(|value| value.checked_add(byte as u64))
            .ok_or(WalletError::InsufficientFunds)?;
    }
    Ok(value)
}

pub(super) fn decimal_amount_to_be_bytes(
    amount: &str,
    decimals: u8,
) -> Result<Vec<u8>, WalletError> {
    let decimal = decimal_amount_to_integer_string(amount, decimals)?;
    decimal_string_to_be_bytes(&decimal)
}

fn decimal_amount_to_integer_string(amount: &str, decimals: u8) -> Result<String, WalletError> {
    let amount = amount.trim();
    if amount.is_empty() || amount.starts_with('-') {
        return Err(WalletError::InsufficientFunds);
    }
    let mut parts = amount.split('.');
    let whole = parts.next().unwrap_or("");
    let fraction = parts.next().unwrap_or("");
    if parts.next().is_some()
        || whole.is_empty()
        || !whole.chars().all(|ch| ch.is_ascii_digit())
        || !fraction.chars().all(|ch| ch.is_ascii_digit())
    {
        return Err(WalletError::InsufficientFunds);
    }
    let decimals = decimals as usize;
    if fraction.len() > decimals && fraction[decimals..].chars().any(|ch| ch != '0') {
        return Err(WalletError::InsufficientFunds);
    }
    let fraction = &fraction[..fraction.len().min(decimals)];
    let mut value = String::with_capacity(whole.len() + decimals);
    value.push_str(whole.trim_start_matches('0'));
    value.push_str(fraction);
    value.push_str(&"0".repeat(decimals - fraction.len()));
    let value = value.trim_start_matches('0');
    if value.is_empty() {
        return Ok("0".to_string());
    }
    Ok(value.to_string())
}

fn decimal_string_to_be_bytes(value: &str) -> Result<Vec<u8>, WalletError> {
    if value == "0" {
        return Ok(Vec::new());
    }
    let mut bytes = vec![0_u8];
    for digit in value.bytes() {
        if !digit.is_ascii_digit() {
            return Err(WalletError::InsufficientFunds);
        }
        let mut carry = (digit - b'0') as u16;
        for byte in bytes.iter_mut().rev() {
            let value = (*byte as u16) * 10 + carry;
            *byte = (value & 0xff) as u8;
            carry = value >> 8;
        }
        while carry > 0 {
            bytes.insert(0, (carry & 0xff) as u8);
            carry >>= 8;
        }
    }
    while bytes.first() == Some(&0) {
        bytes.remove(0);
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::transactions::encoding::left_pad_32;

    #[test]
    fn decimal_amount_conversion_handles_native_and_token_units() {
        assert_eq!(
            hex::encode(decimal_amount_to_be_bytes("1.25", 18).unwrap()),
            "1158e460913d0000"
        );
        assert_eq!(
            hex::encode(left_pad_32(&decimal_amount_to_be_bytes("2", 6).unwrap())),
            "00000000000000000000000000000000000000000000000000000000001e8480"
        );
        assert!(decimal_amount_to_be_bytes("1.0000001", 6).is_err());
    }
}
