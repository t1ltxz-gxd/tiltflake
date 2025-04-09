//! This crate provides utilities for building, managing epochs, handling flakes, and generating unique identifiers.
#![forbid(unsafe_code)]
#![deny(
	clippy::all,
	clippy::pedantic,
	clippy::nursery,
	clippy::cargo,
	missing_docs,
	unreachable_pub,
	unused_crate_dependencies
)]
#![warn(
	rust_2018_idioms,
	rust_2021_compatibility,
	missing_debug_implementations,
	trivial_casts,
	trivial_numeric_casts,
	unused_import_braces,
	unused_qualifications
)]
#![allow(
	clippy::module_name_repetitions,
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::must_use_candidate,
	clippy::doc_markdown,
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss,
	clippy::cast_precision_loss,
	clippy::similar_names,
	clippy::struct_excessive_bools
)]
#![doc(
	html_logo_url = "https://raw.githubusercontent.com/t1ltxz-gxd/tiltflake/main/assets/images/logo.png"
)]
#![doc(
	html_favicon_url = "https://raw.githubusercontent.com/t1ltxz-gxd/tiltflake/main/assets/images/favicon.png"
)]
/// Module for building and constructing various components.
mod builder;
/// Public module for handling epoch-related functionality.
pub mod epoch;
/// Module for error definitions and handling.
mod error;
/// Public module for managing flakes and their operations.
pub mod flake;
/// Module for generating and managing unique identifiers.
mod id;
pub use {epoch::*, flake::*};
