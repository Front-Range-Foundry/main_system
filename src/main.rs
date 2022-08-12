use std::env; 

struct Commander {
    is_authorized: bool,
    is_admin_authorized: bool,
    user: String,
    login_attempts: u16,
}

impl Commander { 
    fn new() -> Commander {
        Commander {
            is_authorized: false,
            is_admin_authorized: false,
            user: String::from(""),
            login_attempts: 2,
        }
    }

    fn authorize(&mut self, arguments: &Vec<String>) { 
        let flag = &arguments[2];
        let code = &arguments[3];
        if flag != "-c" && code != "please" {
            println!("ACCESS_DENIED");
            self.login_attempts = self.login_attempts + 1;
            if (self.login_attempts > 2) {
                while true {
                    println!("AH AH AH! YOU DIDN'T SAY THE MAGIC WORD!");
                }
            }
        }
    }

    fn parse_command(&mut self) {
        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);
        let command = &args[1] as &str;
        println!("{:?}", command);

        match command { 
            "access" => {
                self.authorize(&args);
            }
            "admin" => println!("admin"),
            _ => println!("unknown command"),
        }
    }

    fn start_system(&self) {
        println!("INGN SYSTEMS - MAIN - VERSION 0.0.1");
        println!("ENTER AUTHORIZATION KEY...");
        println!("WELCOME USER. BOOTING UP...");
    }
}

fn main() {

    let mut commander = Commander::new();
    commander.start_system();
    commander.parse_command();

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

