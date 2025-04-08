use std::error::Error;
use tiltflake::{epoch::EpochType, flake::Tiltflake};

fn main() -> Result<(), Box<dyn Error>> {
	let generator = Tiltflake::builder()
		.with_machine_id(1)
		.with_epoch(EpochType::Discord)
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
