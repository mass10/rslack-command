use std::collections::HashMap;
// use serde_json::Map;

use super::application_error;
use super::configuration;

///
/// テキストファイル全体を String で返します。
///
fn read_text_file_all(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;

	return Ok(content);
}

///
/// コンフィギュレーションを行います。
///
#[allow(unused)]
fn configure_with_toml(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// TOML ファイル読み込み
	#[allow(unused)]
	let path = if path == "" { "settings.toml" } else { path };
	let conf = read_toml_file(path)?;

	// [settings] の内容を表示します。
	// {
	// 	println!("--- HEADER ---");
	// 	let string_value = conf.settings.string_value.unwrap_or(String::new());
	// 	let threashold = conf.settings.integral_value.unwrap_or(0);
	// 	let attributes = conf.settings.attributes;
	// 	let boolean_attribute = conf.settings.boolean_attribute;

	// 	println!("[TRACE] boolean_attribute: [{:?}]", boolean_attribute);
	// 	println!("[TRACE] integral_value: [{}]", threashold);
	// 	println!("[TRACE] string_value: [{}]", string_value);
	// 	println!("[TRACE] attributes: [{:?}]", attributes);
	// 	println!();
	// }

	// [[groups]] の内容を表示します。
	// {
	// 	for group in conf.groups {
	// 		println!("[TRACE] {}", group.name.unwrap());
	// 		if group.members.is_some() {
	// 			for member in &group.members.unwrap() {
	// 				println!("[TRACE] {:?}", member);
	// 			}
	// 		}
	// 		println!();
	// 	}
	// }

	return Ok(());
}
/// コンフィギュレーション構造体
#[derive(serde_derive::Deserialize, std::fmt::Debug)]
pub struct ConfigurationSettings {
	settings: Option<Vec<HashMap<String, Option<String>>>>,
}

impl ConfigurationSettings {
	/// 新しいインスタンスを返します。
	pub fn new() -> std::result::Result<ConfigurationSettings, std::boxed::Box<dyn std::error::Error>> {
		// let mut instance = ConfigurationSettings { settings: None };
		let mut instance = read_toml_file("settings.toml")?;
		instance.configure_default()?;
		instance.configure()?;
		return Ok(instance);
	}

	fn configure_default(&mut self) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		return Ok(());
	}

	fn configure(&mut self) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		// コンフィギュレーション構造体
		// let mut conf = ConfigurationSettings { port: 0, server: "".to_string() };

		// 提供するコマンドラインオプションを定義しています。
		let mut getopt = getopts::Options::new();
		getopt.optflag("h", "help", "");
		getopt.optopt("s", "server", "", "");
		getopt.optopt("p", "port", "", "");

		// 解析
		let result = getopt.parse(std::env::args().skip(1));
		if result.is_err() {
			println!("{}", result.err().unwrap());
			println!("{}", getopt.usage(""));
			return Err(Box::new(application_error::ApplicationError::new("")));
		}
		let result = result.unwrap();

		// help
		if result.opt_present("help") {
			println!("{}", getopt.usage(""));
			return Err(Box::new(application_error::ApplicationError::new("")));
		}

		// server
		if result.opt_present("server") {
			// conf.server = result.opt_str("server").unwrap();
		}

		// port
		if result.opt_present("port") {
			// let result = result.opt_str("port").unwrap().parse();
			// if result.is_err() {
			// 	println!("{}", getopt.usage(""));
			// 	return Err(Box::new(application_error::ApplicationError::new("")));
			// }
			// conf.port = result.unwrap();
		}

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

///
/// TOML ファイルを解析します。
///
fn read_toml_file(path: &str) -> std::result::Result<configuration::ConfigurationSettings, Box<dyn std::error::Error>> {
	extern crate toml;

	// ファイル全体を文字列として読み込みます。
	let content = read_text_file_all(path)?;

	// toml 文字列を解析します。
	let _conf: ConfigurationSettings = toml::from_str(&content)?;
	// let conf: Option<Vec<Option<HashMap<String, Option<String>>>>> = toml::from_str(&content)?;
	let conf: HashMap<String, HashMap<String, String>> = toml::from_str(&content)?;

	println!("[TRACE] {:?}", conf);

	return Ok(_conf);
}
