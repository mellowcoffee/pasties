use thiserror::Error;

pub mod page;
pub mod user;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Username must be between 3 and 32 characters long and only contain alphanumeric characters, dashes and underscores")]
    Username,
    #[error("Password must be between {min} and {max} characters long")]
    PasswordLength { min: usize, max: usize },
    #[error("Bio must be shorter than {max} characters")]
    BioLength { max: usize },
    #[error("Avatar url must be shorter than {max} characters")]
    AvatarUrlLength { max: usize },
    #[error("Slug must be between 3 and 64 characters long and only contain alphanumeric characters, dashes and underscores")]
    Slug,
    #[error("HTML content must be less than {max} bytes")]
    HtmlLength { max: usize },
    #[error("CSS content must be less than {max} bytes")]
    CssLength { max: usize },
}

#[macro_export]
macro_rules! newtype {
    ($name:ident(String), |$raw:ident| $check:expr) => {
        $crate::newtype!(@struct $name, String);
        $crate::newtype!(@impl $name, String, |$raw| $check);
        $crate::newtype!(@display $name);
    };
    ($name:ident(String), |$raw:ident, $lim:ident| $check:expr) => {
        $crate::newtype!(@struct $name, String);
        $crate::newtype!(@impl $name, String, |$raw, $lim| $check);
        $crate::newtype!(@display $name);
    };

    ($name:ident($type:ty), |$raw:ident| $check:expr) => {
        $crate::newtype!(@struct $name, $type);
        $crate::newtype!(@impl $name, $type, |$raw| $check);
    };
    ($name:ident($type:ty), |$raw:ident, $lim:ident| $check:expr) => {
        $crate::newtype!(@struct $name, $type);
        $crate::newtype!(@impl $name, $type, |$raw, $lim| $check);
    };

    (@impl $name:ident, $type:ty, |$raw:ident| $check:expr) => {
        impl $name {
            pub fn parse($raw: $type) -> Result<Self, ValidationError> {
                $check.map(Self)
            }
            $crate::newtype!(@accessors $type);
        }
    };
    (@impl $name:ident, $type:ty, |$raw:ident, $lim:ident| $check:expr) => {
        impl $name {
            pub fn parse($raw: $type, $lim: &Limits) -> Result<Self, ValidationError> {
                $check.map(Self)
            }
            $crate::newtype!(@accessors $type);
        }
    };
    (@struct $name:ident, $type:ty) => {
        #[derive(
            Debug, Clone, PartialEq, Eq,
            Serialize, Deserialize, sqlx::Type,
        )]
        #[sqlx(transparent)]
        pub struct $name($type);
    };
    (@accessors $type:ty) => {
        pub fn into_inner(self) -> $type { self.0 }
        pub fn as_inner(&self) -> &$type { &self.0 }
    };
    (@display $name:ident) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}
