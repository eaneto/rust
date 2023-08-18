fn main() {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let sign_bit = n_bits >> 31;
    let exponent = ((n_bits >> 23 & 0xff) as i32) - 127;

    // 1.0 (2^-0)
    let mut mantissa: f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let weight = 2_f32.powf(i as f32 - 23.0);
            mantissa += weight;
        }
    }

    println!("{n_bits}");
    println!("{sign_bit}");
    println!("{exponent}");
    println!("{mantissa}")
}
