


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
        App {name: "scoop".to_string(), cmd: "scoop".to_string(), args: vec!["cleanup".to_string(), "*".to_string()]},
        App {name: "npm".to_string(), cmd: "npm".to_string(), args: vec!["cache".to_string(), "clean".to_string(), "--force".to_string()]},
        App {name: "yarn".to_string(), cmd: "yarn".to_string(), args: vec!["cache".to_string(), "clean".to_string()]},
        App {name: "cleanmgr".to_string(), cmd: "cleanmgr".to_string(), args: vec!["/d".to_string(), "c".to_string(), "/verylowdisk".to_string()]},
        App {name: "brew".to_string(), cmd: "brew".to_string(), args: vec!["cleanup".to_string()]},
    ];
    // done
    println!("{} Done!", prefix);
}

