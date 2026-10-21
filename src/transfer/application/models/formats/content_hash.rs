use sha2::{Digest, Sha256};

pub fn content_hash(parts: &[&str]) -> String {
    let joined = parts.join("\0");
    let digest: [u8; 32] = Sha256::digest(joined.as_bytes()).into();
    sha256_hex(&digest)[..16].to_owned()
}

pub(crate) fn sha256_hex(bytes: &[u8; 32]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    output
}
