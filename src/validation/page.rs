use std::collections::HashSet;
use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::Limits;
use crate::newtype;
use crate::validation::ValidationError;

#[allow(clippy::unwrap_used)]
static SLUG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z0-9_-]{3,64}$").unwrap());

newtype!(Slug(String), |raw| SLUG_RE
    .is_match(&raw)
    .then_some(raw)
    .ok_or(ValidationError::Slug));

newtype!(Html(String), |raw, limits| {
    let max = limits.html_max_bytes;
    (raw.len() <= max)
        .then_some(raw)
        .ok_or(ValidationError::HtmlLength { max })
        .map(|s| {
            ammonia::Builder::default()
                .add_tags(&["img"])
                .add_tag_attributes("img", &["src", "alt", "width", "height", "title"])
                .url_schemes(HashSet::from(["https"]))
                .clean(&s)
                .to_string()
        })
});

newtype!(Css(String), |raw, limits| {
    let max = limits.css_max_bytes;
    (raw.len() <= max)
        .then_some(raw)
        .ok_or(ValidationError::CssLength { max })
});
