// use super::application_error;

use super::functions;

/// TOML ファイルを解析します。
fn read_toml_file(path: &str) -> std::result::Result<std::collections::HashMap<String, SettingSection>, Box<dyn std::error::Error>> {
	extern crate toml;
	let content = functions::read_text_file_all(path)?;
	let conf: std::collections::HashMap<String, SettingSection> = toml::from_str(&content)?;
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
	settings: std::collections::HashMap<String, SettingSection>,
}

impl ConfigurationSettings {
	/// 新しいインスタンスを返します。
	pub fn new() -> std::result::Result<ConfigurationSettings, Box<dyn std::error::Error>> {
		let mut instance = ConfigurationSettings {
			settings: std::collections::HashMap::<String, SettingSection>::new(),
		};
		instance.configure_default("")?;
		instance.configure()?;
		return Ok(instance);
	}

	pub fn get_task(&self, required_task: &str) -> Option<&SettingSection> {
		for (key, vars) in &self.settings {
			if key == required_task {
				return Some(vars);
			}
		}
		return None;
	}

	/// コンフィギュレーションを行います。
	fn configure_default(&mut self, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// TOML ファイル読み込み
		let path = if path == "" { "settings.toml" } else { path };
		let conf = read_toml_file(path)?;

		for (section, vars) in &conf {
			println!("[TRACE] {}", &section);
			println!("[TRACE] {:?}", vars);
		}

		self.settings = conf;

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
