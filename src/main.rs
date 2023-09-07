extern { fn c_main(); }

fn main() {
    unsafe { c_main(); }
}