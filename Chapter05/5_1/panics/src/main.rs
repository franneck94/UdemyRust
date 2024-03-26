// By default, panics will print a failure message, unwind, clean up the stack, and quit
// RUST_BACKTRACE=1 cargo run

fn main() {
    let arr = [1, 2, 3];

    function(&arr);
}

fn function(arr_slice: &[i32]) {
    if arr_slice.len() < 4 {
        panic!("ERROR: PLS HELP ME!");
    }

    println!("{}", arr_slice[3]);
}
