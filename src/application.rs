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
	pub fn run(&self, task_name: &str) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		// コンフィギュレーション
		let conf = configuration::ConfigurationSettings::new()?;

		// タスクを検索
		let task = conf.get_task(task_name);
		if task.is_none() {
			println!("[WARN] タスク {} が定義されていません。", task_name);
			return Ok(());
		}
		let vars = task.unwrap();

		let access_token = vars.access_token.clone().unwrap_or_default();

		// アプリケーションオブジェクトを初期化します。
		let mut slack = slack::SlackClient::new(&access_token)?;

		if vars.file.is_some() {
			// コメントを付けてファイルを投稿します。
			println!("[TRACE] コメントを付けてファイルを投稿します。");
			let file = vars.file.clone().unwrap_or_default();
			let channel = vars.channel.clone().unwrap_or_default();
			let text = vars.text.clone().unwrap_or_default();
			let file_title = vars.file_title.clone().unwrap_or_default();
			slack.upload_file(&channel, &text, &file, &file_title)?;
		} else {
			// テキストメッセージを投稿します。
			let channel = vars.channel.clone().unwrap_or_default();
			let text = vars.text.clone().unwrap_or_default();
			println!("[TRACE] テキストメッセージを投稿します。channel: {}, text: {}", &channel, &text);
			slack.post_text(&channel, &text)?;
		}

		return Ok(());
	}
}
