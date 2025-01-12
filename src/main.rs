mod server;

use std::{sync::mpsc, thread, time::Duration};

use rust_key_value_db::utils;

fn main() {
    let (cmd_channel_sender, cmd_channel_receiver) = mpsc::channel::<String>();
    let (data_channel_sender, data_channel_receiver) = mpsc::channel();
    
    thread::spawn(move || {
        server::server_main(cmd_channel_receiver, data_channel_sender);
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

        let cmd = utils::input::get_line("Command: ");
        if cmd.is_empty() {
            continue; // Skip if no input is provided
        }

        // Send the input to the command channel
        let _ = cmd_channel_sender.send(cmd.clone());

        // Exit condition
        if cmd == "cmd:exit" {
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }

    println!("Program terminated.");
}
