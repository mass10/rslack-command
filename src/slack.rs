extern crate reqwest;

/// テキストファイル全体を読み込んで返します。
///
/// ### Arguments
/// * `path` ファイルへのパス
///
/// ### Returns
/// ファイル全体の内容を返します。
pub fn read_text_file(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path).unwrap();
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

/// JSON をパースします。
///
/// ### Arguments
/// * `json_text` JSON 文字列
///
/// ### Returns
/// `serde_json::Value`
pub fn parse_json_to_value(json_text: &str) -> std::option::Option<serde_json::Value> {
	let result = serde_json::from_str::<serde_json::Value>(json_text);
	if result.is_err() {
		return None;
	}
	return Some(result.ok().unwrap());
}

/// ファイル、あるいはディレクトリーの名前部分を返します。
///
/// ### Arguments
/// * `path` ファイルのパス
///
/// ### Returns
/// ファイル名
pub fn get_file_name(path: &str) -> String {
	let file = std::path::Path::new(path);
	return file.file_name().unwrap().to_str().unwrap().to_string();
}

fn select(left: &str, right: &str) -> String {
	return if left != "" { left.to_string() } else { right.to_string() };
}

///
/// コンフィギュレーション
///
pub struct Configuration {
	/// Slack BOT の access_token
	pub access_token: String,
}

impl Configuration {
	/// コンフィギュレーションオブジェクトを初期化します。
	///
	/// ### Returns
	/// `Configuration` の新しいインスタンスを返します。
	pub fn new() -> std::result::Result<Configuration, Box<dyn std::error::Error>> {
		let conf = Configuration { access_token: String::new() };
		return Ok(conf);
	}

	/// コンフィギュレーションを行います。
	pub fn configure(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// テキストファイル全体を読み込み
		let content = read_text_file(".settings.json")?;

		// JSON をパース
		let settings = parse_json_to_value(&content);
		if settings.is_none() {
			return Ok(());
		}
		let settings = settings.unwrap();

		// Slack BOT Access Token
		let access_token = settings["access_token"].as_str().unwrap();
		self.access_token = access_token.to_string();

		return Ok(());
	}
}

///
/// アプリケーション
///
pub struct SlackClient {
	conf: std::option::Option<Configuration>,
}

impl SlackClient {
	/// アプリケーションのインスタンスを返します。
	///
	/// ### Returns
	/// `SlackClient` の新しいインスタンス
	pub fn new() -> std::result::Result<SlackClient, Box<dyn std::error::Error>> {
		let app = SlackClient { conf: None };
		return Ok(app);
	}

	/// コンフィギュレーション
	///
	/// ### Returns
	/// 内部で保持している `Configuration` への参照
	pub fn configure(&mut self) -> std::result::Result<Box<&Configuration>, Box<dyn std::error::Error>> {
		if self.conf.is_some() {
			let conf = self.conf.as_ref().unwrap();
			return Ok(Box::new(conf));
		}
		let mut conf = Configuration::new()?;
		conf.configure()?;
		self.conf = Some(conf);
		let conf = self.conf.as_ref().unwrap();
		return Ok(Box::new(conf));
	}

	/// テキストメッセージを投稿します。
	///
	/// ### Arguments
	/// * `channel` チャネル
	/// * `text` コメント
	pub fn post_text(&mut self, channel: &str, text: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// コンフィギュレーション
		let conf = self.configure()?;

		// multipart/form-data を作成
		let form = reqwest::multipart::Form::new().text("text", text.to_string()).text("channel", channel.to_string());

		// リクエスト送信
		let access_token_header = format!("Bearer {}", conf.access_token);
		let client = reqwest::Client::new();
		let url = "https://slack.com/api/chat.postMessage";
		let mut response = client
			.post(url)
			.header("Content-Type", "multipart/form-data")
			.header("Authorization", access_token_header)
			.multipart(form)
			.send()?;

		// 応答
		let content = response.text()?;
		println!("{}", content);

		// JSON を分解してフィールドを読み取る場合
		if true {
			let value = serde_json::from_str::<serde_json::Value>(content.as_str())?;
			println!("[TRACE] {}", value);
			println!("[TRACE] {}", value["error"].as_str().unwrap_or_default());
			println!("[TRACE] {:?}", value["ok"]);
			println!("[TRACE] {:?}", value["response_metadata"]);
		}

		return Ok(());
	}

	/// コメントを付けてファイルを投稿します。
	///
	/// ### Arguments
	/// * `channel` チャネル
	/// * `text` コメント
	/// * `path` ファイルへのパス
	/// * `file_name` ファイルの表示名(省略時はファイル名が採用されます)
	pub fn upload_file(&mut self, channel: &str, text: &str, path: &str, file_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// コンフィギュレーション
		let conf = self.configure()?;

		let file_name = select(file_name, &get_file_name(path));

		// multipart/form-data を作成
		let form = reqwest::multipart::Form::new()
			.text("initial_comment", text.to_string())
			.text("channels", channel.to_string())
			.text("title", file_name)
			.file("file", path)?;

		// リクエスト送信
		let access_token_header = format!("Bearer {}", conf.access_token);
		let client = reqwest::Client::new();
		let url = "https://slack.com/api/files.upload";
		let mut response = client
			.post(url)
			.header("Content-Type", "multipart/form-data")
			.header("Authorization", access_token_header)
			.multipart(form)
			.send()?;

		// 応答
		let content = response.text()?;
		println!("{}", content);

		// JSON を分解してフィールドを読み取る場合
		if true {
			let value = serde_json::from_str::<serde_json::Value>(content.as_str())?;
			println!("[TRACE] {}", value);
			println!("[TRACE] {}", value["error"].as_str().unwrap_or_default());
			println!("[TRACE] {:?}", value["ok"]);
			println!("[TRACE] {:?}", value["response_metadata"]);
		}

		return Ok(());
	}
}
