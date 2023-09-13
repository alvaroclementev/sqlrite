use std::io::{self, Write};

enum CliCommand {
    Exit,
}

pub fn cli() -> io::Result<()> {
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
        println!("Read: {}", trimmed);

        if trimmed.starts_with('.') {
            // Try to parse a command
            if let Some(command) = parse_command(trimmed) {
                // Run the command
                match command {
                    CliCommand::Exit => break,
                }
            } else {
                println!(
                    "Unrecognized command: {}",
                    trimmed.strip_prefix('.').unwrap()
                );
            }
        } else {
            println!("Unrecognized input: {}", trimmed);
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
