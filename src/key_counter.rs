use k256::ecdsa::signature::{Signer, Verifier};
use k256::ecdsa::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
pub struct KeyCounter {
    pub public_keys: HashMap<String, VerifyingKey>,
}
impl KeyCounter {
    pub fn new() -> Self {
        Self {
            public_keys: HashMap::new(),
        }
    }
}

pub fn generate_key_pair() -> (SigningKey, VerifyingKey) {
    let private_key = SigningKey::random(&mut OsRng);
    let public_key = VerifyingKey::from(&private_key);
    (private_key, public_key)
}

pub fn sign_message(private_key: &SigningKey, message: &str) -> Vec<u8> {
    let signature: k256::ecdsa::Signature = private_key.sign(message.as_bytes()); // explicitly specify as_bytes
    let der_signature = signature.to_der();
    der_signature.as_bytes().to_vec()
}

pub fn verify_signature(public_key: &VerifyingKey, message: &str, signature: &[u8]) -> bool {
    if let Ok(signature) = k256::ecdsa::Signature::from_der(signature) {
        return public_key.verify(message.as_bytes(), &signature).is_ok();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_verification() {
        let (private_key, public_key) = generate_key_pair();
        let message = "Hello, World!";

        // Sign the message
        let signature = sign_message(&private_key, message);

        // Verify the signature
        assert!(verify_signature(&public_key, message, &signature));
    }

    #[test]
    fn test_invalid_signature_verification() {
        let (_, public_key) = generate_key_pair();
        let message = "Hello, World!";
        let invalid_signature = vec![0; 64]; // Invalid signature with all zeros

        // Verify the invalid signature
        assert!(!verify_signature(&public_key, message, &invalid_signature));
    }
}
