use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::Limits;
use crate::validation::ValidationError;

macro_rules! newtype {
    ($name:ident, | $raw:ident, $lim:ident | $check:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name(String);
        impl $name {
            pub fn parse($raw: String, $lim: &Limits) -> Result<Self, ValidationError> {
                $check.map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn into_string(self) -> String {
                self.0
            }
        }
        impl sqlx::Type<sqlx::Postgres> for $name {
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                <String as sqlx::Type<sqlx::Postgres>>::type_info()
            }

            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
                <String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
        }
        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for $name {
            fn decode(
                value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                <String as sqlx::Decode<sqlx::Postgres>>::decode(value).map(Self)
            }
        }
    };
}

#[allow(clippy::unwrap_used)]
static USERNAME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z0-9_-]{3,32}$").unwrap());

newtype!(Username, |raw, _limits| USERNAME_RE
    .is_match(&raw)
    .then_some(raw)
    .ok_or(ValidationError::Username));

newtype!(Password, |raw, limits| {
    let (min, max) = (limits.password_min_len, limits.password_max_len);
    (min..=max)
        .contains(&raw.chars().count())
        .then_some(raw)
        .ok_or(ValidationError::PasswordLength { min, max })
});

newtype!(Bio, |raw, limits| {
    let max = limits.bio_max_len;
    (raw.chars().count() <= max)
        .then_some(raw)
        .ok_or(ValidationError::BioLength { max })
});

newtype!(AvatarUrl, |raw, limits| {
    let max = limits.avatar_url_max_len;
    (raw.chars().count() <= max)
        .then_some(raw)
        .ok_or(ValidationError::AvatarUrlLength { max })
});
