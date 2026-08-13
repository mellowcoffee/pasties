use std::sync::LazyLock;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::Limits;
use crate::error::UserError;
use crate::newtype;
use crate::validation::ValidationError;

#[allow(clippy::unwrap_used)]
static USERNAME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9_-]{3,32}$").unwrap());

newtype!(Username(String), |raw| USERNAME_RE
    .is_match(&raw)
    .then_some(raw)
    .ok_or(ValidationError::Username));

newtype!(Password(String), |raw, limits| {
    let (min, max) = (limits.password_min_len, limits.password_max_len);
    (min..=max)
        .contains(&raw.chars().count())
        .then_some(raw)
        .ok_or(ValidationError::PasswordLength { min, max })
});

impl Password {
    pub fn hash(&self) -> Result<String, UserError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(self.0.as_bytes(), &salt)?
            .to_string();
        Ok(hash)
    }

    // TODO: Something more descriptive than a `bool`
    pub fn verify(&self, hash: &str) -> Result<bool, UserError> {
        let parsed = PasswordHash::new(hash)?;
        match Argon2::default().verify_password(self.0.as_bytes(), &parsed) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(e)?,
        }
    }
}

newtype!(Bio(String), |raw, limits| {
    let max = limits.bio_max_len;
    (raw.chars().count() <= max)
        .then_some(raw)
        .ok_or(ValidationError::BioLength { max })
});

newtype!(AvatarUrl(String), |raw, limits| {
    let max = limits.avatar_url_max_len;
    (raw.chars().count() <= max)
        .then_some(raw)
        .ok_or(ValidationError::AvatarUrlLength { max })
});
