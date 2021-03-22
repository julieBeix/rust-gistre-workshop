fn getstring(s : Option<String>) -> String{
    match s{
        Some(s) => s,
        None => String::from(""),
    }
}
pub fn cat(path: &str) -> String {
    let buffer = std::fs::read_to_string(path).expect("Something went wrong reading the file");
    return buffer;
}

fn main() {
    let mut args = std::env::args();
    let s = getstring(args.nth(1));
    let text = cat(&s);

    print!("{}", text);
}
