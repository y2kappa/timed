#[derive(Error, Debug)]
pub enum TimedError {
    #[error("Tracing can only be initialized once")]
    TracingInitializationFailed,
    #[error("Tracing finish failed: {0}")]
    TracingFinishFailed(String),
}
