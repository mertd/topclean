use std::io::{self, Write};
use std::process::{Command, Output};
use serde_derive::{Deserialize};
use clap::Parser;

const PREFIX: &str = "[topclean]";

/// Free up disk space by cleaning caches and temporary files
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Skip commands that require user interaction to exit
    #[clap(short, long)]
    skip_interactive: bool,
}

#[derive(Deserialize)]
struct Config {
    apps: Vec<App>
}

#[derive(Deserialize)]
struct App {
    /** App Name */
    name: String,
    /** Executable */
    cmd: String,
    /** Arguments */
    args: Vec<String>,
    /** Does not exit without user interaction */
    interactive: bool,
}

impl App {
    fn clean(&self) {
        let output = Command::new(&self.cmd).arg(&self.args.join(" ")).output();
        match output {
            Ok(_) => {
                let result = output.unwrap();
                io::stdout().write_all(&result.stdout).unwrap();
                io::stderr().write_all(&result.stderr).unwrap();
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        
    }
}

fn run(run_interactive: bool) -> bool {
    println!("{} Starting!", PREFIX);
    let config: Config = toml::from_str(include_str!("config.toml")).unwrap();
    for app in config.apps {
        if !run_interactive && app.interactive {
            println!("{} Skipping {}", PREFIX, app.name);
        } else {
            println!("{} Cleaning {}", PREFIX, app.name);
            app.clean();
        }
    }
    println!("{} Done!", PREFIX);
    return true;
}

fn main() {
    let args = Args::parse();
    if args.skip_interactive {
        println!("{} Skipping interactive commands", PREFIX);
    }
    run(!args.skip_interactive);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn runs() {
        // Skip interactive commands as they will never exit in a CI pipeline
        let result = run(false);
        assert_eq!(result, true);
    }
}
