use std::io;

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
    //TODO: Send ping request to target and return resulting string

    //Temporary compile code
    let temp = String::new();
    temp
}

fn string_search (word: &str, string: &String) -> bool {
    //TODO: Search string for inputted word

    //Temporary compile code
    true
}