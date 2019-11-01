use std::env;

/// Structure to contain all the initial configurations
pub struct InitConfig {
    pub debug: bool,
}

impl InitConfig {
    /// Returns a new InitConfig. The debug value defaults to false.
    fn new() -> Self {
        Self { debug: false }
    }
}

/// Returns a newly created init_config after parsing command line inputs
///
/// The following arguments are valid arguments:
/// 1. -d or --debug: Use a constant known seed
pub fn handle_init_input() -> InitConfig {
    let mut config = InitConfig::new();
    let args: Vec<String> = env::args().collect();

    // Argument parsing
    // cargo run -- *arguments go here*

    // Arguments:
    //      -d | --debug: Use a constant known seed
    match args.len() {
        len if len > 1 => {
            for i in 1..args.len() {
                match &args[i] {
                    string
                        if *string == String::from("-d") || *string == String::from("--debug") =>
                    {
                        config.debug = true;
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }
    config
}
