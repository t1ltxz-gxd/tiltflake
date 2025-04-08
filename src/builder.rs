use crate::epoch::EpochType;
use crate::flake::Tiltflake;

impl Tiltflake {
	/// Creates a new `TiltflakeBuilder` instance with default values.
	///
	/// # Returns
	/// A `TiltflakeBuilder` initialized with default settings.
	pub fn builder() -> TiltflakeBuilder {
		TiltflakeBuilder::default()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct TiltflakeBuilder {
	machine_id: u16,
	epoch: EpochType,
}

impl Default for TiltflakeBuilder {
	fn default() -> Self {
		Self {
			machine_id: 1,
			epoch: EpochType::Unix,
		}
	}
}

impl TiltflakeBuilder {
	pub const fn with_machine_id(mut self, machine_id: u16) -> Self {
		self.machine_id = machine_id;
		self
	}

	pub const fn with_epoch(mut self, epoch: EpochType) -> Self {
		self.epoch = epoch;
		self
	}

	pub fn build(self) -> Tiltflake {
		Tiltflake::new(self.machine_id, &self.epoch)
	}
}
