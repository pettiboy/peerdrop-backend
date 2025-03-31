use ed25519_dalek::{Signature, VerifyingKey};

pub fn eddsa_verify_signature(message: &str, signature: &str, public_key: &str) -> bool {
    // -- message
    let message: &[u8] = message.as_bytes();

    println!("{:?}", message);

    // -- signature
    let signature_bytes = match hex::decode(signature) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    let signature_array: &[u8; 64] = signature_bytes
        .as_slice()
        .try_into()
        .expect("Slice with incorrect length");
    let reconstructed_sign = Signature::from_bytes(signature_array);

    // -- public key
    // decode pk from hex
    let public_key_bytes = hex::decode(public_key).expect("invalid hex string for secret key");

    // ensure its 32 bytes
    let verifying_key_array: &[u8; 32] = &public_key_bytes
        .try_into()
        .expect("public key must be 32 bytes");

    let verifying_key = VerifyingKey::from_bytes(verifying_key_array).expect("invalid public key");

    let is_valid_sign = verifying_key
        .verify_strict(message, &reconstructed_sign)
        .is_ok();

    return is_valid_sign;
}
