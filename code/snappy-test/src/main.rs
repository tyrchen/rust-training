extern crate libc;
use libc::size_t;

#[link(name = "snappy")]
extern "C" {
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}

fn main() {
    let len = 10000;
    let result = unsafe { snappy_max_compressed_length(len) };
    println!("max compressed length for {} byte buffer: {}", len, result);
}
