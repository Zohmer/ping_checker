use std::io;
use std::process::Command;

fn main() {
    let mut online = true;
    let mut offline = true;

    println!("Enter target: ");

    let mut target: String = String::new();

    io::stdin().read_line(&mut target).expect("Failed to read line");

    loop {
        if ping_check (&target) {
            if offline {
                println! ("target is online");
                offline = false;
                online = true;
            }
        } else if online {
            println! ("target is offline");
            offline = true;
            online = false;
        }
    }
}

fn ping_check (target: &String) -> bool {
    let ping_result = ping(target);

    if string_search ("from", &ping_result) || 
    string_search ("From", &ping_result) ||
    string_search ("FROM", &ping_result) {
        return true
    } else {
        return false
    }
}

fn ping (target: &String) -> String {
    let ping = if cfg! (target_os = "windows") {
        let command = String::from("ping /n 1 ") + target;
        Command::new ("cmd")
                .args (&["/C", &command[..]])
                .output()
                .expect("ping command failed to start")
    } else {
        let command = String::from("ping -c 1 ") + target;
        Command::new ("sh")
                .arg ("-c")
                .arg (command)
                .output()
                .expect("ping command failed to start")
    };
    
    let ping_result = ping.stdout;
    
    String::from_utf8 (ping_result).expect("Failed to convert")
}

fn string_search (word: &str, string: &String) -> bool {
    let word = word.to_string();

    let split: Vec<&str> = string.split(&word).collect();
    split.len() > 1
}