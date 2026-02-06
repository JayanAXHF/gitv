use crate::auth::AuthProvider;
use crate::errors::AppError;

pub struct KeyringAuth {
    service: String,
}

impl KeyringAuth {
    pub fn new(service: &str) -> Result<Self, AppError> {
        Ok(Self {
            service: service.to_string(),
        })
    }
}

impl AuthProvider for KeyringAuth {
    fn get_token(&self) -> Result<String, AppError> {
        let entry = keyring::Entry::new(&self.service, "github")?;
        let token = entry.get_password()?;
        Ok(token)
    }
}
