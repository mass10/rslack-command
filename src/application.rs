use super::configuration;
use super::slack;

///
/// Application.
///
pub struct Application;

impl Application {
	/// Returns a new instance of `Application`
	///
	/// ### Returns
	/// Returns a new instance of `Application`
	pub fn new() -> std::result::Result<Application, Box<dyn std::error::Error>> {
		return Ok(Application {});
	}

	/// Run application.
	pub fn run(&self, task_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// Configuration
		let conf = configuration::ConfigurationSettings::new()?;

		// Find task.
		let task = conf.get_task(task_name);
		if task.is_none() {
			println!("[ERROR] No such task [{}] defined.", task_name);
			return Ok(());
		}
		let vars = task.unwrap();

		let access_token = vars.access_token.clone().unwrap_or_default();

		// Initialize the application instance.
		let mut slack = slack::SlackClient::new(&access_token)?;

		if vars.file.is_some() {
			// Post text message with file.
			let file = vars.file.clone().unwrap_or_default();
			let channel = vars.channel.clone().unwrap_or_default();
			let text = vars.text.clone().unwrap_or_default();
			let file_title = vars.file_title.clone().unwrap_or_default();
			slack.upload_file(&channel, &text, &file, &file_title)?;
		} else {
			// Post text message.
			let channel = vars.channel.clone().unwrap_or_default();
			let text = vars.text.clone().unwrap_or_default();
			slack.post_text(&channel, &text)?;
		}

		return Ok(());
	}
}
