use crate::error::AppError;
use base64::Engine;
use rand::rngs::OsRng;
use rand::TryRngCore;

pub fn new(size: &'static usize) -> Result<String, AppError> {
    let mut random_bytes = vec![0u8; *size];
    OsRng.try_fill_bytes(&mut random_bytes).map_err(|e| {
        tracing::error!(err=?e, "failed to generate random bytes for session token");
        anyhow::Error::from(e)
    })?;

    let token = base64::engine::general_purpose::URL_SAFE.encode(&random_bytes);

    Ok(token)
}
