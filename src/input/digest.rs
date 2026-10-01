use sha2::{Digest, Sha256};

pub(crate) fn sha256_prefixed(bytes: &[u8]) -> String {
    format!("sha256:{}", sha256_hex(bytes))
}

/// Receipt seeds use bare hexadecimal plan digests for protocol compatibility.
pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    lower_hex(&Sha256::digest(bytes))
}

pub(crate) fn receipt_id(parts: &[&str; 6]) -> String {
    let mut digest = Sha256::new();
    for value in parts {
        let length = u64::try_from(value.len()).unwrap_or(u64::MAX);
        digest.update(length.to_be_bytes());
        digest.update(value.as_bytes());
    }
    format!("receipt-{}", &lower_hex(&digest.finalize())[..24])
}

fn lower_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{receipt_id, sha256_hex, sha256_prefixed};

    #[test]
    fn digests_are_stable_and_length_delimited() {
        assert_eq!(sha256_prefixed(b"abc").len(), 71);
        assert_eq!(sha256_hex(b"abc").len(), 64);
        let first = receipt_id(&["ab", "c", "", "", "", ""]);
        let second = receipt_id(&["a", "bc", "", "", "", ""]);
        assert_ne!(first, second);
        assert_eq!(first.len(), 32);
    }
}
