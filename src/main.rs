use std::process::{Command, Output};
use std::io::{self, Write};

const PREFIX: &str = "[topclean]";

/**
 * Shorthand to convert String
 */
fn s(input: &str) -> String {
    return String::from(input);
}

struct App {
    name: String,
    cmd: String,
    args: Vec<String>,
}

impl App {
    fn clean(&self) -> Output {
        println!("{} Cleaning {}", PREFIX, self.name);
        let shell: &str;
        let arg: &str;
        if cfg!(target_os = "windows") {
            shell = "cmd";
            arg = "/C";
        } else {
            shell = "sh";
            arg = "-c";
        }
        let output = Command::new(shell)
            .args([arg, &self.cmd])
            .args(&self.args)
            .output()
            .expect(&[&self.name, "cleaning failed"].join(" "));
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        return output;
    }
}

fn main() {
    println!("{} Starting!", PREFIX);
    let apps = [
        App {name: s("scoop"), cmd: s("scoop"), args: vec![s("cleanup"), s("*")]},
        App {name: s("npm"), cmd: s("npm"), args: vec![s("cache"), s("clean"), s("--force")]},
        App {name: s("yarn"), cmd: s("yarn"), args: vec![s("cache"), s("clean")]},
        App {name: s("cleanmgr"), cmd: s("cleanmgr"), args: vec![s("/d"), s("c"), s("/verylowdisk")]},
        App {name: s("brew"), cmd: s("brew"), args: vec![s("cleanup")]},
    ];
    for app in apps {
        app.clean();
    }
    println!("{} Done!", PREFIX);
}
