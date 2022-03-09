fn main() {
    let prefix = "[topclean]";
    println!("{} Starting!", prefix);
    // do something
    println!("{} Done!", prefix);
}

struct App {
    name: String,
    cmd: String,
    args: [String],
}
