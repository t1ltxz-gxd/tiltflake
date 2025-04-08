use chrono::{DateTime, TimeZone, Utc};

/// Represents the type of epoch for the Snowflake generator.
///
/// - `Unix`: Represents the UNIX epoch starting at `1970-01-01T00:00:00Z`.
/// - `Discord`: Represents the Discord epoch starting at `2015-01-01T00:00:00Z`.
/// - `Custom(DateTime<Utc>)`: Represents a custom epoch defined by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpochType {
	/// Represents the UNIX epoch starting at `1970-01-01T00:00:00Z`.
	Unix, // UNIX epoch (1970-01-01T00:00:00Z)
	/// Represents the Discord epoch starting at `2015-01-01T00:00:00Z`.
	Discord, // Discord epoch (2015-01-01T00:00:00Z)
	/// Represents a custom epoch defined by the user.
	/// The `DateTime<Utc>` specifies the starting point of the custom epoch.
	Custom(DateTime<Utc>), // Custom epoch
}

impl EpochType {
	/// Returns the base `DateTime<Utc>` corresponding to the `EpochType`.
	///
	/// # Examples
	///
	/// - For `EpochType::Unix`, it returns `1970-01-01T00:00:00Z`.
	/// - For `EpochType::Discord`, it returns `2015-01-01T00:00:00Z`.
	/// - For `EpochType::Custom`, it returns the custom epoch value.
	pub fn base_datetime(&self) -> DateTime<Utc> {
		match self {
			Self::Unix => Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap(),
			Self::Discord => Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap(),
			Self::Custom(custom_epoch) => *custom_epoch,
		}
	}
}
