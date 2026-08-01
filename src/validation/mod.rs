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
}
