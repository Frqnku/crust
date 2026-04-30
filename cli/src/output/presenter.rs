use colored::Colorize;
use super::message::{Success, ErrorMessage};

/// Handles formatted output presentation with colors and structure
pub struct Presenter;

impl Presenter {
	/// Print a success message with structured formatting
	pub fn success(success: &Success) {
		let check = "✓".green().bold();
		let title = success.title().green().bold();
		
		println!("{} {}", check, title);
		
		for detail in success.details() {
			println!("  {} {}", "→".cyan(), detail);
		}
	}

	/// Print a batch result with mixed coloring for partial successes.
	pub fn batch_result(title: impl AsRef<str>, bounded_context: impl AsRef<str>, succeeded: &[String], failed: &[(String, String)]) {
		if succeeded.is_empty() {
			eprintln!("{} {}", "✗".red().bold(), title.as_ref().red().bold());

			for (name, error) in failed {
				eprintln!("  • {}", format!("{}: {}", name, error).red());
			}

			return;
		}

		eprintln!("{} {}", "⚠".yellow(), title.as_ref().yellow());
		eprintln!("  • {}", format!("Context: {}", bounded_context.as_ref()).bright_white());
		eprintln!("  • {}", format!("✓ Created: {}", succeeded.join(", ")).green());

		for (name, error) in failed {
			eprintln!("  • {}", format!("✗ {}: {}", name, error).red());
		}
	}

	/// Print an error message with structured formatting
	pub fn error(error: &ErrorMessage) {
		let cross = "✗".red().bold();
		let title = error.title().red().bold();
		
		eprintln!("{} {}", cross, title);
		
		for detail in error.details() {
			eprintln!("  {} {}", "•".red(), detail.bright_red());
		}
		
		if let Some(hint) = error.hint() {
			eprintln!();
			eprintln!("  {} {}", "💡".yellow(), hint.yellow());
		}
	}

	/// Print an info message
	pub fn info(message: impl AsRef<str>) {
		let info = "ℹ".cyan();
		println!("{} {}", info, message.as_ref().cyan());
	}

	/// Print a warning message
	pub fn warning(message: impl AsRef<str>) {
		let warn = "⚠".yellow();
		eprintln!("{} {}", warn, message.as_ref().yellow());
	}

	/// Print a progress/step message
	pub fn step(step_num: usize, message: impl AsRef<str>) {
		let step = format!("[{}/3]", step_num).cyan().bold();
		println!("{} {}", step, message.as_ref().bright_white());
	}

	/// Print a separator line
	pub fn separator() {
		println!("{}", "─".repeat(60).dimmed());
	}
}
