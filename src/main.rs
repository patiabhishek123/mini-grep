use std::env;
use std::process;
use Mini_grep::Config;
fn main(){
    let args:Vec<String>=env::args().collect();
    let config=Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problems parsing arguments{}",err);
        process::exit(1);
    });

    println!("Searching for {} ",config.query);
    println!("IN file \n{}",config.filename);
  if let Err(e)=Mini_grep::run(config){
    eprintln!("Application error: {}",e);
    process::exit(1);
  }
}

