use crate::error::WalletError;

pub(super) fn format_hex_units(hex_value: &str, decimals: u8) -> Result<String, WalletError> {
    let decimal = hex_to_decimal_string(hex_value)?;
    Ok(format_decimal_units(&decimal, decimals))
}

pub(super) fn format_decimal_units(value: &str, decimals: u8) -> String {
    let value = value.trim_start_matches('0');
    if value.is_empty() {
        return "0".to_string();
    }
    let decimals = decimals as usize;
    if decimals == 0 {
        return value.to_string();
    }
    if value.len() <= decimals {
        let mut fraction = format!("{}{}", "0".repeat(decimals - value.len()), value);
        while fraction.ends_with('0') {
            fraction.pop();
        }
        if fraction.is_empty() {
            return "0".to_string();
        }
        return format!("0.{fraction}");
    }
    let split_at = value.len() - decimals;
    let whole = &value[..split_at];
    let mut fraction = value[split_at..].to_string();
    while fraction.ends_with('0') {
        fraction.pop();
    }
    if fraction.is_empty() {
        return whole.to_string();
    }
    format!("{whole}.{fraction}")
}

fn hex_to_decimal_string(hex_value: &str) -> Result<String, WalletError> {
    let normalized = hex_value.trim_start_matches("0x");
    if normalized.is_empty() {
        return Ok("0".to_string());
    }
    let mut digits = vec![0_u8];
    for ch in normalized.chars() {
        let nibble = ch.to_digit(16).ok_or(WalletError::NetworkUnavailable)? as u8;
        let mut carry = nibble as u16;
        for digit in &mut digits {
            let value = (*digit as u16) * 16 + carry;
            *digit = (value % 10) as u8;
            carry = value / 10;
        }
        while carry > 0 {
            digits.push((carry % 10) as u8);
            carry /= 10;
        }
    }
    let decimal = digits
        .iter()
        .rev()
        .map(|digit| char::from(b'0' + *digit))
        .collect::<String>();
    Ok(decimal)
}
