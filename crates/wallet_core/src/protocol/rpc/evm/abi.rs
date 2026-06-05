use crate::error::WalletError;

pub(super) fn decode_abi_u8(hex_value: &str) -> Result<u8, WalletError> {
    let bytes = decode_hex_result(hex_value)?;
    if bytes.len() < 32 {
        return Err(WalletError::InvalidTokenContract);
    }
    let value = parse_word_u128(&bytes[..32])?;
    u8::try_from(value).map_err(|_| WalletError::InvalidTokenContract)
}

pub(super) fn decode_abi_string(hex_value: &str) -> Result<String, WalletError> {
    let bytes = decode_hex_result(hex_value)?;
    if bytes.len() >= 64 {
        let offset = parse_word_usize(&bytes[..32])?;
        if offset + 32 <= bytes.len() {
            let length = parse_word_usize(&bytes[offset..offset + 32])?;
            let start = offset + 32;
            let end = start + length;
            if end <= bytes.len() {
                let value = String::from_utf8(bytes[start..end].to_vec())
                    .map_err(|_| WalletError::InvalidTokenContract)?;
                if !value.trim().is_empty() {
                    return Ok(value);
                }
            }
        }
    }
    if bytes.len() >= 32 {
        let value_bytes = bytes[..32]
            .iter()
            .copied()
            .take_while(|byte| *byte != 0)
            .collect::<Vec<_>>();
        let value =
            String::from_utf8(value_bytes).map_err(|_| WalletError::InvalidTokenContract)?;
        if !value.trim().is_empty() {
            return Ok(value);
        }
    }
    Err(WalletError::InvalidTokenContract)
}

fn decode_hex_result(hex_value: &str) -> Result<Vec<u8>, WalletError> {
    let normalized = hex_value.trim_start_matches("0x");
    if normalized.is_empty() {
        return Err(WalletError::InvalidTokenContract);
    }
    hex::decode(normalized).map_err(|_| WalletError::InvalidTokenContract)
}

fn parse_word_usize(word: &[u8]) -> Result<usize, WalletError> {
    let value = parse_word_u128(word)?;
    usize::try_from(value).map_err(|_| WalletError::InvalidTokenContract)
}

fn parse_word_u128(word: &[u8]) -> Result<u128, WalletError> {
    if word.len() != 32 {
        return Err(WalletError::InvalidTokenContract);
    }
    if word[..16].iter().any(|byte| *byte != 0) {
        return Err(WalletError::InvalidTokenContract);
    }
    let mut value = 0_u128;
    for byte in &word[16..] {
        value = (value << 8) | (*byte as u128);
    }
    Ok(value)
}
