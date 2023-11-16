use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotDbError {
    #[error("There is no score associated with this user.")]
    NoSuchScore,
}
