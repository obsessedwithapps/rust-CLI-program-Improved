use std::fs;
use std::error::Error;
use std::env;

//All of our program's logic will be in here for us to use

#[cfg(test)] //Tests will happen beneath this line
mod tests { 
    use super::*;

    #[test]
    fn case_sensitive() { //We'll use this to test our search function to make sure that it works

        let query = "duct";
        let file_contents = "\
Rust: 
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, file_contents));

    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let file_contents = "\
Rust:
Systems programming language.
It's true.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, file_contents))
    }

}

pub fn search<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {

    //We updated the search function to use iterator adaptors
    //This takes and filters every line that contains the query and collects it

    file_contents.lines()
        .filter(|line| line.contains(query))
        .collect()

} 

pub fn search_case_insensitive<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    
    for line in file_contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line); 
        }
    }

    results //Returns the results
} 

pub struct Config { //This is our config struct

    pub query: String, //This is the query field which holds a String
    pub filename: String, //This is the filename field which holds a String as well
    pub case_sensitive: bool, //This makes it either case sensitive or insensitive

}

impl Config { //This makes implementations to our Config struct

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> { //We changed the name of our parse_config function to new and updated the parameters

        //The std library documentation shows that the type of the iterator that args returns is std::env::Args. We updated the function to instead take std::env::Args
        //instead of &[String]

        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("A query string was not able to be retrieved")
        };

        let filename = match args.next() {

            //args.next() is needed since the filename of the input is the second value while the first one is the query. We want to ignore the first and go to the next.

            Some(arg) => arg,
            None => return Err("No filename was provided")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config {query, filename, case_sensitive}) //The return type is Result, we used Ok(()) here.
    
    }

}

pub fn read(config: Config) -> Result<(), Box<dyn Error>> { //This the read function that reads the contents

    let file_contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive { //If its case sensitive
        
        search(&config.query, &file_contents) //Search for whatever to 

    } else {

        search_case_insensitive(&config.query, &file_contents)

    };

    for line in results { //A for loop here is made 
        println!("{}", line); //Prints the line that it finds when we use the search function for the specific thing that we're looking for
    }

    Ok(())

}