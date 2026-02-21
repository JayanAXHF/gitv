use crate::{auth::AuthProvider, errors::AppError};

pub struct EnvAuth;
impl EnvAuth {
    const ENV_VAR: &'static str = "GH_TOKEN";
}
impl AuthProvider for EnvAuth {
    fn get_token(&self) -> Result<String, AppError> {
        std::env::var(Self::ENV_VAR).map_err(|_| {
            AppError::Other(anyhow::anyhow!(
                "{} environment variable not set",
                Self::ENV_VAR
            ))
        })
    }

    fn set_token(&self, token: &str) -> Result<(), AppError> {
        // Safety: This is safe because the env variable is only read once, so there is
        // threat of a race condition.
        unsafe {
            std::env::set_var(Self::ENV_VAR, token);
        }
        Ok(())
    }
}
