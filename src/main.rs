use std::io::{self, Write};
use std::process::{Command};

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
    fn clean(&self, skip_interactive: bool) -> bool {
        if self.interactive && skip_interactive {
            println!("{} Skipping {}", PREFIX, self.name);
            return false;
        }
        println!("{} Cleaning {}", PREFIX, self.name);
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
        return true;
    }
}

fn run(skip_interactive: bool) -> bool {
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
        app.clean(skip_interactive);
    }
    println!("{} Done!", PREFIX);
    return true;
}

fn main() {
    run(false);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn runs() {
        let result = run(true);
        assert_eq!(result, true);
    }
}
