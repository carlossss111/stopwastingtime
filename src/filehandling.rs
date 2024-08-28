use std::{fs, io::{self, BufRead, Write}, os::unix::fs::FileExt};

const BLACKLIST_PATH : &str = "./blacklist.txt";

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
