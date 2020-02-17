use openssl::rsa::Rsa;
use openssl::pkey::Private;

pub fn init_encryption() -> (Rsa<Private>, Vec<u8>) {
    let rsa = Rsa::generate(2048).expect("failed to generate public key");
    let public_key = rsa.public_key_to_der().expect("failed to convert public key to DER");
    (rsa, public_key)
}