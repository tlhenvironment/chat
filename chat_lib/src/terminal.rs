use std::process::Command;

pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear the terminal");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear the terminal");
    }
}
