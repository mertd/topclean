
/**
 * Shorthand to convert String
 */
fn s(input: &str) -> String {
    return String::from(input);
}

fn main() {
    let prefix =  "[topclean]";
    println!("{} Starting!", prefix);
    // do something
    struct App {
        name: String,
        cmd: String,
        args: Vec<String>,
    }
    let apps = [
        App {name: s("scoop"), cmd: s("scoop"), args: vec![s("cleanup"), s("*")]},
        App {name: s("npm"), cmd: s("npm"), args: vec![s("cache"), s("clean"), s("--force")]},
        App {name: s("yarn"), cmd: s("yarn"), args: vec![s("cache"), s("clean")]},
        App {name: s("cleanmgr"), cmd: s("cleanmgr"), args: vec![s("/d"), s("c"), s("/verylowdisk")]},
        App {name: s("brew"), cmd: s("brew"), args: vec![s("cleanup")]},
    ];
    // done
    println!("{} Done!", prefix);
}

