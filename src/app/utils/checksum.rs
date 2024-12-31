use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use rand::Rng;
use sha2::{Digest, Sha256};

pub fn generate_hash() -> String {
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
    let mut hasher = Sha256::new();
    hasher.update(random_bytes);
    hex::encode(hasher.finalize())
}

fn obfuscate_bytes(bytes: &mut [u8]) {
    let mut prev: u8 = 165;
    for (idx, byte) in bytes.iter_mut().enumerate() {
        let old_value = *byte;
        *byte = (old_value ^ prev).wrapping_add((idx % 256) as u8);
        prev = *byte;
    }
}

pub fn generate_checksum(device_id: &str, mac_addr: Option<&str>) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        / 1_000_000;

    let mut timestamp_bytes = vec![
        ((timestamp >> 40) & 255) as u8,
        ((timestamp >> 32) & 255) as u8,
        ((timestamp >> 24) & 255) as u8,
        ((timestamp >> 16) & 255) as u8,
        ((timestamp >> 8) & 255) as u8,
        (255 & timestamp) as u8,
    ];

    obfuscate_bytes(&mut timestamp_bytes);
    let encoded = BASE64.encode(&timestamp_bytes);

    match mac_addr {
        Some(mac) => format!("{}{}/{}", encoded, device_id, mac),
        None => format!("{}{}", encoded, device_id),
    }
}
