use std::{collections::HashMap, sync::mpsc};

pub fn server_main(
    cmd_channel_receiver: mpsc::Receiver<String>,
    data_channel_sender: mpsc::Sender<String>,
) {
    let mut values: HashMap<String, String> = HashMap::new();

    let _ = data_channel_sender.send("server:started".to_string());

    for received_raw in cmd_channel_receiver {
        let received = received_raw.as_str();
        let args: Vec<&str> = received.split_whitespace().collect();
        let command = args[0];

        match command {
            "cmd:add" => {
                if args.len() >= 3 {
                    let key = args[1].to_string();
                    let value = args[2].to_string();
    
                    values.insert(key, value);
                    let _ = data_channel_sender.send("Value added!".to_string());
                } else {
                    let _ = data_channel_sender.send(
                        "Invalid cmd:add format. Use: cmd:add <key> <value>".to_string(),
                    );
                }
            },
            "cmd:print" => {
                print!("Values: ");
                for (k, v) in &values {
                    println!("{k}: {v}");
                }
                print!("");
            },
            "cmd:ping" => {
                let _ = data_channel_sender.send("server:pong".to_string());
            },
            "cmd:exit" => break,
            _ => {
                let _ = data_channel_sender.send(format!("Unknown command: {}", received));
            },
            
        }
    }

    let _ = data_channel_sender.send("Server stopped!".to_string());
}
