//!
//! Common operations.
//!

extern crate chrono;

/// Return system timestamp
///
/// # Returns
/// Timestamp as `String`
#[allow(unused)]
pub fn get_timestamp() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

/// Retrieve the whole content of file
///
/// # Returns
/// Entire content of file as `String`
pub fn read_text_file_all(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path)?;
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

/// Split string at the first separator
///
/// # Arguments
/// * `s` - String to split
/// * `separator` - Separator to split at
/// # Returns
/// The left part and the latter
pub fn split_string(s: &str, separator: &str) -> (String, String) {
	let position = s.find(separator);
	if position.is_none() {
		return ("".to_string(), "".to_string());
	}
	let position = position.unwrap();
	let key = &s[0..position];
	let value = &s[position + 1..];
	return (key.to_string(), value.to_string());
}
