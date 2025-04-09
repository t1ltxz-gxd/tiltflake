use chrono::TimeZone;
use tiltflake::{EpochType, Tiltflake};

fn main() {
	let id: u64 = 1359135689932804096;
	let custom_epoch =
		EpochType::Custom(chrono::Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap());

	let (datetime, machine_id, sequence) = Tiltflake::parse_id(id, custom_epoch);

	println!("Parsed ID: {}", id);
	println!("Timestamp: {}", datetime);
	println!("Machine ID: {}", machine_id);
	println!("Sequence: {}", sequence);
}
