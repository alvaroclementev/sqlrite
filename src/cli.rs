use std::io::{self, Write};

use crate::sql;

enum CliCommand {
    Exit,
}

pub fn cli() -> anyhow::Result<()> {
    loop {
        print!("db> ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        if buffer.is_empty() {
            // The user issued a <C-D>
            break;
        }

        // Process the input
        let trimmed = buffer.trim();
        if trimmed.starts_with('.') {
            // Try to parse a command
            if let Some(command) = parse_command(trimmed) {
                // Run the command
                match command {
                    CliCommand::Exit => break,
                }
            } else {
                println!("Unrecognized command: '{}'", trimmed);
            }
            continue;
        }

        // Interpret as SQL
        match sql::Statement::prepare(trimmed).and_then(|stmt| stmt.execute()) {
            Ok(_) => (),
            Err(msg) => println!("error: {}", msg),
        }
    }
    Ok(())
}

fn parse_command(input: &str) -> Option<CliCommand> {
    match input {
        ".exit" => Some(CliCommand::Exit),
        _ => None,
    }
}
