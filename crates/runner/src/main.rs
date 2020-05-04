use engine::Engine;

fn main() {
    println!("Hello, from runner!");

    match Engine.run() {
        Err(err) => eprintln!("{}", err),
        Ok(()) => println!("success"),
    };
}
