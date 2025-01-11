use std::process::Command;

pub fn is_command_available(command: &str, arg: &str) -> bool {
  Command::new(command)
      .arg(arg) // Most tools respond to this argument
      .output()
      .map(|output| output.status.success())
      .unwrap_or(false) // Return false if the command fails
}