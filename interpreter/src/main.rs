mod lexer;
mod repl;
mod token;

fn main() {
    println!("Hello! This is the Monkey programming language!\n");
    println!("Feel free to type in commands\n");
    repl::start();
}
