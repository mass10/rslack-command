use super::configuration;
use super::slack;

/// アプリケーション構造体
pub struct Application;

impl Application {
	/// 新しいインスタンスを返します。
	pub fn new() -> std::result::Result<Application, std::boxed::Box<dyn std::error::Error>> {
		return Ok(Application {});
	}

	/// アプリケーションを実行します。
	pub fn run(&self, requested_section: &str) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		// コンフィギュレーション
		let conf = configuration::ConfigurationSettings::new()?;

		// アプリケーションオブジェクトを初期化します。
		let mut slack = slack::SlackClient::new()?;

		if false {
			// テキストメッセージを投稿します。
			slack.post_text(
				"notifications",
				"テキストメッセージ\r\n`warning`\r\n:four_leaf_clover::four_leaf_clover::four_leaf_clover::four_leaf_clover::four_leaf_clover:",
			)?;

			// コメントを付けてファイルを投稿します。
			slack.upload_file("notifications", "さあうけとるがよい", "0.jpg", "")?;
		}

		return Ok(());
	}
}
