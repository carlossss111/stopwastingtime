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
        .get_matches();
    
    // Positional ON/OFF arg
    if let Some(state) = args_in.get_one::<String>("state") {
        match state.as_str() {
            "on" => println!("ON"),
            "off" => println!("OFF"),
            _ => unreachable!(),
        }
    }

    // Options
    if args_in.get_flag("list") {}
    if let Some(_domain) = args_in.get_one::<String>("add") {}
    if let Some(_domain) = args_in.get_one::<String>("remove") {}
    if let Some(_timespan) = args_in.get_one::<u32>("timer") {}
}
