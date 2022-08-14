use core::time;
use std::thread;
use dialoguer::Input;

enum CommandName {
    Exit,
    Access,
    InvalidCommand,
}

enum CommandFlag {
    NoFlag,
    AuthAsSuperUser(String),
}

fn build_command(input: String) -> (CommandName, CommandFlag) {

    let mut default_command_name = "help";
    let mut default_command_flag = "";
    let mut default_input = "";

    let trimmed_input = input.trim();
    let split_input = trimmed_input.split(" ");
    let as_vec = split_input.collect::<Vec<&str>>();

    if as_vec.len() >= 3 {
        default_command_name = as_vec[0];
        default_command_flag = as_vec[1];
        default_input = as_vec[2];
    } else if as_vec.len() == 2 {
        default_command_name = as_vec[0];
        default_command_flag = as_vec[1];
    } else if as_vec.len() == 1 {
        default_command_name = as_vec[0];
    }
    let command_name: CommandName;
    let command_flag: CommandFlag;
    match default_command_name {
        "exit" => command_name = CommandName::Exit,
        "access" => command_name = CommandName::Access,
        _ => command_name = CommandName::InvalidCommand
    }

    match default_command_flag {
        "-su" => command_flag = CommandFlag::AuthAsSuperUser(String::from(default_input)),
        "security" => command_flag = CommandFlag::AuthAsSuperUser(String::from(default_input)),
        "main" => command_flag = CommandFlag::AuthAsSuperUser(String::from(default_input)),
        _ => command_flag = CommandFlag::NoFlag
    }

    (command_name, command_flag)
    
} 
struct Commander {
    _is_authorized: bool,
    _is_admin_authorized: bool,
    _user: String,
    login_attempts: u8,
}

impl Commander { 
    fn new() -> Commander {
        Commander {
            _is_authorized: false,
            _is_admin_authorized: false,
            _user: String::from(""),
            login_attempts: 0,
        }
    }

    fn authorize(&mut self, code: String) { 
        if code != "please" {
            thread::sleep(time::Duration::from_millis(500));
            self.login_attempts = self.login_attempts + 1;
            if self.login_attempts < 3 {
                println!("access: PERMISSION DENIED");
            } else {
                println!("access: PERMISSION DENIED....and...");
                thread::sleep(time::Duration::from_millis(500));
                loop {
                    thread::sleep(time::Duration::from_millis(25));
                    println!("YOU DIDN'T SAY THE MAGIC WORD!");
                }
            }
        }
    }

    fn start_system(&mut self) {
        println!("Jurassic Park, System Security Interface");
        println!("Version 4.0.5, Alpha E");
        println!("Ready...");
        loop { 
            let command: String = Input::new().with_prompt(">").interact().expect("Got a weird command.");
            self.handle_command(command);
        }
    }

    fn handle_command(&mut self, input: String) {
        // need to complicate this with d
        let command = build_command(input);
        // command || flag  - > might make sense to use enum
        match command {
            (CommandName::Exit, CommandFlag::NoFlag) => {
                std::process::exit(0);
            },
            (CommandName::Access, CommandFlag::AuthAsSuperUser(code)) => {
                self.authorize(code);
            },
            _ => println!("Unknown command."),
        }
    }
}

fn main() {

    let mut commander = Commander::new();
    commander.start_system();

    // here shall lie the preparedness functions such as 
    // - electrical systems can be turned on 
    // = water systems can be turned on
    // - ventilation systems can be turned on
    // - security systems can be turned on 
    // - door locks can be turned on 
    // - databases are available for access
    // - phone systems can be turned on 

    // at this point, open shell should be running
    // which can take commands. 

    // Some commands that we know of:
        // access - to get access to a particular system in the list mentioned above
        // exit - to exit the shell
        // help - to navigate the shell
        // shutdown - to shutdown the system
        // reboot - to reboot the system
        // run -s <system> - to run a system
        // run -s <system> -c <command> - to run a command on a system
        // open -s <system> - to access a specific sub-program 
        // exit -s <system> - to exit a specific sub-program
        // help -s <system> - to get help on a specific sub-program
        // shutdown -s <system> - to shutdown a specific sub-program
    
        // emergency - to activate emergency protocols
        // emergency -s <system> - to activate emergency protocols on a specific system
}

