fn main() {
    let decimal_flt = 265.4321_f32;

    // let decimal_int: u32 = decimal_flt;

    let decimal_int1 = decimal_flt as u8;
    println!("Casting {decimal_flt} -> {decimal_int1}");

    let decimal_int2 = decimal_flt as u16;
    println!("Casting {decimal_flt} -> {decimal_int2}");

    let decimal_int3 = decimal_flt as u32;
    println!("Casting {decimal_flt} -> {decimal_int3}");

    let value = 1337;
    let decimal_int4 = value as u8;
    println!("Casting {decimal_flt} -> {decimal_int4}");
}
