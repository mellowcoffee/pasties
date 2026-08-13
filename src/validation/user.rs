use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::Limits;
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
