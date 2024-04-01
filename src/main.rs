use std::io::{self, Write};
use std::process::{Command, Output};
use serde_derive::Deserialize;
use clap::Parser;

const PREFIX: &str = "[topclean]";

/// Free up disk space by cleaning caches and temporary files
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Skip apps that require user input to exit even if there were no errors
    #[clap(short, long)]
    interactive: bool,
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
    /** App requires user input to exit even if there were no errors */
    interactive: bool,
}

impl App {
    fn clean(&self) -> Output {
        let mut command: Command;
        // choose appropriate shell
        if cfg!(target_os = "windows") {
            command = Command::new("cmd");
            command.arg("/c");
        } else {
            command = Command::new("sh");
            command.arg("-c");
        }
        // construct command with arguments
        command.arg(format!("{} {}", &self.cmd, &self.args.join(" ")));
        // execute
        let output = command
            .output()
            .expect(&[&self.name, "cleaning failed"].join(" "));
        // print app output
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        return output;
    }
}

fn run(interactive: bool) -> bool {
    println!("{} Starting!", PREFIX);
    let config: Config = toml::from_str(include_str!("config.toml")).unwrap();
    for app in config.apps {
        if !interactive && app.interactive {
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
    if args.interactive {
        println!("{} Including interactive commands", PREFIX);
    }
    run(args.interactive);
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
