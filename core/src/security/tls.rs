//! TLS/SSL configuration for secure connections

use rustls::{ServerConfig, ClientConfig, pki_types::CertificateDer};
use std::fs;
use std::path::Path;
use std::io::Cursor;

pub fn create_server_config(
    cert_path: &Path,
    key_path: &Path,
) -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let cert_file = fs::File::open(cert_path)?;
    let key_file = fs::File::open(key_path)?;

    let mut cert_reader = Cursor::new(fs::read(cert_path)?);
    let mut key_reader = Cursor::new(fs::read(key_path)?);

    let certs = load_certificates(&mut cert_reader)?;
    let key = load_private_key(&mut key_reader)?;

    let config = ServerConfig::builder()
        .with_single_cert(certs, key)?;

    Ok(config)
}

pub fn create_client_config() -> Result<ClientConfig, Box<dyn std::error::Error>> {
    let config = ClientConfig::builder()
        .with_native_roots()?
        .with_no_client_auth();

    Ok(config)
}

pub fn load_certificates(
    reader: &mut dyn std::io::Read,
) -> Result<Vec<CertificateDer<'static>>, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    
    let certs = rustls_pemfile::certs(&mut Cursor::new(buf))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|c| CertificateDer::from(c.as_ref().to_vec()))
        .collect();

    Ok(certs)
}

pub fn load_private_key(
    reader: &mut dyn std::io::Read,
) -> Result<rustls::pki_types::PrivateKeyDer<'static>, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let keys = rustls_pemfile::private_key(&mut Cursor::new(buf))?;
    
    Ok(match keys {
        rustls_pemfile::Item::Pkcs1Key(key) => rustls::pki_types::PrivateKeyDer::Pkcs1(key),
        rustls_pemfile::Item::Pkcs8Key(key) => rustls::pki_types::PrivateKeyDer::Pkcs8(key),
        rustls_pemfile::Item::Sec1Key(key) => rustls::pki_types::PrivateKeyDer::Sec1(key),
        _ => return Err("Unsupported key format".into()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config_creation() {
        assert!(create_client_config().is_ok());
    }
}
