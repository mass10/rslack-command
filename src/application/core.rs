use crate::configuration::ConfigurationSettings;
use crate::services::slack;

///
/// Application.
///
pub struct Application;

impl Application {
	/// Returns a new instance of `Application`
	///
	/// # Returns
	/// Returns a new instance of `Application`
	pub fn new() -> std::result::Result<Application, Box<dyn std::error::Error>> {
		return Ok(Application {});
	}

	/// Run application.
	///
	/// # Arguments
	/// `task_name` Task name to launch.
	pub fn run(&self, task_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// Configuration
		let conf = ConfigurationSettings::new()?;

		// Find task.
		let task = conf.get_task(task_name);
		if task.is_none() {
			println!("No such task [{}] defined.", task_name);
			return Ok(());
		}
		let task = task.unwrap();

		let access_token = task.access_token.clone().unwrap_or_default();

		// Initialize the application instance.
		let mut slack = slack::SlackClient::new(&access_token)?;

		if task.file.is_some() {
			// Post text message with file.
			let file = task.file.clone().unwrap_or_default();
			let channel = task.channel.clone().unwrap_or_default();
			let text = task.text.clone().unwrap_or_default();
			let file_title = task.file_title.clone().unwrap_or_default();
			slack.upload_file(&channel, &text, &file, &file_title)?;
		} else {
			// Post text message.
			let channel = task.channel.clone().unwrap_or_default();
			let text = task.text.clone().unwrap_or_default();
			slack.post_text(&channel, &text)?;
		}

		return Ok(());
	}
}
