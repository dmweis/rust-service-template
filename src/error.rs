use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Generic error")]
    GenericError,
}
