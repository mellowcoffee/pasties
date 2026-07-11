use std::sync::LazyLock;

use regex::Regex;
use thiserror::Error;
use crate::newtype_str;
use serde::{Serialize, Deserialize};

use crate::config::Limits;

#[derive(Debug, Error)]
pub enum ValidationError {
    // Users
    #[error("username must be 3-32 characters of [a-z0-9_-]")]
    Username,
    #[error("password length must be between {min} and {max} characters")]
    PasswordLength { min: usize, max: usize },
    #[error("passwords do not match")]
    PasswordMismatch,
    #[error("bio exceeds {max} characters")]
    BioLength { max: usize },
    #[error("avatar URL invalid, exceeds {max} characters, or not http(s)")]
    AvatarUrl { max: usize },

    // Pages
    #[error("slug must be 3-64 characters of [a-z0-9_-]")]
    Slug,
    #[error("page html exceeds {max} bytes")]
    HtmlLength { max: usize },
    #[error("page css exceeds {max} bytes")]
    CssLength { max: usize },
}

// Regexes
static USERNAME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z0-9_-]{3,32}$").unwrap());
static SLUG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z0-9_-]{3,64}$").unwrap());

// Constraint functions
pub fn check_regex(re: &Regex, s: &str, err: ValidationError) -> Result<(), ValidationError> {
    if re.is_match(s) { Ok(()) } else { Err(err) }
}

pub fn check_max_bytes(s: &str, max: usize, err: ValidationError) -> Result<(), ValidationError> {
    if s.len() <= max { Ok(()) } else { Err(err) }
}

pub fn check_max_chars(s: &str, max: usize, err: ValidationError) -> Result<(), ValidationError> {
    if s.chars().count() <= max {
        Ok(())
    } else {
        Err(err)
    }
}

pub fn check_min_chars(s: &str, min: usize, err: ValidationError) -> Result<(), ValidationError> {
    if s.chars().count() >= min {
        Ok(())
    } else {
        Err(err)
    }
}

pub fn check_char_range(s: &str, min: usize, max: usize, err: ValidationError) -> Result<(), ValidationError> {
    let n = s.chars().count();
    if n >= min && n <= max {
        Ok(())
    } else {
        Err(err)
    }
}

pub fn check_http_url(s: &str, max_len: usize, err: ValidationError) -> Result<(), ValidationError> {
    if s.is_empty() { return Ok(()); }
    match url::Url::parse(s) {
        Ok(u) if matches!(u.scheme(), "http" | "https") && s.len() <= max_len => Ok(()),
        _ => Err(err),
    }
}

// Validators
newtype_str!(Username);
newtype_str!(Password);
newtype_str!(Slug);
newtype_str!(Bio);
newtype_str!(AvatarUrl);

impl Username {
    pub fn parse(raw: String) -> Result<Self, ValidationError> {
        check_regex(&USERNAME_RE, &raw, ValidationError::Username)?;
        Ok(Username(raw))
    }
}

impl Slug {
    pub fn parse(raw: String) -> Result<Self, ValidationError> {
        check_regex(&SLUG_RE, &raw, ValidationError::Slug)?;
        Ok(Slug(raw))
    }
}

impl Password {
    pub fn parse_with(raw: String, limits: &Limits) -> Result<Self, ValidationError> {
        check_char_range(
            &raw,
            limits.password_min_len,
            limits.password_max_len,
            ValidationError::PasswordLength {
                min: limits.password_min_len,
                max: limits.password_max_len,
            },
        )?;
        Ok(Password(raw))
    }
}

impl Bio {
    pub fn parse_with(raw: String, limits: &Limits) -> Result<Self, ValidationError> {
        check_max_chars(
            &raw,
            limits.bio_max_len,
            ValidationError::BioLength {
                max: limits.bio_max_len,
            },
        )?;
        Ok(Bio(raw))
    }
}

impl AvatarUrl {
    pub fn parse_with(raw: String, limits: &Limits) -> Result<Self, ValidationError> {
        check_http_url(
            &raw,
            limits.avatar_url_max_len,
            ValidationError::AvatarUrl {
                max: limits.avatar_url_max_len,
            },
        )?;
        Ok(AvatarUrl(raw))
    }
}
