<div align="center">

[![Preview](/assets/images/hero.png)](https://github.com/t1ltxz-gxd/tiltflake)
<p>A strict, deterministic, and time-traveling Snowflake ID generator for Rust — supports timewalk generation from SystemTime, DateTime<Utc>, epoch milliseconds, or RFC3339 strings.</p>

---
[![GitHub License](https://img.shields.io/github/license/t1ltxz-gxd/tiltflake)](https://github.com/t1ltxz-gxd/tiltflake/blob/main/LICENSE "license")
[![Tests](https://img.shields.io/github/actions/workflow/status/t1ltxz-gxd/tiltflake/ci.yml?style=flat-square&logo=github&label=Tests)](https://github.com/t1ltxz-gxd/tiltflake/tests)
[![Forks](https://custom-icon-badges.demolab.com/github/forks/t1ltxz-gxd/tiltflake?logo=fork)](https://github.com/t1ltxz-gxd/tiltflake/network/members)
[![Contributors](https://custom-icon-badges.demolab.com/github/contributors/t1ltxz-gxd/tiltflake?logo=people)](https://github.com/t1ltxz-gxd/tiltflake/graphs/contributors)
[![Stars](https://custom-icon-badges.demolab.com/github/stars/t1ltxz-gxd/tiltflake?logo=star)](https://github.com/t1ltxz-gxd/tiltflake/stargazers)
[![Open issues](https://custom-icon-badges.demolab.com/github/issues-raw/t1ltxz-gxd/tiltflake?logo=issue)](https://github.com/t1ltxz-gxd/tiltflake/issues)


[![Powered by Rust](https://custom-icon-badges.herokuapp.com/badge/-Powered%20by%20Rust-0d1620?logo=rust)](https://www.rust-lang.org/ "Powered by Rust")
</div>

___

## 🧩 Installation
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
tiltflake = {version = "*", features = ["serde"]}
```

## 📖 Usage

### Generate a Snowflake ID with a UNIX epoch
```rust
use tiltflake::Tiltflake;
use std::error::Error;

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
```

### Generate a Snowflake ID with a custom epoch
```rust
use chrono::{TimeZone, Utc};
use tiltflake::{EpochType, Tiltflake};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let epoch = EpochType::Custom(
		Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).single().unwrap(),
	);
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
```

### Generate a Snowflake ID with a custom Discord epoch
```rust
use tiltflake::{EpochType, Tiltflake};
use std::error::Error;

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
```

### See all examples in the `examples` folder.

___

## 🤝 Contributing

Contributions are what make the open source community an amazing place to learn, be inspired, and create.
Any contributions you make are **greatly appreciated**.

1. [Fork the repository](https://github.com/t1ltxz-gxd/tiltflake/fork)
2. Clone your fork `git clone https://github.com/t1ltxz-gxd/tiltflake.git`
3. Create your feature branch `git checkout -b feat-smth-amazing`
4. Stage changes `git add .`
5. Commit your changes `git commit -m 'feat: add some amazing feature'`
   - Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for commit messages.
   - Use `fix`, `feat`, `docs`, `style`, `refactor`, `perf`, `test`, `chore` prefixes.
   - Use the present tense ("add feature" not "added feature").
   - Use the imperative mood ("move cursor to..." not "moves cursor to...").
   - Limit the first line to 72 characters or less.
   - Reference issues and pull requests liberally after the first line.
6. Push to the branch `git push origin feat-smth-amazing`
7. Submit a pull request

## ❤️ Credits

Released with ❤️ by [Tilt](https://github.com/t1ltxz-gxd).
