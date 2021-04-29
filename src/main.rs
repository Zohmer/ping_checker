use std::io;
use std::process::Command;

fn main() {
    let mut online = true;
    let mut offline = true;

    println!("Enter target: ");

    let mut target: String = String::new();

    io::stdin().read_line(&mut target).expect("Failed to read line");

    //loop {
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
    //}
}

fn ping_check (target: &String) -> bool {
    let ping_result = ping(target);

    if string_search ("from", &ping_result) || 
    string_search ("From", &ping_result) ||
    string_search ("FROM", &ping_result) {
        println!("ping_check true");
        return true
    } else {
        return false
    }
}

fn ping (target: &String) -> String {
    let ping_result = if cfg! (target_os = "windows") {
        Command::new ("ping")
                .arg ("/n 1")
                .arg (target)
                .output()
                .expect("ping command failed to start")
    } else {
        Command::new ("ping")
                .arg ("-c 1")
                .arg (target)
                .output()
                .expect("ping command failed to start")
    };
    
    let output = String::from_utf8(ping_result.stdout).expect("Failed to convert");
    println!("ping_result: {}", output);
    output.to_string()
}

fn string_search (word: &str, string: &String) -> bool {
    let word = word.to_string();

    let split: Vec<&str> = string.split(&word).collect();
    println!("split {}", split[0]);
    split.len() > 1
}