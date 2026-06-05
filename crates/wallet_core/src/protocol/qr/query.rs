use crate::error::WalletError;

pub(super) fn parse_query(
    query: &str,
    amount_key: &str,
    reject_required_keys: bool,
) -> Result<(Option<String>, Option<String>), WalletError> {
    let mut amount = None;
    let mut note = None;

    for part in query.split('&').filter(|part| !part.is_empty()) {
        let (key, value) = part.split_once('=').unwrap_or((part, ""));
        if reject_required_keys && key.starts_with("req-") {
            return Err(WalletError::InvalidAddress);
        }

        let value = percent_decode(value)?;
        match key {
            key if key == amount_key => amount = Some(value),
            "message" => note = Some(value),
            _ => {}
        }
    }

    Ok((amount, note))
}

fn percent_decode(value: &str) -> Result<String, WalletError> {
    let mut bytes = Vec::with_capacity(value.len());
    let input = value.as_bytes();
    let mut index = 0;

    while index < input.len() {
        match input[index] {
            b'%' => {
                if index + 2 >= input.len() {
                    return Err(WalletError::InvalidAddress);
                }
                let high = hex_value(input[index + 1])?;
                let low = hex_value(input[index + 2])?;
                bytes.push((high << 4) | low);
                index += 3;
            }
            b'+' => {
                bytes.push(b' ');
                index += 1;
            }
            byte => {
                bytes.push(byte);
                index += 1;
            }
        }
    }

    String::from_utf8(bytes).map_err(|_| WalletError::InvalidAddress)
}

fn hex_value(byte: u8) -> Result<u8, WalletError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(WalletError::InvalidAddress),
    }
}
