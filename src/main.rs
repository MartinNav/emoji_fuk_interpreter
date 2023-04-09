use std::env;
mod parser;
mod interpreter;

fn main(){
    let arg1 = env::args().nth(1);

    let mut code:String= String::new();
    match arg1{
        Some(a)=>{
           match a.as_str() {
               "-dir" | "--dir" => {
                    match env::args().nth(2) {
                        Some(directory) => {
                            println!("Reading data from directory: {}",directory);
                            code = std::fs::read_to_string(directory)
                            .expect("You have provided invalid directory\nShutting down");
                        },
                        None => {
                            println!("Argument was not provided\nShutting down...");
                            return;
                        },
                    }
                },
               "-h" | "--help" => {
                    println!("Manual:");
                    println!("--help\t-h\tto get help");
                    println!("--dir\t-dir\tYou have to specify directory");
                    println!("\t\t-dir [directory]");
                    return;
                },
                _=>println!("Error:Invalid parameter\nTry:\n\t--help to get help"),
           } 

        },
        _=>{
            println!("Gimmie the code:");
            std::io::stdin().read_line(&mut code).expect("No code provided");
            }
    }

    let parsed = parser::parse(code);
    interpreter::run(parsed);
    
    
}
