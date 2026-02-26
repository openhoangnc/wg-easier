use rand::RngCore;
use x25519_dalek::{StaticSecret, PublicKey};
use base64::{Engine as _, engine::general_purpose::STANDARD};

/// Generate a WireGuard keypair.
/// Returns `(private_key_base64, public_key_base64)`.
pub fn generate_keypair() -> (String, String) {
    let private = StaticSecret::random_from_rng(&mut rand::thread_rng());
    let public = PublicKey::from(&private);
    let private_b64 = STANDARD.encode(private.as_bytes());
    let public_b64 = STANDARD.encode(public.as_bytes());
    (private_b64, public_b64)
}

/// Generate a WireGuard preshared key (32 random bytes, base64-encoded).
pub fn generate_preshared_key() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    STANDARD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{Engine as _, engine::general_purpose::STANDARD};

    #[test]
    fn test_keypair_format() {
        let (private, public) = generate_keypair();
        let priv_bytes = STANDARD.decode(&private).expect("valid base64 private");
        let pub_bytes = STANDARD.decode(&public).expect("valid base64 public");
        assert_eq!(priv_bytes.len(), 32, "private key must be 32 bytes");
        assert_eq!(pub_bytes.len(), 32, "public key must be 32 bytes");
    }

    #[test]
    fn test_preshared_key_format() {
        let psk = generate_preshared_key();
        let bytes = STANDARD.decode(&psk).expect("valid base64 psk");
        assert_eq!(bytes.len(), 32, "preshared key must be 32 bytes");
    }

    #[test]
    fn test_keypair_uniqueness() {
        let (priv1, _) = generate_keypair();
        let (priv2, _) = generate_keypair();
        assert_ne!(priv1, priv2, "each keypair should be unique");
    }
}
