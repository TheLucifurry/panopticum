use std::process::Command;

pub fn is_command_available(command: &str, arg: &str) -> bool {
  Command::new(command)
      .arg(arg)
      .output()
      .map(|output| output.status.success())
      .unwrap_or_default()
}