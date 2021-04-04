mod application;
mod application_error;
mod configuration;
mod functions;
mod slack;

///
/// Starting configuration
///
#[derive(std::clone::Clone)]
struct StartConfigurationSettings {
	/// Path to settings.toml.
	settings_path: String,

	/// Task name to launch.
	target_task: String,
}

impl StartConfigurationSettings {
	/// Returns a new instance.
	///
	/// ### Returns
	/// A new instance of `StartConfigurationSettings`.
	pub fn new() -> std::result::Result<StartConfigurationSettings, Box<dyn std::error::Error>> {
		let mut instance = StartConfigurationSettings {
			target_task: String::new(),
			settings_path: String::new(),
		};
		if true {
			instance.configure()?;
		}
		if false {
			instance.configure2()?;
		}
		return Ok(instance);
	}

	/// Reads commandline options
	fn configure(&mut self) -> std::result::Result<(), String> {
		let mut current_option = String::new();

		// Commandline options. The 1st token is application itself.
		let args: Vec<String> = std::env::args().skip(1).collect();

		// Reading tokens
		for e in args {
			if e == "--help" || e == "-h" {
				return Err("show usage".to_string());
			}
			if e == "--version" || e == "-v" {
				return Err("show version".to_string());
			}
			if e.starts_with("--file=") || e.starts_with("-f=") {
				let (_, value) = functions::split_string(&e, "=");
				if value == "" {
					return Err("show usage".to_string());
				}
				self.settings_path = value;
				continue;
			}
			if e == "--file" || e == "-f" {
				current_option = e;
				continue;
			}
			if e.starts_with("-") {
				// Unknown option flag given.
				current_option.clear();
				println!("Unknown option {}", e);
				return Err("show usage".to_string());
			}

			if current_option == "--file" || current_option == "-f" {
				// Must be the path to rmake file.
				self.settings_path = e;
				current_option.clear();
				continue;
			}

			// Must be the name of a task to launch.
			self.target_task = e;
		}

		if current_option != "" {
			// No values followed option flag.
			return Err("show usage".to_string());
		}

		// Configuration valid.
		return Ok(());
	}

	fn configure2(&mut self) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		// Options to be served.
		let mut getopt = getopts::Options::new();
		getopt.optflag("h", "help", "");
		getopt.optopt("s", "settings", "", "");
		getopt.optopt("t", "task", "", "");

		// parsing.
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
		if result.opt_present("settings") {
			self.settings_path = result.opt_str("settings").unwrap();
		}

		// task
		if result.opt_present("task") {
			self.target_task = result.opt_str("port").unwrap();
		}

		return Ok(());
	}
}

/// Show usage.
fn usage() {
	println!("USAGE:");
	println!("    --help: Show this message.");
	println!();
}

/// Show application version.
fn version() {
	println!("{}", env!("CARGO_PKG_DESCRIPTION"));
	println!();
	println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
	println!();
	println!("{}", "https://github.com/mass10/rslack-command");
}

/// アプリケーションのエントリーポイント
fn main() {
	// コマンドラインオプション
	let result = StartConfigurationSettings::new();
	if result.is_err() {
		let error = result.err().unwrap();
		let error_message = error.to_string();
		if error_message == "show usage" {
			usage();
		} else if error_message == "show version" {
			version();
		} else {
			println!("[ERROR] Unknown error {:?}", &error_message)
		}
		return;
	}
	let conf = result.unwrap();

	// Initialize an instance of Application.
	let result = application::Application::new();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
	let app = result.unwrap();

	// Task to launch.
	let requested_task = conf.target_task;

	// アプリケーションを実行します。
	let result = app.run(&requested_task);
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
