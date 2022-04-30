fn main() {
    let decimal_flt = 265.4321_f32;

    // let decimal_int: u8 = decimal_flt;
    let decimal_int1 = decimal_flt as u8;
    println!("Casting: {} -> {}", decimal_flt, decimal_int1);

    let decimal_int1 = decimal_flt as u16;
    println!("Casting: {} -> {}", decimal_flt, decimal_int1);

    let value = 1337;
    let decimal_int1 = value as i8;
    println!("Casting: {} -> {}", value, decimal_int1);

    let decimal_int1 = value as i16;
    println!("Casting: {} -> {}", value, decimal_int1);
}
