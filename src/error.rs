use chrono::ParseError;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum TiltflakeError {
	#[error("RFC3339 parse error: {0}")]
	Rfc3339ParseError(#[from] ParseError),

	#[error("Timestamp is before the custom epoch")]
	TimestampBeforeEpoch,

	#[error("Timestamp exceeds the 41-bit limit")]
	TimestampTooLarge,
}
