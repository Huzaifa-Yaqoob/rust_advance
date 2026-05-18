use std::env;
use mini_grep::Config;

fn main() {
    let args = env::args();

    let conf = match Config::new(args) {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("Problem parsing arguments: {}", e);
            std::process::exit(1);
        }
    };

  if let Err(e) =  mini_grep::run(conf) {
      println!("Application error: {}", e);
      std::process::exit(1);
  };

}


