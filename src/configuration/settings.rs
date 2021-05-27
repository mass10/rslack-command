//!
//! Settings
//!

use super::super::util::functions;

type ConfigurationSettingsMap = std::collections::BTreeMap<String, SettingSection>;

/// Parse TOML file.
///
/// ### Arguments.
/// `path` path to file.
fn read_toml_file(path: &str) -> std::result::Result<ConfigurationSettingsMap, Box<dyn std::error::Error>> {
	extern crate toml;
	let content = functions::read_text_file_all(path)?;
	let conf: ConfigurationSettingsMap = toml::from_str(&content)?;
	return Ok(conf);
}

/// Definitions of each section.
///
/// ### Definition
/// * `access_token`
/// * `channel`
/// * `text`
/// * `file`
/// * `file_title`
#[derive(serde_derive::Deserialize, std::fmt::Debug)]
pub struct SettingSection {
	pub access_token: Option<String>,
	pub channel: Option<String>,
	pub text: Option<String>,
	pub file: Option<String>,
	pub file_title: Option<String>,
}

///
/// Configuration settings.
///
#[derive(serde_derive::Deserialize, std::fmt::Debug)]
pub struct ConfigurationSettings {
	settings: ConfigurationSettingsMap,
}

impl ConfigurationSettings {
	/// 新しいインスタンスを返します。
	pub fn new() -> std::result::Result<ConfigurationSettings, Box<dyn std::error::Error>> {
		let mut instance = ConfigurationSettings {
			settings: ConfigurationSettingsMap::new(),
		};
		instance.configure_default("")?;
		instance.configure_envs()?;
		return Ok(instance);
	}

	/// Find task. Returns the only task when no name was given.
	pub fn get_task(&self, name: &str) -> Option<&SettingSection> {
		if name == "" && self.settings.len() == 1 {
			return self.get_first_task();
		}
		return self.settings.get(name);
	}

	/// Returns the first task.
	///
	/// ### Remarks
	/// Item order is not guaranteed.
	fn get_first_task(&self) -> Option<&SettingSection> {
		for (_, value) in &self.settings {
			return Some(&value);
		}
		return None;
	}

	/// default configuration.
	///
	/// ### Arguments.
	/// `path` path to `settings.toml`.
	fn configure_default(&mut self, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// Reading TOML.
		let path = if path == "" { "settings.toml" } else { path };
		self.settings = read_toml_file(path)?;
		return Ok(());
	}

	/// Configuration with environment variables.
	fn configure_envs(&self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		return Ok(());
	}
}

impl std::fmt::Display for ConfigurationSettings {
	/// Implements default behavior as [std::fmt::Display].
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		return write!(f, "");
	}
}
