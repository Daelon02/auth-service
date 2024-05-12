use crate::errors::Result;
use alcoholic_jwt::JWKS;
use std::fs::File;
use std::io::Read;

pub fn load_pem_cert(file_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut pem_cert = Vec::new();
    file.read_to_end(&mut pem_cert)?;
    Ok(pem_cert)
}

pub async fn fetch_jwks(uri: &str) -> Result<JWKS> {
    let res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;
    Ok(val)
}
