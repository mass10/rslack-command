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
	/// `tasks` Task names to launch.
	pub fn run(&self, tasks: &Vec<String>) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// Configuration
		let conf = ConfigurationSettings::new()?;

		for task_name in tasks {
			// Find task.
			let task = conf.get_task(task_name);
			if task.is_none() {
				println!("No such task [{}] defined.", task_name);
				return Ok(());
			}
			let task = task.unwrap();

			let access_token = task.access_token.clone();

			// Initialize the application instance.
			let mut slack = slack::SlackClient::new(&access_token)?;

			if task.file.is_some() {
				// Post text message with file.
				let file = task.file.clone().unwrap_or_default();
				let file_title = task.file_title.clone().unwrap_or_default();
				slack.upload_file(&task.channel, &task.text, &file, &file_title)?;
			} else {
				// Post text message.
				slack.post_text(&task.channel, &task.text)?;
			}
		}

		return Ok(());
	}
}
