// use std::env
use std::error::Error;
use std::{env, fs};

 pub fn run(config:Config) ->Result<(),Box<dyn Error>>{
 
    let contents=fs::read_to_string(config.filename)?;
let results = if config.case_sensitive {
  search(&config.query, &contents)
} else {
  case_insensitive(&config.query, &contents)
};
   for line in results{
    println!("{}",line);
   }
    Ok(())
} 
     
     
     pub struct Config { 
        pub query:String,
        pub filename:String,
        pub case_sensitive:bool
    }

      impl Config{
    pub fn new(args:&[String])->Result<Config,&str>{
    if args.len()<3{
       
        return Err("not enough arguments");
    }
    
        let query=args[1].clone();  
        let filename=args[2].clone();
        let case_sensitive=env::var("CASE_INSENSITIVE").is_err();
       return Ok(Config {query,filename,case_sensitive});
    }
    }


    pub fn case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}



pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

    #[cfg(test)]
    mod tests {
      use super::*;

      #[test]
      fn one_result() {
        let query: &str="duct";
        let contents:&str="\
        rust:
        eating,unclockable,parts.
        Pich three.
        Trust me.
        Duct tape.";
          assert_eq!(vec!["eating,unclockable,parts"],search(query,contents));

      }
      
      #[test]
      fn case_insensitive() {
        let query: &str="duct";
        let contents:&str="\
        rust:
        eating,unclockable,parts.
        Pich three";
          assert_eq!(vec!["eating,unclockable,parts"],search(query,contents));

      }

    }
  
    

     

