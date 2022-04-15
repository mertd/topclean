use clap::Parser;
use dirs;
use serde_derive::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::process::{Command, Output};

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
    apps: Vec<App>,
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
    fn clean(&self) -> Output {
        let mut command: Command;
        if cfg!(target_os = "windows") {
            command = Command::new("cmd");
            command.arg("/c");
        } else {
            command = Command::new("sh");
            command.arg("-c");
        }
        command.arg(format!("{} {}", &self.cmd, &self.args.join(" ")));
        let output = command
            .output()
            .expect(&[&self.name, "cleaning failed"].join(" "));
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        return output;
    }
}

fn run(run_interactive: bool) -> bool {
    println!("{} Starting!", PREFIX);

    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("topclean.toml");
    config_path.push("config.toml");

    let config: Config = toml::from_str(&fs::read_to_string(config_path).unwrap()).unwrap();
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
