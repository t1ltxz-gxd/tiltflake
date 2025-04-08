use chrono::{TimeZone, Utc};
use tiltflake::{epoch::EpochType, flake::Tiltflake};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let epoch = EpochType::Custom(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).single().unwrap());
	let generator = Tiltflake::builder()
		.with_epoch(epoch)
		.with_machine_id(1)
		.build();
	let id = generator.generate_from_rfc3339("2025-04-08T12:00:00Z", 0)?;
	println!("Snowflake ID: {}", id);

	let (dt, machine_id, seq) = generator.parse(id);
	println!(
		"Parsed: {}, machine_id={}, sequence={}",
		dt, machine_id, seq
	);

	Ok(())
}
