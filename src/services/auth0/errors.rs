#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("Missing client id")]
    MissingClientId,
    #[error("Missing client secret")]
    MissingClientSecret,
    #[error("Missing audience")]
    MissingAudience,
    #[error("Missing grant type")]
    MissingGrantType,
    #[error("Missing user id")]
    MissingUserId,
    #[error("Missing connection")]
    MissingConnection,
    #[error("Missing extra")]
    MissingExtra,
    #[error("Missing email")]
    MissingEmail,
    #[error("Missing password")]
    MissingPassword,
    #[error("Missing username")]
    MissingUsername,
}
