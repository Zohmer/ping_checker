use std::io;
use std::process::Command;

fn main() {
    let mut online = false;
    let mut first_run = true;

    println!("Enter target: ");

    let mut target: String = String::new();

    io::stdin().read_line(&mut target).expect("Failed to read line");

    loop {
        if ping_check (&target) {
            if !online {
                println! ("target is online");
                online = true;
            }
        } else if online  || first_run{
            println! ("target is offline");
            online = false;
        }

        first_run = false;
    }
}

fn ping_check (target: &String) -> bool {
    let ping_result = ping(target);

    if string_search ("from", &ping_result) {
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
    let word = word.to_string().to_lowercase();
    let string = string.to_lowercase();

    let split: Vec<&str> = string.split(&word).collect();
    split.len() > 1
}