pub(in crate::protocol::transactions) fn left_pad_32(value: &[u8]) -> Vec<u8> {
    let mut padded = vec![0_u8; 32_usize.saturating_sub(value.len())];
    padded.extend_from_slice(value);
    padded
}
