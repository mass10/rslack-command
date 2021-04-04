extern crate reqwest;

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

///
/// アプリケーション
///
pub struct SlackClient {
	access_token: String,
}

impl SlackClient {
	/// アプリケーションのインスタンスを返します。
	///
	/// ### Returns
	/// `SlackClient` の新しいインスタンス
	pub fn new(access_token: &str) -> std::result::Result<SlackClient, Box<dyn std::error::Error>> {
		let app = SlackClient {
			access_token: access_token.to_string(),
		};
		return Ok(app);
	}

	/// テキストメッセージを投稿します。
	///
	/// ### Arguments
	/// * `channel` チャネル
	/// * `text` コメント
	pub fn post_text(&mut self, channel: &str, text: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// multipart/form-data を作成
		let form = reqwest::multipart::Form::new()
			// テキストメッセージ
			.text("text", text.to_string())
			// チャネル
			.text("channel", channel.to_string());

		// リクエスト送信
		let access_token_header = format!("Bearer {}", self.access_token);
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
		let file_name = if file_name != "" { file_name.to_string() } else { get_file_name(path) };

		// multipart/form-data を作成
		let form = reqwest::multipart::Form::new()
			// テキスト
			.text("initial_comment", text.to_string())
			// チャネル
			.text("channels", channel.to_string())
			// ファイルタイトル
			.text("title", file_name.clone())
			// ファイル
			.file("file", path)?;

		// リクエスト送信
		let access_token_header = format!("Bearer {}", self.access_token);
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
