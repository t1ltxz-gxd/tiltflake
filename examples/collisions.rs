use chrono::{TimeZone, Utc};
use std::collections::HashSet;
use tiltflake::{EpochType, Tiltflake};

fn main() {
	let epoch = EpochType::Custom(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap());
	let generator = Tiltflake::builder()
		.with_machine_id(42)
		.with_epoch(epoch)
		.build();

	let now = Utc::now();
	let now_ms = now.timestamp_millis();

	let mut ids = HashSet::new();

	for sequence in 0..=4095 {
		let id = generator
			.generate_from_unix_millis(now_ms as u64, sequence)
			.unwrap();
		println!("{}", id);
		if !ids.insert(id.into_inner()) {
			println!("⚠️ The conflict was found at sequence = {}", sequence);
		}
	}

	println!("✅ Unique IDs: {}", ids.len());
}
