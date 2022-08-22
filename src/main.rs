use std::env;
use std::process;

use minigrep::Config; //We get the Config type from minigrep::Config.
                      //minigrep is the name of the project. We made a library file (lib.rs) and its part of our minigrep folder.
                      //In order to use Config, we needed to get it directly from a file in our project (project::Config) in the src folder.

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| { 
        
        //The env::args function returns an iterator. 
        //Rather than collecting the iterator values into a vector and passing a slice to Config::new, we're passing ownership directly from env::args to Config::new directly
        
        eprintln!("There was a problem parsing the arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In {}", config.filename);

    if let Err(e) = minigrep::read(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

}

