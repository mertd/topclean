use std::process::Command;

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
    fn clean(&self) {
        println!("{} Cleaning {}", PREFIX, self.name);
        let output = Command::new(&self.cmd)
            .args(&self.args)
            .output()
            .expect(&[&self.name, "cleaning failed"].join(" "));
        println!("status: {}", output.status);
    }
}

fn main() {
    println!("{} Starting!", PREFIX);
    // do something    
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
    // done
    println!("{} Done!", PREFIX);
}
