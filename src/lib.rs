// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Decode the hex string to bytes
    let tx_bytes = hex::decode(raw_tx_hex).map_err(|e| format!("Hex decode error: {}", e))?;
    // The version is the first 4 bytes (little-endian)
    if tx_bytes.len() < 4 {
        return Err("Transaction data too short to contain version".to_string());
    }
    let version = u32::from_le_bytes([tx_bytes[0], tx_bytes[1], tx_bytes[2], tx_bytes[3]]);
    Ok(version)
}
