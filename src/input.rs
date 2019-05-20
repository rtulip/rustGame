use std::env;

/// init_config
/// 
/// Structure to contain all the initial configurations
pub struct init_config {
    pub debug: bool,
}

impl init_config{

    fn new() -> Self {
        Self {debug: false}
    }

}

/// handle_init_input()
/// 
/// args:
///     None
/// 
/// returns: A newly created init_config after parsing command line inputs
/// 
pub fn handle_init_input() -> init_config {
        let mut config = init_config::new();
        let args: Vec<String> = env::args().collect();

        // Argument parsing
        // cargo run -- *arguments go here*

        // Arguments: 
        //      -d | --debug: Use a constant known seed
        match args.len(){
            len if len > 1 => {
                for i in 1..args.len(){
                    match &args[i]{
                        string if *string == String::from("-d") || *string == String::from("--debug") => {
                            config.debug = true;
                        },
                        _ => (),
                    }
                }
            },
            _ => (),

        }
        config
    }
