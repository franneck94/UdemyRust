// Arrays: Fixed length, only same dtypes

fn main() {
    let _array = [1, 2, 3];

    println!("{}", _array[0]);
    println!("{}", _array[1]);
    println!("{}", _array[2]);

    let _array2: [i32; 3] = [4, 5, 6];

    println!("{}", _array2[0]);
    println!("{}", _array2[1]);
    println!("{}", _array2[2]);

    let _array3: [i32; 3] = [1337; 3];

    println!("{}", _array3[0]);
    println!("{}", _array3[1]);
    println!("{}", _array3[2]);

    println!("{:?}", _array);
    // println!("{}", _array[5]);
}
