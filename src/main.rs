//!
//! Application entrypoint.
//!

mod application;
mod configuration;
mod services;
mod util;

///
/// Starting configuration
///
#[derive(std::clone::Clone)]
struct StartConfigurationSettings {
	/// Path to settings.toml.
	settings_path: String,

	/// Name of the tasks to launch.
	target_tasks: Vec<String>,
}

impl StartConfigurationSettings {
	/// Returns a new instance.
	///
	/// ### Returns
	/// A new instance of `StartConfigurationSettings`.
	pub fn new() -> std::result::Result<StartConfigurationSettings, Box<dyn std::error::Error>> {
		let mut instance = StartConfigurationSettings {
			target_tasks: vec![],
			settings_path: String::new(),
		};
		instance.configure()?;
		return Ok(instance);
	}

	/// Configures the instance.
	fn configure(&mut self) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		// Options to be served.
		let mut getopt = getopts::Options::new();
		getopt.optflag("h", "help", "Shows usage.");
		getopt.opt("v", "version", "Shows version info.", "STRING", getopts::HasArg::No, getopts::Occur::Optional);
		getopt.opt("", "file", "Path to settings.toml", "FILE", getopts::HasArg::Yes, getopts::Occur::Optional);

		// parse.
		let result = getopt.parse(std::env::args().skip(1));
		if result.is_err() {
			println!("{}", result.err().unwrap());
			println!("{}", getopt.usage(""));
			return Err("".into());
		}
		let result = result.unwrap();

		// optional: help
		if result.opt_present("help") {
			println!("{}", getopt.usage(""));
			return Err("".into());
		}

		// optional: Path to settings.toml
		if result.opt_present("file") {
			self.settings_path = result.opt_str("file").unwrap();
		}

		// optional: version
		if result.opt_present("version") {
			return Err("SHOW_VERSION".into());
		}

		// required: target tasks
		if result.free.len() == 0 {
			return Err("SHOW_USAGE".into());
		}
		self.target_tasks = result.free;

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

/// Rust application entrypoint.
fn main() {
	// Command line arguments.
	let result = StartConfigurationSettings::new();
	if result.is_err() {
		let error = result.err().unwrap();
		let error_message = error.to_string();
		if error_message == "" {
			// NOP
		} else if error_message == "SHOW_USAGE" {
			usage();
		} else if error_message == "SHOW_VERSION" {
			version();
		} else {
			println!("[ERROR] Unknown error {:?}", &error_message);
			std::process::exit(1);
		}
		return;
	}
	let conf = result.unwrap();

	// Initialize an instance of Application.
	let result = application::core::Application::new();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
	let application = result.unwrap();

	// Run.
	let result = application.run(&conf.target_tasks);
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
