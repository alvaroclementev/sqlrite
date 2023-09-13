use std::io;

mod cli;

fn main() -> io::Result<()> {
    println!("Welcome to SQLRite");
    cli::cli()
}
