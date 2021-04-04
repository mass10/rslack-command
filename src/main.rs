mod application;
mod application_error;
mod configuration;
mod functions;
mod slack;

///
/// Commandline options
///
#[derive(Clone)]
struct StartConfigurationSettings {
	/// Path to rmake file
	rmakefile_path: String,

	/// Task name to execute
	target_task: String,
}

impl StartConfigurationSettings {
	/// Reads commandline options
	pub fn configure() -> std::result::Result<StartConfigurationSettings, String> {
		// start configuration
		let mut conf = StartConfigurationSettings {
			target_task: String::new(),
			rmakefile_path: String::new(),
		};

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
				conf.rmakefile_path = value;
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
				conf.rmakefile_path = e;
				current_option.clear();
				continue;
			}

			// Must be the name of a task to launch.
			conf.target_task = e;
		}

		if current_option != "" {
			// No values followed option flag.
			return Err("show usage".to_string());
		}

		// Configuration valid.
		return Ok(conf);
	}

	fn _configure(&mut self) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
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

///
fn usage() {}

///
fn version() {}

/// アプリケーションのエントリーポイント
fn main() {
	// コマンドラインオプション
	let result = StartConfigurationSettings::configure();
	if result.is_err() {
		let result_string = result.err().unwrap();
		if result_string == "show usage" {
			usage();
		} else if result_string == "show version" {
			version();
		}
		return;
	}

	let _conf = result.unwrap();

	// アプリケーションのインスタンスを初期化します。
	let result = application::Application::new();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
	let app = result.unwrap();

	// 実行するタスク []
	let requested_task = _conf.target_task;

	// アプリケーションを実行します。
	let result = app.run(&requested_task);
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
