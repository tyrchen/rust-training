fn main() {
    if atty::is(atty::Stream::Stdout) {
        println!("stdout is a tty");
    } else {
        println!("stdout is not a tty");
    }
}
