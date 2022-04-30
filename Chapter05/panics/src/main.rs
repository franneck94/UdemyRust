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
