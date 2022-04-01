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
    /** Skip during test */
    skip: bool,
}

impl App {
    fn clean(&self) -> bool {
        if self.skip {
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

fn run() -> bool {
    println!("{} Starting!", PREFIX);
    let apps = [
        App {
            name: s("scoop"),
            cmd: s("scoop"),
            args: vec![s("cleanup"), s("*")],
            skip: false,
        },
        App {
            name: s("npm"),
            cmd: s("npm"),
            args: vec![s("cache"), s("clean"), s("--force")],
            skip: false,
        },
        App {
            name: s("yarn"),
            cmd: s("yarn"),
            args: vec![s("cache"), s("clean")],
            skip: false,
        },
        App {
            name: s("cleanmgr"),
            cmd: s("cleanmgr"),
            args: vec![s("/d"), s("c"), s("/verylowdisk")],
            skip: true,
        },
        App {
            name: s("brew"),
            cmd: s("brew"),
            args: vec![s("cleanup")],
            skip: false,
        },
    ];
    for app in apps {
        app.clean();
    }
    println!("{} Done!", PREFIX);
    return true;
}

fn main() {
    run();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn runs() {
        let result = run();
        assert_eq!(result, true);
    }
}
