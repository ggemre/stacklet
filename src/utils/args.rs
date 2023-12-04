use std::env;

#[derive(Debug)]
pub struct Args {
    help: bool,
    version: bool,
    exec_path: Option<String>,
}

impl Args {
    pub fn help(&self) -> bool {
        self.help
    }

    pub fn version(&self) -> bool {
        self.version
    }

    pub fn exec_path(&self) -> Option<&String> {
        self.exec_path.as_ref()
    }
}

// Function to parse command-line arguments and return an instance of Args
pub fn parse_args() -> Args {
    // Get the command-line arguments without the program name
    let args: Vec<String> = env::args().skip(1).collect();

    // Define default values for optional arguments
    let mut help = false;
    let mut version = false;
    let mut exec_path: Option<String> = None;

    // Parse command-line arguments
    let mut iter = args.iter().peekable();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" | "--help" => help = true,
            "-v" | "--version" => version = true,
            "-x" | "--exec" => {
                // Check if the next argument is available
                if let Some(path) = iter.next() {
                    exec_path = Some(path.clone());
                } else {
                    println!("Error: Missing argument for -x/--exec");
                    print_help(); // Assuming a function to print help
                    std::process::exit(1);
                }
            }
            _ => {
                println!("Error: Unknown argument '{}'", arg);
                print_help(); // Assuming a function to print help
                std::process::exit(1);
            }
        }
    }

    // Return an instance of Args with parsed values
    Args {
        help,
        version,
        exec_path,
    }
}

pub fn print_help() {
    println!("Usage:");
    println!("  -h, --help      Print help message and quit");
    println!("  -v, --version   Print program version");
    println!("  -x, --exec      Path to executable (required)");
}

// Example function to print program version
pub fn print_version() {
    println!("Program version: 1.0");
}
