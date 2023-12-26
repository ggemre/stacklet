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

/// Parse command-line arguments and return an instance of Args
pub fn parse_args() -> Args {
    // get cli args without the program name
    let args: Vec<String> = env::args().skip(1).collect();

    // initialize default values for optional arguments
    let mut help = false;
    let mut version = false;
    let mut exec_path: Option<String> = None;

    // iterate through args and match up with known options
    let mut iter = args.iter().peekable();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" | "--help" => help = true,
            "-v" | "--version" => version = true,
            "-x" | "--exec" => {
                // check if the next argument is available
                if let Some(path) = iter.next() {
                    exec_path = Some(path.clone());
                } else {
                    println!("Error: Missing argument for -x/--exec");
                    print_help();
                    std::process::exit(1);
                }
            }
            _ => {
                println!("Error: Unknown argument '{}'", arg);
                print_help();
                std::process::exit(1);
            }
        }
    }

    Args {
        help,
        version,
        exec_path,
    }
}

/// Print the program help message.
pub fn print_help() {
    println!("Usage:");
    println!("  -h, --help      Print help message and quit");
    println!("  -v, --version   Print program version");
    println!("  -x, --exec      Path to executable (required)");
}

/// Print the program version.
pub fn print_version() {
    println!("Program version: 1.0");
}
