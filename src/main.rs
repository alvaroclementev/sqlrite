mod cli;
mod sql;

fn main() -> anyhow::Result<()> {
    println!("Welcome to SQLRite");
    cli::cli()
}
