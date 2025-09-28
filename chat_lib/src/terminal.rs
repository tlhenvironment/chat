use std::process::Command;

use tokio::io::{self, AsyncBufReadExt as _, BufReader};

#[derive(Debug)]
pub enum TermError {
    TermSizeError,

}

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

pub async fn capture_user_input(){
    // Capture user input asynchronously
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    loop {
        // Read the input line asynchronously
        if let Some(line) = lines.next_line().await.unwrap() {
            println!("Hello, {}!", line);
        }

        

    }
}