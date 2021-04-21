fn main() {
    let value: usize = 0xdeadbeef;
    println!("result: {:?}", process(value));
}

#[no_mangle]
fn process(v: usize) -> usize {
    v % (v >> 16)
}
