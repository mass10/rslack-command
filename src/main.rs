mod application;
mod application_error;
mod configuration;
mod slack;

/// アプリケーションのエントリーポイント
fn main() {
	// アプリケーションのインスタンスを初期化します。
	let result = application::Application::new();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
	let app = result.unwrap();

	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	let map = std::collections::HashMap::<String, String>::new();
	let mut section = "".to_string();
	for e in &args {
		if e == "--request" {
			// section = e;
		}
	}

	// アプリケーションを実行します。
	let result = app.run(&requested_section);
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
