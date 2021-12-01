use std::env;
extern crate minigrep;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    match config {
        Ok(config) => {
            let result = minigrep::search(&config);
            println!("{:?}", result)
        }
        Err(e) => eprintln!("Error encountered: {}", e),
    }
}
