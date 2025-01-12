use std::{collections::HashMap, io::{self, Write}, sync::mpsc, thread, time::Duration};

fn main() {
    let (cmd_channel_sender, cmd_channel_receiver) = mpsc::channel::<String>();
    let (data_channel_sender, data_channel_receiver) = mpsc::channel();

     thread::spawn(move || {
        let mut values: HashMap<String, String> = HashMap::new();

        let _ = data_channel_sender.send("server:started");

        for received in cmd_channel_receiver {
            if received == "cmd:ping" {
                let _ = data_channel_sender.send("server:pong");
            } else if received.starts_with("cmd:add") {
                let args: Vec<&str> = received.split_whitespace().collect();
                if args.len() != 3 {
                    println!("Invalid syntax");
                    continue;
                }

                let key = args[1].to_string();
                let value = args[2].to_string();

                values.insert(key, value);
            } else if received == "cmd:print" {
                print!("Values: ");
                for (k, v) in &values {
                    println!("{k}: {v}");
                }
                print!("");
            } else {
                println!("Unknown command: {received}");
            }
        }

        println!("Server stopped!");
    });

    let mut is_server_ready = false;

    // Main loop for processing user input and received messages
    loop {
        // Check for messages from the data channel
        while let Ok(received) = data_channel_receiver.try_recv() {
            if received == "server:started" {
                println!("Server started");
                is_server_ready = true;
            } else {
                println!("{}", received);
            }
        }

        if !is_server_ready {
            continue;
        }

        // Accept user input
        print!("Command: ");
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

        if cmd.is_empty() {
            continue; // Skip if no input is provided
        }

        // Send the input to the command channel
        let _ = cmd_channel_sender.send(cmd.clone());

        // Exit condition
        if cmd == "cmd:exit" {
            break;
        }

        thread::sleep(Duration::from_millis(1));
    }

    println!("Program terminated.");
}
