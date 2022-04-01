use std::io::{self, Write};
use std::process::{Command, Output};

const PREFIX: &str = "[topclean]";

/**
 * Shorthand to convert String
 */
fn s(input: &str) -> String {
    return String::from(input);
}

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
    let apps = [
        App {
            name: s("scoop"),
            cmd: s("scoop"),
            args: vec![s("cleanup"), s("*")],
            interactive: false,
        },
        App {
            name: s("npm"),
            cmd: s("npm"),
            args: vec![s("cache"), s("clean"), s("--force")],
            interactive: false,
        },
        App {
            name: s("yarn"),
            cmd: s("yarn"),
            args: vec![s("cache"), s("clean")],
            interactive: false,
        },
        App {
            name: s("cleanmgr"),
            cmd: s("cleanmgr"),
            args: vec![s("/d"), s("c"), s("/verylowdisk")],
            interactive: true,
        },
        App {
            name: s("brew"),
            cmd: s("brew"),
            args: vec![s("cleanup")],
            interactive: false,
        },
    ];
    for app in apps {
        if !run_interactive && app.interactive {
            println!("{} Skipping {}", PREFIX, app.name);
            return false;
        } else {
            println!("{} Cleaning {}", PREFIX, app.name);
            app.clean();
        }
    }
    println!("{} Done!", PREFIX);
    return true;
}

fn main() {
    run(true);
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
