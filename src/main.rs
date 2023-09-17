mod cli;
mod sql;
mod storage;

fn main() -> anyhow::Result<()> {
    println!("Welcome to SQLRite");
    cli::cli()
}
