use rustls::{ServerConfig, Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;


// just used openssl to generate a pair of pem files cert and key for testing
pub fn load_ssl_config(cert_path: &str, key_path: &str) -> Arc<ServerConfig> {
    let certs = load_certs(cert_path);
    let key = load_key(key_path);

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("Failed to create server config");

    Arc::new(config)
}

fn load_certs(path: &str) -> Vec<Certificate> {
    let cert_file = File::open(path).expect("Failed to open certificate file");
    let mut reader = BufReader::new(cert_file);
    certs(&mut reader)
        .expect("Failed to load certificates")
        .into_iter()
        .map(Certificate)
        .collect()
}

fn load_key(path: &str) -> PrivateKey {
    let key_file = File::open(path).expect("Failed to open key file");
    let mut reader = BufReader::new(key_file);
    let keys = pkcs8_private_keys(&mut reader).expect("Failed to load private key");
    PrivateKey(keys.into_iter().next().expect("No key found in file"))
}
