use super::functions;

type ConfigurationSettingsMap = std::collections::BTreeMap<String, SettingSection>;

/// TOML ファイルを解析します。
fn read_toml_file(path: &str) -> std::result::Result<ConfigurationSettingsMap, Box<dyn std::error::Error>> {
	extern crate toml;
	let content = functions::read_text_file_all(path)?;
	let conf: ConfigurationSettingsMap = toml::from_str(&content)?;
	return Ok(conf);
}

#[derive(serde_derive::Deserialize, std::fmt::Debug)]
pub struct SettingSection {
	pub access_token: Option<String>,
	pub channel: Option<String>,
	pub text: Option<String>,
	pub file: Option<String>,
	pub file_title: Option<String>,
}

/// コンフィギュレーション構造体
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
		instance.configure()?;
		return Ok(instance);
	}

	/// Find task. Returns the only task when no name was given.
	pub fn get_task(&self, name: &str) -> Option<&SettingSection> {
		if name == "" && self.settings.len() == 1 {
			for (_, value) in &self.settings {
				return Some(&value);
			}
		}
		return self.settings.get(name);
	}

	/// コンフィギュレーションを行います。
	fn configure_default(&mut self, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// TOML ファイル読み込み
		let path = if path == "" { "settings.toml" } else { path };
		self.settings = read_toml_file(path)?;
		return Ok(());
	}

	fn configure(&self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		return Ok(());
	}
}

/// [std::fmt::Display] としての振る舞いを実装します。
impl std::fmt::Display for ConfigurationSettings {
	/// 規定の操作をインプリメントします。
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		return write!(f, "");
	}
}
