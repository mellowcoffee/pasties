use thiserror::Error;

pub mod page;
pub mod user;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("username must be between 3 and 32 characters long and only contain alphanumeric characters, dashes and underscores")]
    Username,
    #[error("password must be between {min} and {max} characters long")]
    PasswordLength { min: usize, max: usize },
    #[error("bio must be shorter than {max} characters")]
    BioLength { max: usize },
    #[error("avatar url must be shorter than {max} characters")]
    AvatarUrlLength { max: usize },
    #[error("slug must be between 3 and 64 characters long and only contain alphanumeric characters, dashes and underscores")]
    Slug,
    #[error("HTML content must be less than {max} bytes")]
    HtmlLength { max: usize },
    #[error("CSS content must be less than {max} bytes")]
    CssLength { max: usize },
}

#[macro_export]
macro_rules! newtype {
    ($name:ident($type:ty), |$raw:ident| $check:expr) => {
        newtype!(@struct $name, $type);
        impl $name {
            pub fn parse($raw: $type) -> Result<Self, ValidationError> {
                $check.map(Self)
            }
            newtype!(@accessors $type);
        }
    };
    ($name:ident($type:ty), |$raw:ident, $lim:ident| $check:expr) => {
        newtype!(@struct $name, $type);
        impl $name {
            pub fn parse($raw: $type, $lim: &Limits) -> Result<Self, ValidationError> {
                $check.map(Self)
            }
            newtype!(@accessors $type);
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
    }
}
