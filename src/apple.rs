use std::{error::Error, fmt, process::{Command, Output}};

use crate::{ConnectionTrait, Result};

const PREFIX_ID: &str = r#"tell application "System Events""#;
const SUFFIX_ID: &str = r#"to get the unix id of every process"#;

const PREFIX_WINDOW: &str = r#"tell application "System Events""#;
const SUFFIX_WINDOW: &str = r#"to get the title of every window of every process"#;

const PERMISSION_ERROR: &str = "osascript is not allowed assistive access";

pub struct Connection;
impl ConnectionTrait for Connection {
	fn new() -> Result<Self> { Ok(Self) }
	fn window_titles(&self) -> Result<Vec<(u32,String)>> {
		let arguments = &["-ss", "-e", &format!("{} {}", PREFIX_ID, SUFFIX_ID)];
		let command_pid = Command::new("osascript").args(arguments).output()
			.expect("failed to execute AppleScript command");
			println!("OUTPUT PID: {:?}", command_pid);

		let arguments = &["-ss", "-e", &format!("{} {}", PREFIX_WINDOW, SUFFIX_WINDOW)];
		let command_window = Command::new("osascript").args(arguments).output()
			.expect("failed to execute AppleScript command");
			println!("OUTPUT WINDOWS: {:?}", command_window);

		
		
		let error_window = String::from_utf8_lossy(&command_window.stderr);
		let error_pid = String::from_utf8_lossy(&command_pid.stderr);
		match (error_window.contains(PERMISSION_ERROR), error_pid.contains(PERMISSION_ERROR)) {
			(false, false) => Ok(split(&String::from_utf8_lossy(&command_window.stdout), &String::from_utf8_lossy(&command_pid.stdout))),
			(_, _) => Err(WindowTitleError::NoAccessibilityPermission.into()),
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum WindowTitleError {
	NoAccessibilityPermission,
}
impl fmt::Display for WindowTitleError {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result<> {
		match self {
			WindowTitleError::NoAccessibilityPermission => write!(fmt, "Permission to use the accessibility API has not been granted")
		}
	}
}
impl Error for WindowTitleError {}

fn split(mut string: &str, mut pid: &str) -> Vec<(u32,String)> {
	let mut titles = Vec::new();
	let mut i = 0;
	while let (Some(start), Some(start_pid)) = (string.find('"'), pid.find(", ")) {
		let end = string[start + 1..].find('"').unwrap();
		let end_pid = pid[start_pid + 1..].find(", ").unwrap();
		titles.push((pid.split(", ").collect::<Vec<&str>>()[i].trim().parse::<u32>().unwrap(),string[start + 1..][..end].trim().replace("\"", "").to_string()));
		println!("pushed pid: {} and name: {}", pid[start_pid + 1..][..end_pid].to_string().trim().parse::<u32>().unwrap(),string[start + 1..][..end].trim().replace("\"", "").to_string());
		string = &string[start + 1..][end + 1..];
		pid = &pid[start_pid + 1..][end_pid + 1..];
		i+=1;
	}
	println!("titles: {:?}", titles);
	
	titles
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_split() {
		let string = r#"{{}, {"0"}, {"1", "2"}}"#;
		assert_eq!(split(string), &[(0,"0".to_string()), (1,"1".to_string()), (2,"2".to_string())]);
	}
}
