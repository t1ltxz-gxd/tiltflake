use std::error::Error;
use tiltflake::Tiltflake;

fn main() -> Result<(), Box<dyn Error>> {
	let generator = Tiltflake::builder().build(); // Default machine_id is 1 and epoch is Unix
	let id = generator.generate_from_rfc3339("2025-04-08T12:00:00Z", 0)?;
	println!("Snowflake ID: {}", id);

	let (dt, machine_id, seq) = generator.parse(id);
	println!(
		"Parsed: {}, machine_id={}, sequence={}",
		dt, machine_id, seq
	);

	Ok(())
}
