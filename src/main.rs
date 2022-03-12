use duct::cmd;

const PREFIX: &str = "[topclean]";

/**
 * Shorthand to convert &str to String
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
        let stdout = cmd!(&self.cmd, &self.args.join(" ")).read();
        println!("{:?}", stdout);
    }
}

fn run() -> bool {
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
