use crate::error::AppError;
use base64::Engine;
use rand::rngs::OsRng;
use rand::TryRngCore;

pub fn new(size: usize) -> Result<String, AppError> {
    if size == 0 {
        return Err(anyhow::anyhow!("size must be greater than 0").into());
    }

    let mut random_bytes = vec![0u8; size];
    OsRng.try_fill_bytes(&mut random_bytes).map_err(|e| {
        tracing::error!(err=?e, "failed to generate random bytes for session token");
        anyhow::Error::from(e)
    })?;

    let token = base64::engine::general_purpose::URL_SAFE.encode(&random_bytes);

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::new;

    #[test]
    fn test_new_session_token() {
        let token = new(32).unwrap();
        assert_eq!(token.len(), 44); // Base64 URL-safe encoding increases the length
        assert!(token.chars().all(|c| c.is_ascii()));
    }

    #[test]
    fn test_new_session_token_empty() {
        let result = new(0);
        assert!(result.is_err());
    }
}
