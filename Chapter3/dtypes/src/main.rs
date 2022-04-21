fn main() {
    let _x1: i8 = 0;
    let _x2: i16 = 0;
    let _x3: i32 = 0;
    let _x4: i64 = 0;
    let _x5: i128 = i128::MIN;

    let _y1: u8 = u8::MAX;
    let _y2: u16 = u16::MAX;
    let _y3: u32 = u32::MAX;
    let _y4: u64 = u64::MAX;
    let _y5: u128 = u128::MAX;

    let _dec = 255;
    println!("_dec: {}", _dec);
    let _hex = 0xff;
    println!("_hex: {}", _hex);
    let _bin = 0b11111111;
    println!("_bin: {}", _bin);

    let _f1 = 2.0;
    let _f2: f64 = 2.0;
    let _f3: f32 = 2.0;
    let _f4 = 2.0_f32;
    let _f5 = 2.0_f64;
}
