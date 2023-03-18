use std::env;
use mini_grep::Config;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    

    println!("Searching {} in {}",config.query,config.file);
    // run(config);
    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application Error{}",e);
        process::exit(1);
    }
}
