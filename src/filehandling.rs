use std::collections::{HashSet, LinkedList};
use std::fs::{self};
use std::io::{self, BufRead, Write};

const HOSTFILE_PATH : &str = "/etc/hosts";
const BLACKLIST_PATH : &str = "/usr/local/etc/stopwastingtime.txt";

// List a blacklist, or create a new empty one if there is none
pub fn list_blacklist() {
    let content = match fs::read_to_string(BLACKLIST_PATH) {
        Ok(response) => response,
        Err(_) => {
            fs::File::create_new(BLACKLIST_PATH)
                .expect("Cannot create blacklist file.");
            String::from("(There is no blacklist, a new empty file has been created.)\n")
        }
    };

    if content.is_empty() {
        println!("(The blacklist is empty)");
    }
    else {
        print!("{content}");
    }
}

// Delete the blacklist
pub fn delete_blacklist() {
    let mut buff = String::new();
    println!("Are you sure you would like to delete {BLACKLIST_PATH}? [Y/n]");
    std::io::stdin().read_line(&mut buff).expect("Could not read stdin.");
    match buff.to_lowercase().trim() {
        "y" => {
            fs::remove_file(BLACKLIST_PATH).expect("Could not delete file");
            println!("File deleted.");
        }
        _ => println!("Aborted."),
    }
}

// Add a line to the blacklist, or create if empty
pub fn add_to_blacklist(domain : &String) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(BLACKLIST_PATH)
        .expect("Unable to open file.");

    file.write_fmt(format_args!("{domain}\n"))
        .expect("Unable to write to file.");
    println!("\"{domain}\" added to the blacklist.");
}

// Read every line of the blacklist except matches, rewrite it back
pub fn remove_from_blacklist(domain: &String) {
    let mut entire_text = String::new();

    {
        let file = fs::OpenOptions::new()
            .read(true)
            .open(BLACKLIST_PATH)
            .expect("Unable to open file.");

        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("Couldn't read line."); 
            if !line.contains(domain) {
                entire_text.push_str((line + "\n").as_str());
            }
        }
    }
        
    {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(BLACKLIST_PATH)
            .expect("Unable to reopen file.");

        file.write(entire_text.as_bytes())
            .expect("Failed to write back to file.");
    }

    println!("{domain} was removed from the blacklist.");
}

// Append all lines from the blacklist onto the end of the hostfile
pub fn turn_on() {
    let mut blacklist_str = String::new();

    {
        let blacklist_file = fs::OpenOptions::new()
            .read(true)
            .open(BLACKLIST_PATH)
            .expect("Unable to open blacklist.");
        let reader = io::BufReader::new(blacklist_file);
        
        for line in reader.lines() {
            let line : String = line.expect("Couldn't read line."); 
            blacklist_str.push_str(format!("127.0.0.1 {line}\n").as_str());
        }
    }

    {
        let mut host_file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(HOSTFILE_PATH)
            .expect("Unable to open hostfile.");

        host_file.write(blacklist_str.as_bytes())
            .expect("Failed to write to hostfile.");
    }
}

// Read every entry in the hostfile except those in the blacklist, then rewrite
pub fn turn_off() {
    let mut blacklist_map = HashSet::new(); 
    let mut hostfile_list : LinkedList<String> = LinkedList::new();

    // Move blacklist to set
    {
        let blacklist_file = fs::OpenOptions::new()
            .read(true)
            .open(BLACKLIST_PATH)
            .expect("Unable to open blacklist.");
        let reader = io::BufReader::new(blacklist_file);

        for line in reader.lines() {
            let line : String = line.expect("Couldn't read line."); 
            blacklist_map.insert(format!("127.0.0.1 {line}"));
        }
    }

    // Move hostfile to list
    {
        let host_file = fs::OpenOptions::new()
            .read(true)
            .open(HOSTFILE_PATH)
            .expect("Unable to open hostfile.");
        let reader = io::BufReader::new(host_file);

        for line in reader.lines() {
            let host_line : String = line.expect("Couldn't read line.");
            if !blacklist_map.contains(&host_line){
                hostfile_list.push_front(host_line);
            }
        }

    }

    // Write back to hostfile
    {
        let host_file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(HOSTFILE_PATH)
            .expect("Unable to open hostfile.");
        let mut writer = io::BufWriter::new(host_file);

        for domain in hostfile_list {

           writer.write_fmt(format_args!("{domain}\n"))
               .expect("Failed to write to file.");
        }
    }
}

