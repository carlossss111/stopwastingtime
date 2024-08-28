pub mod filehandling;

use clap::{arg, Command};

fn main() {
    // Arg handling
    let args_in = Command::new("stopwastingtime")
        .display_name("Stop Wasting Time!")
        .version("0.1")
        .about("Blacklists domains with the hostfile")
        .arg(arg!(["state"] "Desired state of the program").value_parser(["on", "off"]))
        .arg(arg!(-'l' --"list" "Lists blacklisted domains"))
        .arg(arg!(-'a' --"add" <"domain"> "Add domain to blacklist"))
        .arg(arg!(-'r' --"remove" <"domain"> "Remove domain from blacklist"))
        .arg(arg!(-'t' --"timer" <"minutes"> "Change state for n minutes"))
        .arg(arg!(-'D' --"delete" "Delete the blacklist and it's contents"))
        .get_matches();
    
    // Positional ON/OFF arg
    if let Some(state) = args_in.get_one::<String>("state") {
        match state.as_str() {
            "on" => println!("ON"),
            "off" => println!("OFF"),
            _ => unreachable!(), // covered by value_parser()
        }
    }

    // Options
    if args_in.get_flag("list") {
        filehandling::list_blacklist();
    }
    if args_in.get_flag("delete") {
        filehandling::delete_blacklist();
    }
    if let Some(domain) = args_in.get_one::<String>("add") {
        filehandling::add_to_blacklist(domain);
    }
    if let Some(domain) = args_in.get_one::<String>("remove") {
        filehandling::remove_from_blacklist(domain); 
    }
    if let Some(_timespan) = args_in.get_one::<u32>("timer") {

    }
}
