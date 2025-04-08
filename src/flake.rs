use crate::epoch::EpochType;
use crate::error::TiltflakeError;
use crate::id::TiltflakeId;
use chrono::{DateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a Snowflake ID generator with a custom epoch.
///
/// A Snowflake ID is a unique identifier composed of:
/// - A timestamp (41 bits)
/// - A machine ID (10 bits)
/// - A sequence number (12 bits)
#[derive(Debug)]
pub struct Tiltflake {
	/// The machine ID (10 bits) used to identify the generator instance.
	pub machine_id: u16,
	/// The custom epoch used as the starting point for timestamp calculations.
	pub custom_epoch: DateTime<Utc>,
}

impl Tiltflake {
	/// Creates a new `Tiltflake` instance with a specified epoch type.
	///
	/// # Arguments
	/// - `machine_id`: A 16-bit machine ID, which will be masked to 10 bits.
	/// - `epoch_type`: The type of epoch to use (Unix, Discord, or Custom).
	///
	/// # Returns
	/// A new `Tiltflake` instance.
	pub(crate) fn new(machine_id: u16, epoch: &EpochType) -> Self {
		let datetime = epoch.base_datetime();

		Self {
			machine_id: machine_id & 0x3FF, // 10-bit mask
			custom_epoch: datetime,
		}
	}

	/// Generates a Snowflake ID from a given timestamp in milliseconds since UNIX epoch.
	///
	/// # Arguments
	/// - `millis`: The timestamp in milliseconds since UNIX epoch.
	/// - `sequence`: A 16-bit sequence number, which will be masked to 12 bits.
	///
	/// # Returns
	/// A 64-bit Snowflake ID.
	pub fn generate_from_unix_millis(
		&self,
		millis: u64,
		sequence: u16,
	) -> Result<TiltflakeId, TiltflakeError> {
		let delta = i64::try_from(millis).unwrap() - self.custom_epoch.timestamp_millis();

		if delta < 0 {
			return Err(TiltflakeError::TimestampBeforeEpoch);
		}

		if delta as u64 > (1 << 41) - 1 {
			return Err(TiltflakeError::TimestampTooLarge);
		}

		let timestamp = delta as u64;
		let sequence = u64::from(sequence) & 0xFFF;

		Ok(TiltflakeId(
			(timestamp << 22) | ((u64::from(self.machine_id)) << 12) | sequence,
		))
	}

	/// Generates a Snowflake ID from a `SystemTime` instance.
	///
	/// # Arguments
	/// - `time`: A `SystemTime` instance.
	/// - `sequence`: A 16-bit sequence number, which will be masked to 12 bits.
	///
	/// # Returns
	/// A 64-bit Snowflake ID.
	pub fn generate_from_system_time(
		&self,
		time: SystemTime,
		sequence: u16,
	) -> Result<TiltflakeId, TiltflakeError> {
		let millis = time
			.duration_since(UNIX_EPOCH)
			.map_err(|_| TiltflakeError::TimestampBeforeEpoch)?
			.as_millis() as u64;

		self.generate_from_unix_millis(millis, sequence)
	}

	/// Generates a Snowflake ID from a `DateTime<Utc>` instance.
	///
	/// # Arguments
	/// - `datetime`: A `DateTime<Utc>` instance.
	/// - `sequence`: A 16-bit sequence number, which will be masked to 12 bits.
	///
	/// # Returns
	/// A 64-bit Snowflake ID.
	pub fn generate_from_datetime(
		&self,
		datetime: DateTime<Utc>,
		sequence: u16,
	) -> Result<TiltflakeId, TiltflakeError> {
		self.generate_from_unix_millis(
			u64::try_from(datetime.timestamp_millis()).unwrap(),
			sequence,
		)
	}

	/// Generates a Snowflake ID from an RFC3339-formatted string.
	///
	/// # Arguments
	/// - `rfc3339`: A string slice containing an RFC3339-formatted timestamp.
	/// - `sequence`: A 16-bit sequence number, which will be masked to 12 bits.
	///
	/// # Returns
	/// A `Result` containing the 64-bit Snowflake ID or a `chrono::ParseError`.
	pub fn generate_from_rfc3339(
		&self,
		rfc3339: &str,
		sequence: u16,
	) -> Result<TiltflakeId, TiltflakeError> {
		let datetime = rfc3339.parse::<DateTime<Utc>>()?;
		self.generate_from_datetime(datetime, sequence)
	}

	/// Parses a Snowflake ID back into its components.
	///
	/// # Arguments
	/// - `id`: A 64-bit Snowflake ID.
	///
	/// # Returns
	/// A tuple containing:
	/// - `DateTime<Utc>`: The timestamp derived from the ID.
	/// - `u16`: The machine ID.
	/// - `u16`: The sequence number.
	pub fn parse(&self, id: TiltflakeId) -> (DateTime<Utc>, u16, u16) {
		let raw = id.0;
		let timestamp = raw >> 22;
		let machine_id = ((raw >> 12) & 0x3FF) as u16;
		let sequence = (raw & 0xFFF) as u16;

		let datetime = self.custom_epoch
			+ chrono::Duration::milliseconds(
				timestamp
					.try_into()
					.map_err(|_| TiltflakeError::TimestampTooLarge)
					.unwrap(),
			);
		(datetime, machine_id, sequence)
	}

	/// Parses a Snowflake ID into its components.
	///
	/// # Arguments
	/// - `id`: A 64-bit Snowflake ID.
	/// - `epoch`: The epoch type used to calculate the timestamp.
	///
	/// # Returns
	/// A tuple containing:
	/// - `DateTime<Utc>`: The timestamp derived from the ID.
	/// - `u16`: The machine ID.
	/// - `u16`: The sequence number.
	pub fn parse_id(id: u64, epoch: EpochType) -> (DateTime<Utc>, u16, u16) {
		let timestamp = id >> 22; // Extracts timestamp (41 bits)
		let machine_id = ((id >> 12) & 0x3FF) as u16; // Extracts machine_id (10 bits)
		let sequence = (id & 0xFFF) as u16; // Extracts Sequence (12 bits)

		let base_datetime = epoch.base_datetime();
		let datetime = base_datetime
			+ chrono::Duration::milliseconds(
				timestamp
					.try_into()
					.map_err(|_| TiltflakeError::TimestampTooLarge)
					.unwrap(),
			);

		(datetime, machine_id, sequence)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::epoch::EpochType;
	use chrono::{TimeZone, Utc};
	use std::time::{Duration, SystemTime};

	#[test]
	fn generate_with_unix_epoch() {
		let unix_epoch = EpochType::Unix;
		let snowflake = Tiltflake::new(1, &unix_epoch);
		let id = snowflake.generate_from_unix_millis(0, 0).unwrap(); // 1970-01-01T00:00:00Z
		let (datetime, machine_id, sequence) = snowflake.parse(id);
		assert_eq!(datetime, Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap());
		assert_eq!(machine_id, 1);
		assert_eq!(sequence, 0);
	}

	#[test]
	fn generate_with_discord_epoch() {
		let discord_epoch = EpochType::Discord;
		let snowflake = Tiltflake::new(2, &discord_epoch);
		let id = snowflake
			.generate_from_unix_millis(1420070400000, 0)
			.unwrap(); // 2015-01-01T00:00:00Z
		let (datetime, machine_id, sequence) = snowflake.parse(id);
		assert_eq!(datetime, Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap());
		assert_eq!(machine_id, 2);
		assert_eq!(sequence, 0);
	}

	#[test]
	fn new_initializes_with_masked_machine_id() {
		let custom_epoch = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
		let snowflake = Tiltflake::new(0xFFFF, &EpochType::Custom(custom_epoch));
		assert_eq!(snowflake.machine_id, 0x3FF);
		assert_eq!(snowflake.custom_epoch, custom_epoch);
	}

	#[test]
	fn generate_from_unix_millis_creates_valid_id() {
		let custom_epoch = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
		let snowflake = Tiltflake::new(1, &EpochType::Custom(custom_epoch));
		let id = snowflake
			.generate_from_unix_millis(1577836800000, 0)
			.unwrap(); // Jan 1, 2020
		let (datetime, machine_id, sequence) = snowflake.parse(id);
		assert_eq!(datetime, custom_epoch);
		assert_eq!(machine_id, 1);
		assert_eq!(sequence, 0);
	}

	#[test]
	fn generate_from_system_time_creates_valid_id() {
		let custom_epoch = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
		let snowflake = Tiltflake::new(2, &EpochType::Custom(custom_epoch));
		let system_time = SystemTime::UNIX_EPOCH + Duration::from_millis(1577836800000); // Jan 1, 2020
		let id = snowflake.generate_from_system_time(system_time, 5).unwrap();
		let (datetime, machine_id, sequence) = snowflake.parse(id);
		assert_eq!(datetime, custom_epoch);
		assert_eq!(machine_id, 2);
		assert_eq!(sequence, 5);
	}

	#[test]
	fn generate_from_rfc3339_parses_valid_id() {
		let custom_epoch = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
		let snowflake = Tiltflake::new(3, &EpochType::Custom(custom_epoch));
		let id = snowflake
			.generate_from_rfc3339("2020-01-01T00:00:00Z", 10)
			.unwrap();
		let (datetime, machine_id, sequence) = snowflake.parse(id);
		assert_eq!(datetime, custom_epoch);
		assert_eq!(machine_id, 3);
		assert_eq!(sequence, 10);
	}

	#[test]
	fn parse_correctly_extracts_components() {
		let custom_epoch = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
		let snowflake = Tiltflake::new(4, &EpochType::Custom(custom_epoch));
		let id = snowflake
			.generate_from_unix_millis(1577836800000, 15)
			.unwrap(); // Jan 1, 2020
		let (datetime, machine_id, sequence) = snowflake.parse(id);
		assert_eq!(datetime, custom_epoch);
		assert_eq!(machine_id, 4);
		assert_eq!(sequence, 15);
	}

	#[test]
	fn generate_from_unix_millis_handles_max_range() {
		let custom_epoch = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
		let snowflake = Tiltflake::new(5, &EpochType::Custom(custom_epoch));

		// Maximum timestamp within 41 bits
		let max_millis = custom_epoch.timestamp_millis() + (1i64 << 41) - 1;
		let id = snowflake
			.generate_from_unix_millis(max_millis as u64, 0)
			.unwrap();
		let (datetime, machine_id, sequence) = snowflake.parse(id);

		assert_eq!(datetime.timestamp_millis(), max_millis);
		assert_eq!(machine_id, 5);
		assert_eq!(sequence, 0);
	}
}
