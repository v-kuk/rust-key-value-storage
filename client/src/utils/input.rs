use std::io::{self, Write};

pub fn get_line(input_text: &str) -> String {
    // Accept user input
    print!("{input_text}");
    let _ = io::stdout().flush(); // Flush to ensure prompt is printed
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd).expect("Failed to read input");

    // Trim newline and carriage return characters
    if let Some('\n') = cmd.chars().next_back() {
        cmd.pop();
    }
    if let Some('\r') = cmd.chars().next_back() {
        cmd.pop();
    }

    return cmd;
}