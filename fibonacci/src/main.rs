use std::io;

fn main() {
    println!("Compute fibonacci nth number");
    let number = get_number();
    let nth = fibo(number);
    display_fibo(number, &nth);
}

fn display_fibo(n: u32, x: &[u128; 4]) {
    println!("The {}th fibonacci number is {}", n, x[1])
}

fn get_number() -> u32 {
    let mut number_str = String::new();
    io::stdin()
        .read_line(&mut number_str)
        .expect("Failed to read line");

    number_str
        .trim()
        .parse()
        .expect("N entered was not a number")
}

const MAT2_ID: [u128; 4] = [1, 0, 1, 0];
const MAT2_FIB_INIT: [u128; 4] = [1, 1, 1, 0];

fn mat2_mul(x: &[u128; 4], y: &[u128; 4]) -> [u128; 4] {
    let i00 = x[0] * y[0] + x[1] * y[2];
    let i01 = x[0] * y[1] + x[1] * y[3];
    let i10 = x[2] * y[0] + x[3] * y[2];
    let i11 = x[2] * y[1] + x[3] * y[3];
    let ret = [i00, i01, i10, i11];
    ret
}

// [1 1 1 0]^n = [ F_{n+1} F_n F_n F_{n-1} ]
fn mat2_pow_n(x: &[u128; 4], n: u32) -> [u128; 4] {
    if n == 0 {
        MAT2_ID
    } else if n == 1 {
        MAT2_FIB_INIT
    } else if n % 2 == 0 {
        let sqrt = mat2_pow_n(x, n / 2);
        mat2_mul(&sqrt, &sqrt)
    } else {
        let sqrt = mat2_pow_n(x, n / 2);
        let square = mat2_mul(&sqrt, &sqrt);
        mat2_mul(&square, x)
    }
}

fn fibo(n: u32) -> [u128; 4] {
    mat2_pow_n(&MAT2_FIB_INIT, n)
}
