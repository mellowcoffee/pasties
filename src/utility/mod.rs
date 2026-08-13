pub mod sanitize;
// pub mod validation;

use std::time::{SystemTime, UNIX_EPOCH};

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{
    PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};
use argon2::Argon2;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::Utc;
use rand::Rng;
use sha2::{Digest, Sha256};

use crate::error::UserError;

pub fn hash_password(password: &str) -> Result<String, UserError> {
    let salt = SaltString::generate(&mut OsRng);
    let phc = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(phc)
}

pub fn verify_password(
    password: &str,
    phc: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed = PasswordHash::new(phc)?;
    match Argon2::default().verify_password(password.as_bytes(), &parsed) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(e),
    }
}

pub fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn hash_token(token: &str) -> Vec<u8> {
    Sha256::digest(token.as_bytes()).to_vec()
}

pub fn datetime_now() -> chrono::DateTime<Utc> {
    #[allow(clippy::expect_used)]
    let time = i64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis(),
    )
    .expect("Timestamp exceeds i64 capacity");

    #[allow(clippy::expect_used)]
    chrono::DateTime::<Utc>::from_timestamp_millis(time)
        .expect("Milliseconds out of range")
}

#[macro_export]
macro_rules! newtype_str {
    ($name:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! serde_default {
    ($($name:ident: $ty:ty = $val:expr;)*) => {
        paste::paste! {
            $(
                const fn [<default_ $name>]() -> $ty { $val }
            )*
        }
    };
}
