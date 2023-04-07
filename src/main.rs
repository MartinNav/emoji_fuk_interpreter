
mod parser;
mod interpreter;

fn main(){
    println!("Gimmie the code:");
    let mut code:String= String::new();
    std::io::stdin().read_line(&mut code).expect("No code provided");

    let parsed = parser::parse(code);
    interpreter::run(parsed);
}
