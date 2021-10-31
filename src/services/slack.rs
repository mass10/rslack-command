//!
//! Operations for Slack.
//!

extern crate reqwest;

/// Returns the name of a file.
///
/// ### Arguments
/// * `path` Path to file.
///
/// ### Returns
/// File name.
pub fn get_file_name(path: &str) -> String {
	let file = std::path::Path::new(path);
	return file.file_name().unwrap().to_str().unwrap().to_string();
}

///
/// Slack client.
///
pub struct SlackClient {
	access_token: String,
}

impl SlackClient {
	/// Returns A new instance of
	///
	/// ### Returns
	/// A new instance of `SlackClient`.
	pub fn new(access_token: &str) -> std::result::Result<SlackClient, Box<dyn std::error::Error>> {
		let app = SlackClient {
			access_token: access_token.to_string(),
		};
		return Ok(app);
	}

	/// Post text message.
	///
	/// ### Arguments
	/// * `channel` channel.
	/// * `text` text message.
	pub fn post_text(&mut self, channel: &str, text: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// multipart/form-data を作成
		let form = reqwest::multipart::Form::new()
			// text message
			.text("text", text.to_string())
			// channel
			.text("channel", channel.to_string());

		// リクエスト送信
		let access_token_header = format!("Bearer {}", self.access_token);
		let client = reqwest::Client::new();
		let mut response = client
			.post("https://slack.com/api/chat.postMessage")
			.header("Content-Type", "multipart/form-data")
			.header("Authorization", access_token_header)
			.multipart(form)
			.send()?;

		// 応答
		let content = response.text()?;

		// Parse JSON and print human readable text.
		let value = serde_json::from_str::<serde_json::Value>(content.as_str())?;
		let str = serde_json::to_string_pretty(&value)?;
		println!("{}", str);

		return Ok(());
	}

	/// Post text message with file.
	///
	/// ### Arguments
	/// * `channel` channel to post.
	/// * `text` text message.
	/// * `path` path to file.
	/// * `file_name` title of file.
	pub fn upload_file(&mut self, channel: &str, text: &str, path: &str, file_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// title of file.
		let file_title = if file_name != "" { file_name.to_string() } else { get_file_name(path) };

		// multipart/form-data を作成
		let form = reqwest::multipart::Form::new()
			// text message
			.text("initial_comment", text.to_string())
			// channel
			.text("channels", channel.to_string())
			// file title
			.text("title", file_title.clone())
			// path to file
			.file("file", path)?;

		// リクエスト送信
		let access_token_header = format!("Bearer {}", self.access_token);
		let client = reqwest::Client::new();
		let mut response = client
			.post("https://slack.com/api/files.upload")
			.header("Content-Type", "multipart/form-data")
			.header("Authorization", access_token_header)
			.multipart(form)
			.send()?;

		// 応答
		let content = response.text()?;

		// Parse JSON and print human readable text.
		let value = serde_json::from_str::<serde_json::Value>(content.as_str())?;
		let str = serde_json::to_string_pretty(&value)?;
		println!("{}", str);

		return Ok(());
	}
}
