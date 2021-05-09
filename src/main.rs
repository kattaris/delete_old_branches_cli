use std::io;
use std::io::{BufRead, Write};

use git2::BranchType;

use errors::term_errors::Error;
use handlers::git;

mod errors;
mod handlers;

fn main() -> Result<(), Error> {
    crossterm::terminal::enable_raw_mode()?;

    let input = io::stdin();
    let output = io::stdout();

    loop {
        let mut handle_out = output.lock();
        write!(handle_out, "Type a command >")?;
        handle_out.flush()?;

        let line = input.lock().lines().next().unwrap()?;
        match line.as_str() {
            "quit" | "q" => {
                break;
            }
            "local" | "l" => {
                let branches = git::handle_branches(
                    BranchType::Local);
                for (item, time) in branches {
                    writeln!(handle_out, "{} - Last commit: {}", item, time)?;
                }
            }
            "remote" | "r" => {
                let branches = git::handle_branches(
                    BranchType::Remote);
                for (item, time) in branches {
                    writeln!(handle_out, "{} - Last commit: {}", item, time)?;
                }
            }
            _ => {
                write!(handle_out, "{}\n", "Unknown command")?;
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

